//
//  ContentView.swift
//  test
//
//  Created by Oscar on 2023-08-11.
//

import BitwardenSdk
import SwiftUI

struct ContentView: View {

    @State private var msg: String

    init() {
        let client = Client(settings: nil)

        _msg = State(initialValue: client.echo(msg: "Sdk"))

        let SERVER_URL = "https://localhost:8080/"
        let API_URL = SERVER_URL + "api/"
        let IDENTITY_URL = SERVER_URL + "identity/"

        let EMAIL = "test@bitwarden.com"
        let PASSWORD = "asdfasdfasdf"

        // Disable SSL Cert validation. Don't do this in production
        let ignoreHttpsDelegate = IgnoreHttpsDelegate()
        let http = URLSession(
            configuration: URLSessionConfiguration.default, delegate: ignoreHttpsDelegate,
            delegateQueue: nil)

        Task {

            ////////////////////////////// Get master password hash //////////////////////////////

            struct PreloginRequest: Codable { let email: String }
            struct PreloginResponse: Codable {
                let kdf: UInt32
                let kdfIterations: UInt32
                let kdfMemory: UInt32?
                let kdfParallelism: UInt32?

            }

            let (preloginDataJson, _) = try await http.data(
                for: request(
                    method: "POST", url: IDENTITY_URL + "accounts/prelogin",
                    fn: { r in
                        r.setValue("application/json", forHTTPHeaderField: "Content-Type")
                        r.httpBody = try JSONEncoder().encode(PreloginRequest(email: EMAIL))
                    }))
            let preloginData = try JSONDecoder().decode(
                PreloginResponse.self, from: preloginDataJson)

            let kdf: Kdf
            if preloginData.kdf == 0 {
                kdf = Kdf.pbkdf2(iterations: preloginData.kdfIterations)
            } else {
                kdf = Kdf.argon2id(
                    iterations: preloginData.kdfIterations, memory: preloginData.kdfMemory!,
                    parallelism: preloginData.kdfParallelism!)
            }

            let passwordHash = try await client.auth().hashPassword(
                email: EMAIL, password: PASSWORD, kdfParams: kdf)

            ///////////////////////////// Login /////////////////////////////

            struct LoginResponse: Codable {
                let Key: String
                let PrivateKey: String
                let access_token: String
                let refresh_token: String
            }

            let (loginDataJson, _) = try await http.data(
                for: request(
                    method: "POST", url: IDENTITY_URL + "connect/token",
                    fn: { r in
                        r.setValue(
                            EMAIL.data(using: .utf8)?.base64EncodedString(),
                            forHTTPHeaderField: "Auth-Email")
                        r.setValue(
                            "application/x-www-form-urlencoded", forHTTPHeaderField: "Content-Type")

                        var comp = URLComponents()
                        comp.queryItems = [
                            URLQueryItem(name: "scope", value: "api offline_access"),
                            URLQueryItem(name: "client_id", value: "web"),
                            URLQueryItem(name: "deviceType", value: "12"),
                            URLQueryItem(
                                name: "deviceIdentifier",
                                value: "0745d426-8dab-484a-9816-4959721d77c7"),
                            URLQueryItem(name: "deviceName", value: "edge"),
                            URLQueryItem(name: "grant_type", value: "password"),
                            URLQueryItem(name: "username", value: EMAIL),
                            URLQueryItem(name: "password", value: passwordHash),
                        ]
                        r.httpBody = comp.percentEncodedQuery!
                            .replacingOccurrences(of: "@", with: "%40")
                            .replacingOccurrences(of: "+", with: "%2B")
                            .data(using: .utf8)
                    }))
            let loginData = try JSONDecoder().decode(LoginResponse.self, from: loginDataJson)

            ///////////////////////////// Sync /////////////////////////////

            struct SyncOrganization: Codable {
                let id: String
                let key: String
            }
            struct SyncProfile: Codable {
                let organizations: [SyncOrganization]

            }
            struct SyncFolder: Codable {
                let id: String
                let name: String
                let revisionDate: String
            }
            struct SyncResponse: Codable {
                let profile: SyncProfile
                let folders: [SyncFolder]
            }

            let (syncDataJson, _) = try await http.data(
                for: request(
                    method: "GET", url: API_URL + "sync?excludeDomains=true",
                    fn: { r in
                        r.setValue(
                            "Bearer " + loginData.access_token, forHTTPHeaderField: "Authorization")
                    }))

            let syncData = try JSONDecoder().decode(SyncResponse.self, from: syncDataJson)

            ///////////////////////////// Initialize crypto /////////////////////////////

            try await client.crypto().initializeUserCrypto(
                req: InitUserCryptoRequest(
                    kdfParams: kdf,
                    email: EMAIL,
                    privateKey: loginData.PrivateKey,
                    method: InitUserCryptoMethod.password(
                        password: PASSWORD,
                        userKey: loginData.Key
                    )
                ))
            
            try await client.crypto().initializeOrgCrypto(
                req: InitOrgCryptoRequest(
                    organizationKeys: Dictionary.init(
                        uniqueKeysWithValues: syncData.profile.organizations.map { ($0.id, $0.key) }
                    )
                ))

            ///////////////////////////// Decrypt some folders /////////////////////////////

            let dateFormatter = ISO8601DateFormatter()
            dateFormatter.formatOptions = [.withFractionalSeconds]

            let decryptedFolders = try await client.vault().folders().decryptList(
                folders: syncData.folders.map {
                    Folder(
                        id: $0.id, name: $0.name,
                        revisionDate: dateFormatter.date(from: $0.revisionDate)!)
                })
            print(decryptedFolders)
        }
    }

    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundColor(.accentColor)
            Text("Hello " + msg)
        }
        .padding()
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}

func request(method: String, url: String, fn: (inout URLRequest) throws -> Void) -> URLRequest {
    var req = URLRequest(url: URL(string: url)!)
    req.httpMethod = method
    try! fn(&req)
    return req
}

class IgnoreHttpsDelegate: NSObject {}

extension IgnoreHttpsDelegate: URLSessionDelegate {
    public func urlSession(
        _ session: URLSession, didReceive challenge: URLAuthenticationChallenge,
        completionHandler: @escaping (URLSession.AuthChallengeDisposition, URLCredential?) -> Void
    ) {
        //Trust the certificate even if not valid
        let urlCredential = URLCredential(trust: challenge.protectionSpace.serverTrust!)

        completionHandler(.useCredential, urlCredential)
    }
}
