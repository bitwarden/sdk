package com.bitwarden.myapplication

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
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

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val SERVER_URL = "https://10.0.2.2:8080/"
        val API_URL = SERVER_URL + "api/"
        val IDENTITY_URL = SERVER_URL + "identity/"

        val EMAIL = "test@bitwarden.com"
        val PASSWORD = "asdfasdfasdf"

        GlobalScope.launch {
            var client = Client(null)
            val http = httpClient()

            ///////////////////////////// Get master password hash /////////////////////////////
            @Serializable
            data class PreloginRequest(val email: String)

            @Serializable
            data class PreloginResponse(
                val kdf: UInt,
                val kdfIterations: UInt,
                val kdfMemory: UInt?,
                val kdfParallelism: UInt?
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

            ///////////////////////////// Sync /////////////////////////////

            val syncBody = http.get(API_URL + "sync?excludeDomains=true") {
                bearerAuth(loginBody.access_token)
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

            ///////////////////////////// Initialize crypto /////////////////////////////
            val orgs = ((syncBody["profile"] as JsonObject)["organizations"]) as JsonArray
            val orgKeys = HashMap<Uuid, String>()

            for (org in orgs) {
                val o = org as JsonObject
                orgKeys[(o["id"] as JsonPrimitive).content] = (o["key"] as JsonPrimitive).content
            }

            client.crypto().initializeUserCrypto(
                InitUserCryptoRequest(
                    kdfParams = kdf,
                    email = EMAIL,
                    privateKey = loginBody.PrivateKey,
                    method = InitUserCryptoMethod.Password(
                        password = PASSWORD,
                        userKey = loginBody.Key
                    )
                )
            )

            client.crypto().initializeOrgCrypto(
                InitOrgCryptoRequest(
                    organizationKeys = orgKeys
                )
            )

            ///////////////////////////// Decrypt some folders /////////////////////////////

            val decryptedFolders = client.vault().folders().decryptList(folders)

            println(decryptedFolders)

        }

        setContent {
            MyApplicationTheme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background
                ) {
                    Greeting("Hey")
                }
            }
        }
    }
}

@Composable
fun Greeting(name: String, modifier: Modifier = Modifier) {
    Text(
        text = "Hello $name!", modifier = modifier
    )
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    MyApplicationTheme {
        Greeting("Sdk")
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
