#!/usr/bin/env python3
import logging
import os

from bitwarden_sdk import BitwardenClient, DeviceType, client_settings_from_dict

# Create the BitwardenClient, which is used to interact with the SDK
client = BitwardenClient(
    client_settings_from_dict(
        {
            "apiUrl": os.getenv("API_URL", "http://localhost:4000"),
            "deviceType": DeviceType.SDK,
            "identityUrl": os.getenv("IDENTITY_URL", "http://localhost:33656"),
            "userAgent": "Python",
        }
    )
)

# Add some logging & set the org id
logging.basicConfig(level=logging.DEBUG)
organization_id = os.getenv("ORGANIZATION_ID")

# Attempt to authenticate with the Secrets Manager Access Token
client.access_token_login(os.getenv("ACCESS_TOKEN"))

# -- Example Project Commands --

project = client.projects().create("ProjectName", organization_id)
project2 = client.projects().create("Project - Don't Delete Me!", organization_id)
updated_project = client.projects().update(
    project.data.id, "Cool New Project Name", organization_id
)
get_that_project = client.projects().get(project.data.id)

input("Press Enter to delete the project...")
client.projects().delete([project.data.id])

print(client.projects().list(organization_id))

# -- Example Secret Commands --

secret = client.secrets().create(
    "TEST_SECRET",
    "This is a test secret",
    organization_id,
    "Secret1234!",
    [project2.data.id],
)
secret2 = client.secrets().create(
    "Secret - Don't Delete Me!",
    "This is a test secret that will stay",
    organization_id,
    "Secret1234!",
    [project2.data.id],
)
secret_updated = client.secrets().update(
    secret.data.id,
    "TEST_SECRET_UPDATED",
    "This as an updated test secret",
    organization_id,
    "Secret1234!_updated",
    [project2.data.id],
)
secret_retrieved = client.secrets().get(secret.data.id)

input("Press Enter to delete the secret...")
client.secrets().delete([secret.data.id])

print(client.secrets().list(organization_id))
