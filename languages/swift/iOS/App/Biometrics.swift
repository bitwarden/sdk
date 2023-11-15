//
//  Biometrics.swift
//  App
//
//  Created by Dani on 15/11/23.
//


import Foundation
import LocalAuthentication

let SERVICE: String = "com.example.app"

// We should separate keys for each user by appending the user_id
let KEY: String = "biometric_key"


func biometricStoreValue(value: String) {
    var error: Unmanaged<CFError>?
    let accessControl = SecAccessControlCreateWithFlags(
        nil,
        kSecAttrAccessibleWhenUnlockedThisDeviceOnly,
        .biometryCurrentSet,
        &error)
    
    guard accessControl != nil && error == nil else {
        fatalError("SecAccessControlCreateWithFlags failed")
    }
        
    let query = [
        kSecClass: kSecClassGenericPassword,
        kSecAttrService: SERVICE,
        kSecAttrAccount: KEY,
        kSecValueData: value.data(using: .utf8)!,
        kSecAttrAccessControl: accessControl as Any
    ] as CFDictionary
    
    // Try to delete the previous secret, if it exists
    // Otherwise we get `errSecDuplicateItem`
    SecItemDelete(query)
    
    let status = SecItemAdd(query, nil)    
    guard status == errSecSuccess else {
        fatalError("Unable to store the secret: " + errToString(status: status))
    }
}

private func errToString(status: OSStatus) -> String {
    if let err = SecCopyErrorMessageString(status, nil) as String? {
        err
    } else {
        "Unknown error"
    }
}

func biometricRetrieveValue() -> String? {
    let searchQuery = [
        kSecClass: kSecClassGenericPassword,
        kSecAttrService: SERVICE,
        kSecAttrAccount: KEY,
        kSecMatchLimit: kSecMatchLimitOne,
        kSecReturnData: true,
        kSecReturnAttributes: true,
    ] as CFDictionary
    
    var item: AnyObject?
    let status = SecItemCopyMatching(searchQuery, &item)

    // If the item is not found, we just return nil
    if status == errSecItemNotFound {
        return nil
    }

    // TODO: We probably want to handle these errors better
    guard status == noErr else {
        fatalError("Unable to retrieve the secret: " + errToString(status: status))
    }
    
    if let resultDictionary = item as? [String: Any],
        let data = resultDictionary[kSecValueData as String] as? Data {
        return String(decoding: data, as: UTF8.self)
    }
    
    return nil
}
