import json
import logging
from BitwardenClient.bitwarden_client import BitwardenClient
from BitwardenClient.schemas import client_settings_from_dict

client = BitwardenClient(client_settings_from_dict({
    "apiUrl": "http://localhost:4000",
    "deviceType": "SDK",
    "identityUrl": "http://localhost:33656",
    "userAgent": "Python",
}))

logging.basicConfig(level=logging.DEBUG)

result = client.password_login("test@bitwarden.com", "asdfasdf")
print(result)
print(client.get_user_api_key("asdfasdf"))

sync = client.sync()

secret = client.secrets().create("TEST_SECRET", "This is a test secret",
                                 sync.data.profile.organizations[0].id, "Secret1234!")
print(secret)

client.secrets().delete([secret.data.id])
