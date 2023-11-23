package com.bitwarden.myapplication

import android.os.Build
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyProperties
import android.util.Log
import androidx.biometric.BiometricManager
import androidx.biometric.BiometricManager.Authenticators
import androidx.biometric.BiometricPrompt
import androidx.biometric.BiometricPrompt.CryptoObject
import androidx.core.content.ContextCompat
import androidx.fragment.app.FragmentActivity
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.async
import java.security.KeyStore
import java.util.Base64
import javax.crypto.Cipher
import javax.crypto.KeyGenerator
import javax.crypto.SecretKey
import javax.crypto.spec.GCMParameterSpec

/**
 *   IMPORTANT: This file is provided only for the purpose of demostrating the use of the biometric unlock functionality.
 *   It hasn't gone through a throrough security review and should not be considered production ready. It also doesn't 
 *   handle a lot of errors and edge cases that a production application would need to deal with. 
 *   Developers are encouraged to review and improve the code as needed to meet their security requirements. 
 *   Additionally, we recommend to consult with security experts and conduct thorough testing before using the code in production.
 */

class Biometric(private var activity: FragmentActivity) {
    private var promptInfo: BiometricPrompt.PromptInfo =
        BiometricPrompt.PromptInfo.Builder().setTitle("Unlock")
            .setSubtitle("Bitwarden biometric unlock")
            .setDescription("Confirm biometric to continue").setConfirmationRequired(true)
            .setNegativeButtonText("Use account password").build()

    suspend fun encryptString(
        keyName: String, plaintext: String, callback: (String, String) -> Unit
    ) {
        if (canAuthenticate()) {
            val cipher = getCipher()
            cipher.init(Cipher.ENCRYPT_MODE, getSecretKey(keyName))

            val bio = createBiometricPrompt {
                val ciphertext = it.cipher!!.doFinal(plaintext.toByteArray())
                callback(
                    String(Base64.getEncoder().encode(ciphertext)),
                    String(Base64.getEncoder().encode(cipher.iv))
                )
            }
            CoroutineScope(Dispatchers.Main).async {
                bio.authenticate(promptInfo, CryptoObject(cipher))
            }.await()
        }
    }

    suspend fun decryptString(
        keyName: String, encrypted: String, initializationVector: String, callback: (String) -> Unit
    ) {
        if (canAuthenticate()) {
            val enc = Base64.getDecoder().decode(encrypted)
            val iv = Base64.getDecoder().decode(initializationVector)

            val cipher = getCipher()
            cipher.init(Cipher.DECRYPT_MODE, getSecretKey(keyName), GCMParameterSpec(128, iv))

            val bio = createBiometricPrompt {
                callback(String(it.cipher!!.doFinal(enc)))
            }

            CoroutineScope(Dispatchers.Main).async {
                bio.authenticate(promptInfo, CryptoObject(cipher))
            }.await()
        }
    }

    private fun canAuthenticate() = BiometricManager.from(activity)
        .canAuthenticate(Authenticators.BIOMETRIC_STRONG) == BiometricManager.BIOMETRIC_SUCCESS

    private fun createBiometricPrompt(processData: (CryptoObject) -> Unit): BiometricPrompt {
        return BiometricPrompt(activity,
            ContextCompat.getMainExecutor(activity),
            object : BiometricPrompt.AuthenticationCallback() {
                override fun onAuthenticationError(errorCode: Int, errString: CharSequence) {
                    super.onAuthenticationError(errorCode, errString)
                    Log.e("Biometric", "$errorCode :: $errString")
                }

                override fun onAuthenticationFailed() {
                    super.onAuthenticationFailed()
                    Log.e("Biometric", "Authentication failed for an unknown reason")
                }

                override fun onAuthenticationSucceeded(result: BiometricPrompt.AuthenticationResult) {
                    super.onAuthenticationSucceeded(result)
                    processData(result.cryptoObject!!)
                }
            })
    }

    private fun getCipher(): Cipher {
        val transform =
            "${KeyProperties.KEY_ALGORITHM_AES}/${KeyProperties.BLOCK_MODE_GCM}/${KeyProperties.ENCRYPTION_PADDING_NONE}"
        return Cipher.getInstance(transform)
    }

    private fun getSecretKey(keyName: String): SecretKey {
        // If the SecretKey exists, return it
        val keyStore = KeyStore.getInstance("AndroidKeyStore")
        keyStore.load(null)
        keyStore.getKey(keyName, null)?.let { return it as SecretKey }

        // Otherwise, we generate a new one
        val keyGenParams = KeyGenParameterSpec.Builder(
            keyName, KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT
        ).apply {
            setBlockModes(KeyProperties.BLOCK_MODE_GCM)
            setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
            setKeySize(256)
            setUserAuthenticationRequired(true)
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
                setUserAuthenticationParameters(0, KeyProperties.AUTH_BIOMETRIC_STRONG)
            }
        }.build()

        val keyGenerator =
            KeyGenerator.getInstance(KeyProperties.KEY_ALGORITHM_AES, "AndroidKeyStore")
        keyGenerator.init(keyGenParams)
        return keyGenerator.generateKey()
    }
}
