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

result = client.access_token_login("access token here")

secret = client.secrets().create("TEST_SECRET", "This is a test secret", "organization id here", "Secret1234!", ["project id here"])

input("Press Enter to delete the secret...")

client.secrets().delete([secret.data.id])
