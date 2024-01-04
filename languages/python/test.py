import bitwarden_sdk
from bitwarden_sdk import HashPurpose, Kdf
from pprint import pprint
import asyncio

async def main():
    client = bitwarden_sdk.Client(None)

    kdf = Kdf.PBKDF2(600000)

    password = await client.auth().hash_password("test@test.com", "password", kdf, HashPurpose.SERVER_AUTHORIZATION)

    pprint(password)

asyncio.run(main())
