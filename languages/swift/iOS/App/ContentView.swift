//
//  ContentView.swift
//  test
//
//  Created by Oscar on 2023-08-11.
//

import BitwardenSdk
import SwiftUI

/**
 *   IMPORTANT: This file is provided only for the purpose of demostrating the use of the SDK functionality.
 *   It hasn't gone through a throrough security review and should not be considered production ready. It also doesn't
 *   handle a lot of errors and edge cases that a production application would need to deal with.
 *   Developers are encouraged to review and improve the code as needed to meet their security requirements.
 *   Additionally, we recommend to consult with security experts and conduct thorough testing before using the code in production.
 */

let SERVER_URL = "https://localhost:8080/"
let API_URL = SERVER_URL + "api/"
let IDENTITY_URL = SERVER_URL + "identity/"

let EMAIL = "test@bitwarden.com"
let PASSWORD = "asdfasdfasdf"

let PIN = "1234"

struct ContentView: View {
    private var http: URLSession

    @State private var client: Client

    @State private var accessToken: String = ""

    init() {
        // Disable SSL Cert validation. Don't do this in production
        http = URLSession(
            configuration: URLSessionConfiguration.default, delegate: IgnoreHttpsDelegate(),
            delegateQueue: nil)

        client = Client(settings: nil)
    }

    @State var setupBiometrics: Bool = true
    @State var setupPin: Bool = true
    @State var outputText: String = ""

    var body: some View {
        VStack {
            Toggle("Setup biometric unlock after login", isOn: $setupBiometrics).padding(.init(top: 0, leading: 20, bottom: 0, trailing: 20))
            Toggle("Setup PIN unlock after login", isOn: $setupPin).padding(.init(top: 0, leading: 20, bottom: 0, trailing: 20))

            Button(action: {
                Task {
                    do {
                        try await clientExamplePassword(clientAuth: client.auth(), clientCrypto: client.crypto(), setupBiometrics: setupBiometrics, setupPin: setupPin)
                        try await decryptVault(clientCrypto: client.crypto(), clientVault: client.vault())
                    } catch {
                        print("ERROR:", error)
                    }
                }
            }, label: {
                Text("Login with username + password")
            })

            Divider().padding(30)

            Button(action: {
                Task {
                    do {
                        try await clientExampleBiometrics(clientCrypto: client.crypto())
                        try await decryptVault(clientCrypto: client.crypto(), clientVault: client.vault())
                    } catch {
                        print("ERROR:", error)
                    }
                }
            }, label: {
                Text("Unlock with biometrics")
            })

            Button(action: {
                Task {
                    do {
                        try await clientExamplePin(clientCrypto: client.crypto())
                        try await decryptVault(clientCrypto: client.crypto(), clientVault: client.vault())
                    } catch {
                        print("ERROR:", error)
                    }
                }
            }, label: {
                Text("Unlock with PIN")
            })

            Button(action: {
                Task {
                    do {
                        try await authenticatorTest(clientFido: client.platform().fido2())
                    } catch {
                        print("ERROR:", error)
                    }
                }
            }, label: {
                Text("Passkey")
            })

            Button(action: {
                client = Client(settings: nil)
            }, label: {
                Text("Lock & reset client")
            }).padding()

            Text("Output: " + outputText).padding(.top)
        }
        .padding()
    }

    func clientExamplePassword(clientAuth: ClientAuthProtocol, clientCrypto: ClientCryptoProtocol, setupBiometrics: Bool, setupPin: Bool) async throws {
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

        let passwordHash = try await clientAuth.hashPassword(
            email: EMAIL, password: PASSWORD, kdfParams: kdf, purpose: .serverAuthorization)

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
                        URLQueryItem(name: "password", value: passwordHash)
                    ]
                    r.httpBody = comp.percentEncodedQuery!
                        .replacingOccurrences(of: "@", with: "%40")
                        .replacingOccurrences(of: "+", with: "%2B")
                        .data(using: .utf8)
                }))
        let loginData = try JSONDecoder().decode(LoginResponse.self, from: loginDataJson)

        try await clientCrypto.initializeUserCrypto(
            req: InitUserCryptoRequest(
                kdfParams: kdf,
                email: EMAIL,
                privateKey: loginData.PrivateKey,
                method: InitUserCryptoMethod.password(
                    password: PASSWORD,
                    userKey: loginData.Key
                )
            ))

        accessToken = loginData.access_token

        if setupBiometrics {
            let defaults = UserDefaults.standard
            defaults.set(loginData.PrivateKey, forKey: "privateKey")
            defaults.set(preloginData.kdf, forKey: "kdfType")
            defaults.set(preloginData.kdfIterations, forKey: "kdfIterations")
            defaults.set(preloginData.kdfMemory, forKey: "kdfMemory")
            defaults.set(preloginData.kdfParallelism, forKey: "kdfParallelism")
            defaults.synchronize()

            let key = try await clientCrypto.getUserEncryptionKey()
            biometricStoreValue(value: key)
        }

        if setupPin {
            let pinOptions = try await clientCrypto.derivePinKey(pin: PIN)

            let defaults = UserDefaults.standard
            defaults.set(loginData.PrivateKey, forKey: "privateKey")
            defaults.set(preloginData.kdf, forKey: "kdfType")
            defaults.set(preloginData.kdfIterations, forKey: "kdfIterations")
            defaults.set(preloginData.kdfMemory, forKey: "kdfMemory")
            defaults.set(preloginData.kdfParallelism, forKey: "kdfParallelism")

            defaults.set(pinOptions.encryptedPin, forKey: "encryptedPin")
            defaults.set(pinOptions.pinProtectedUserKey, forKey: "pinProtectedUserKey")

            defaults.synchronize()
        }
    }

    func clientExampleBiometrics(clientCrypto: ClientCryptoProtocol) async throws {
        let defaults = UserDefaults.standard
        let privateKey = defaults.string(forKey: "privateKey")!
        let kdf = if defaults.integer(forKey: "kdfType") == 0 {
            Kdf.pbkdf2(iterations: UInt32(defaults.integer(forKey: "kdfIterations")))
        } else {
            Kdf.argon2id(
                iterations: UInt32(defaults.integer(forKey: "kdfIterations")),
                memory: UInt32(defaults.integer(forKey: "kdfMemory")),
                parallelism: UInt32(defaults.integer(forKey: "kdfParallelism"))
            )
        }

        let key = biometricRetrieveValue()!

        try await clientCrypto.initializeUserCrypto(req: InitUserCryptoRequest(
            kdfParams: kdf,
            email: EMAIL,
            privateKey: privateKey,
            method: InitUserCryptoMethod.decryptedKey(
                decryptedUserKey: key
            )
        ))
    }

    func clientExamplePin(clientCrypto: ClientCryptoProtocol) async throws {
        let defaults = UserDefaults.standard
        let privateKey = defaults.string(forKey: "privateKey")!
        let kdf = if defaults.integer(forKey: "kdfType") == 0 {
            Kdf.pbkdf2(iterations: UInt32(defaults.integer(forKey: "kdfIterations")))
        } else {
            Kdf.argon2id(
                iterations: UInt32(defaults.integer(forKey: "kdfIterations")),
                memory: UInt32(defaults.integer(forKey: "kdfMemory")),
                parallelism: UInt32(defaults.integer(forKey: "kdfParallelism"))
            )
        }

        let encryptedPin = defaults.string(forKey: "encryptedPin")!
        let pinProtectedUserKey = defaults.string(forKey: "pinProtectedUserKey")!

        try await clientCrypto.initializeUserCrypto(req: InitUserCryptoRequest(
            kdfParams: kdf,
            email: EMAIL,
            privateKey: privateKey,
            method: InitUserCryptoMethod.pin(pin: PIN, pinProtectedUserKey: pinProtectedUserKey)
        ))
    }

    func decryptVault(clientCrypto: ClientCryptoProtocol, clientVault: ClientVaultProtocol) async throws {
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
                        "Bearer " + accessToken, forHTTPHeaderField: "Authorization")
                }))

        let syncData = try JSONDecoder().decode(SyncResponse.self, from: syncDataJson)

        ///////////////////////////// Initialize org crypto /////////////////////////////

        try await clientCrypto.initializeOrgCrypto(
            req: InitOrgCryptoRequest(
                organizationKeys: Dictionary.init(
                    uniqueKeysWithValues: syncData.profile.organizations.map { ($0.id, $0.key) }
                )
            ))

        ///////////////////////////// Decrypt some folders /////////////////////////////

        let dateFormatter = ISO8601DateFormatter()
        dateFormatter.formatOptions = [.withFractionalSeconds]

        let decryptedFolders = try await clientVault.folders().decryptList(
            folders: syncData.folders.map {
                Folder(
                    id: $0.id, name: $0.name,
                    revisionDate: dateFormatter.date(from: $0.revisionDate)!)
            })
        print(decryptedFolders)
    }

    func authenticatorTest(clientFido: ClientFido2) async throws {
        let ui = UserInterfaceImpl()
        let cs = CredentialStoreImpl()
        let authenticator = clientFido.authenticator(userInterface: ui, credentialStore: cs)

        // Make credential
        try await authenticator.makeCredential(request: MakeCredentialRequest(
            clientDataHash: Data(),
            rp: PublicKeyCredentialRpEntity(id: "abc", name: "test"),
            user: PublicKeyCredentialUserEntity(id: Data(), displayName: "b", name: "c"),
            pubKeyCredParams: [PublicKeyCredentialParameters(ty: "public-key", alg: 0)],
            excludeList: nil,
            requireResidentKey: true,
            extensions: nil
        ))

        // Get Assertion
        try await authenticator.getAssertion(request: GetAssertionRequest(
            rpId: "",
            clientDataHash: Data(),
            allowList: nil,
            options: Options(rk: true, uv: Uv.preferred),
            extensions: nil
        ))

        try await authenticator.silentlyDiscoverCredentials(rpId: "")

        // Only on android!
        let client = clientFido.client(userInterface: ui, credentialStore: cs)
        try await client.authenticate(origin: "test", request: "test", clientData: ClientData.defaultWithExtraData(androidPackageName: "abc"))
        try await client.register(origin: "test", request: "test", clientData: ClientData.defaultWithExtraData(androidPackageName: "abc"))
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
        // Trust the certificate even if not valid
        let urlCredential = URLCredential(trust: challenge.protectionSpace.serverTrust!)

        completionHandler(.useCredential, urlCredential)
    }
}

class UserInterfaceImpl: Fido2UserInterface {
    func pickCredentialForAuthentication(availableCredentials: [BitwardenSdk.Cipher]) async throws -> BitwardenSdk.CipherViewWrapper {
        abort()
    }

    func pickCredentialForCreation(availableCredentials: [BitwardenSdk.Cipher], newCredential: BitwardenSdk.Fido2Credential) async throws -> BitwardenSdk.CipherViewWrapper {
        abort()
    }

    func checkUser(options: BitwardenSdk.CheckUserOptions, credential: BitwardenSdk.CipherView?) async throws -> BitwardenSdk.CheckUserResult {
        return CheckUserResult(userPresent: true, userVerified: true)
    }
}

class CredentialStoreImpl: Fido2CredentialStore {
    func findCredentials(ids: [Data]?, ripId: String) async throws -> [BitwardenSdk.Cipher] {
        abort()
    }

    func saveCredential(cred: BitwardenSdk.Cipher) async throws {
        print("SAVED CREDENTIAL")
    }
}
