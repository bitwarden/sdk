package com.bitwarden.myapplication

import android.content.Context
import android.os.Bundle
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.Checkbox
import androidx.compose.material3.Divider
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Alignment.Companion.CenterVertically
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import androidx.fragment.app.FragmentActivity
import com.bitwarden.core.DateTime
import com.bitwarden.core.Folder
import com.bitwarden.core.InitOrgCryptoRequest
import com.bitwarden.core.InitUserCryptoMethod
import com.bitwarden.core.InitUserCryptoRequest
import com.bitwarden.core.Kdf
import com.bitwarden.core.Uuid
import com.bitwarden.myapplication.ui.theme.MyApplicationTheme
import com.bitwarden.sdk.Client
import io.ktor.client.HttpClient
import io.ktor.client.call.body
import io.ktor.client.engine.cio.CIO
import io.ktor.client.plugins.contentnegotiation.ContentNegotiation
import io.ktor.client.request.bearerAuth
import io.ktor.client.request.forms.FormDataContent
import io.ktor.client.request.get
import io.ktor.client.request.header
import io.ktor.client.request.post
import io.ktor.client.request.setBody
import io.ktor.http.ContentType
import io.ktor.http.Parameters
import io.ktor.http.contentType
import io.ktor.serialization.kotlinx.json.json
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.launch
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.JsonPrimitive
import java.security.cert.X509Certificate
import java.util.Base64
import javax.net.ssl.X509TrustManager

const val SERVER_URL = "https://10.0.2.2:8080/"
const val API_URL = SERVER_URL + "api/"
const val IDENTITY_URL = SERVER_URL + "identity/"

const val EMAIL = "test@bitwarden.com"
const val PASSWORD = "asdfasdfasdf"

// We should separate keys for each user by appending the user_id
const val BIOMETRIC_KEY = "biometric_key"

class MainActivity : FragmentActivity() {
    private lateinit var biometric: Biometric
    private lateinit var client: Client
    private lateinit var http: HttpClient

    private var accessToken = ""

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        biometric = Biometric(this)
        client = Client(null)
        http = httpClient()

        setContent {
            MyApplicationTheme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background
                ) {
                    Column(
                        horizontalAlignment = Alignment.CenterHorizontally
                    ) {

                        val setupBiometrics = remember { mutableStateOf(true) }
                        val outputText = remember { mutableStateOf("") }

                        Row {
                            Checkbox(checked = setupBiometrics.value,
                                onCheckedChange = { isChecked ->
                                    setupBiometrics.value = isChecked
                                })
                            Text(
                                "Setup biometric unlock after login",
                                modifier = Modifier.align(CenterVertically)
                            )
                        }

                        Button({
                            GlobalScope.launch {
                                clientExamplePassword(
                                    client, http, outputText, setupBiometrics.value
                                )
                            }
                        }) {
                            Text("Login with username + password")
                        }

                        Divider(
                            color = Color.Black,
                            thickness = 1.dp,
                            modifier = Modifier.padding(30.dp)
                        )

                        Button({
                            GlobalScope.launch {
                                clientExampleBiometrics(client, http, outputText)
                            }
                        }) {
                            Text("Unlock with biometrics")
                        }

                        Button({
                            GlobalScope.launch {
                                client.destroy()
                                client = Client(null)
                                outputText.value = "OK"
                            }
                        }) {
                            Text("Lock & reset client")
                        }

                        Text(
                            "Output: " + outputText.value,
                            modifier = Modifier.padding(vertical = 10.dp)
                        )
                    }
                }
            }
        }
    }

    private suspend fun clientExamplePassword(
        client: Client, http: HttpClient, outputText: MutableState<String>, setupBiometrics: Boolean
    ) {
        println("### Logging in with username and password ###")
        ///////////////////////////// Get master password hash /////////////////////////////
        @Serializable
        data class PreloginRequest(val email: String)

        @Serializable
        data class PreloginResponse(
            val kdf: UInt, val kdfIterations: UInt, val kdfMemory: UInt?, val kdfParallelism: UInt?
        )

        val prelogin_body = http.post(IDENTITY_URL + "accounts/prelogin") {
            contentType(ContentType.Application.Json)
            setBody(PreloginRequest(EMAIL))
        }.body<PreloginResponse>()
        val kdf = if (prelogin_body.kdf == 0u) {
            Kdf.Pbkdf2(prelogin_body.kdfIterations)
        } else {
            Kdf.Argon2id(
                prelogin_body.kdfIterations,
                prelogin_body.kdfMemory!!,
                prelogin_body.kdfParallelism!!
            )
        }
        val masterPasswordHash = client.auth().hashPassword(EMAIL, PASSWORD, kdf)

        ///////////////////////////// Login /////////////////////////////

        @Serializable
        data class LoginResponse(
            val Key: String,
            val PrivateKey: String,
            val access_token: String,
            val refresh_token: String,
        )

        val loginBody = http.post(IDENTITY_URL + "connect/token") {
            contentType(ContentType.Application.Json)
            header("Auth-Email", Base64.getEncoder().encodeToString(EMAIL.toByteArray()))
            setBody(FormDataContent(Parameters.build {
                append("scope", "api offline_access")
                append("client_id", "web")
                append("deviceType", "12")
                append("deviceIdentifier", "0745d426-8dab-484a-9816-4959721d77c7")
                append("deviceName", "edge")

                append("grant_type", "password")
                append("username", EMAIL)
                append("password", masterPasswordHash)
            }))
        }.body<LoginResponse>()

        client.crypto().initializeUserCrypto(
            InitUserCryptoRequest(
                kdfParams = kdf,
                email = EMAIL,
                privateKey = loginBody.PrivateKey,
                method = InitUserCryptoMethod.Password(
                    password = PASSWORD, userKey = loginBody.Key
                )
            )
        )

        accessToken = loginBody.access_token

        decryptVault(client, http, outputText)

        if (setupBiometrics) {
            // Save values for future logins
            val sharedPref = getPreferences(Context.MODE_PRIVATE)
            with(sharedPref.edit()) {
                putString("accessToken", accessToken)
                putString("privateKey", loginBody.PrivateKey)

                putInt("kdfType", prelogin_body.kdf.toInt())
                putInt("kdfIterations", prelogin_body.kdfIterations.toInt())
                putInt("kdfMemory", (prelogin_body.kdfMemory ?: 0u).toInt())
                putInt("kdfParallelism", (prelogin_body.kdfParallelism ?: 0u).toInt())

                // TODO: This should be protected by Android's secure KeyStore
                val decryptedKey = client.crypto().getUserEncryptionKey()

                biometric.encryptString(BIOMETRIC_KEY, decryptedKey) { key, iv ->
                    putString("encryptedUserKey", key)
                    putString("encryptedUserKeyIv", iv)
                    apply()
                }
            }
        }
    }

    private suspend fun clientExampleBiometrics(
        client: Client, http: HttpClient, outputText: MutableState<String>
    ) {
        println("### Unlocking with Biometrics ###")

        val pref = getPreferences(Context.MODE_PRIVATE)
        accessToken = pref.getString("accessToken", "").orEmpty()
        val privateKey = pref.getString("privateKey", "")

        val kdf = if (pref.getInt("kdfType", 0) == 0) {
            Kdf.Pbkdf2(pref.getInt("kdfIterations", 0).toUInt())
        } else {
            Kdf.Argon2id(
                pref.getInt("kdfIterations", 0).toUInt(),
                pref.getInt("kdfMemory", 0).toUInt(),
                pref.getInt("kdfParallelism", 0).toUInt()
            )
        }

        val encryptedUserKey = pref.getString("encryptedUserKey", "")!!
        val encryptedUserKeyIv = pref.getString("encryptedUserKeyIv", "")!!

        biometric.decryptString(
            BIOMETRIC_KEY, encryptedUserKey, encryptedUserKeyIv
        ) { key ->
            GlobalScope.launch {
                client.crypto().initializeUserCrypto(
                    InitUserCryptoRequest(
                        kdfParams = kdf,
                        email = EMAIL,
                        privateKey = privateKey!!,
                        method = InitUserCryptoMethod.DecryptedKey(decryptedUserKey = key)
                    )
                )

                decryptVault(client, http, outputText)
            }
        }
    }

    suspend fun decryptVault(client: Client, http: HttpClient, outputText: MutableState<String>) {
        ///////////////////////////// Sync /////////////////////////////

        val syncBody = http.get(API_URL + "sync?excludeDomains=true") {
            bearerAuth(accessToken)
        }.body<JsonObject>()

        val folders = (syncBody["folders"] as JsonArray).map {
            val o = it as JsonObject
            Folder(
                (o["id"] as JsonPrimitive).content,
                (o["name"] as JsonPrimitive).content,
                DateTime.parse(
                    (o["revisionDate"] as JsonPrimitive).content
                )
            )
        }

        ///////////////////////////// Initialize org crypto /////////////////////////////
        val orgs = ((syncBody["profile"] as JsonObject)["organizations"]) as JsonArray
        val orgKeys = HashMap<Uuid, String>()

        for (org in orgs) {
            val o = org as JsonObject
            orgKeys[(o["id"] as JsonPrimitive).content] = (o["key"] as JsonPrimitive).content
        }

        client.crypto().initializeOrgCrypto(InitOrgCryptoRequest(organizationKeys = orgKeys))

        ///////////////////////////// Decrypt some folders /////////////////////////////

        val decryptedFolders = client.vault().folders().decryptList(folders)
        outputText.value = decryptedFolders.toString()
        println(decryptedFolders)
    }
}

fun httpClient(): HttpClient {
    return HttpClient(CIO) {
        install(ContentNegotiation) {
            json(Json {
                ignoreUnknownKeys = true
            })
        }

        engine {
            https {
                // Disable SSL Cert validation. Don't do this in production
                trustManager = object : X509TrustManager {
                    override fun checkClientTrusted(
                        p0: Array<out X509Certificate>?, p1: String?
                    ) {
                    }

                    override fun checkServerTrusted(
                        p0: Array<out X509Certificate>?, p1: String?
                    ) {
                    }

                    override fun getAcceptedIssuers(): Array<X509Certificate>? = null
                }
            }
        }
    }
}
