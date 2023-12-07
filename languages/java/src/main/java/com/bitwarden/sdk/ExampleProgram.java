package com.bitwarden.sdk;

import com.bitwarden.sdk.schema.*;

import java.util.UUID;

public class ExampleProgram {

    public static void main(String[] args) {
        BitwardenSettings bitwardenSettings = new BitwardenSettings();
        bitwardenSettings.setApiUrl("https://api.bitwarden.com");
        bitwardenSettings.setIdentityUrl("https://identity.bitwarden.com");

        try (BitwardenClient bitwardenClient = new BitwardenClient(bitwardenSettings)) {
            APIKeyLoginResponse apiKeyLoginResponse = bitwardenClient.accessTokenLogin("<access-token>");

            UUID organizationId = UUID.fromString("<organization-id>");
            ProjectResponse projectResponse = bitwardenClient.projects().create(organizationId, "NewTestProject");
            UUID projectId = projectResponse.getID();
            ProjectsResponse projectsResponse = bitwardenClient.projects().list(organizationId);
            projectResponse = bitwardenClient.projects().get(projectId);
            projectResponse = bitwardenClient.projects().update(projectId, organizationId, "NewTestProject2");

            String key = "key";
            String value = "value";
            String note = "note";
            SecretResponse secretResponse = bitwardenClient.secrets().create(key, value, note, organizationId,
                new UUID[]{projectId});
            UUID secretId = secretResponse.getID();
            SecretIdentifiersResponse secretIdentifiersResponse = bitwardenClient.secrets().list(organizationId);
            secretResponse = bitwardenClient.secrets().get(secretId);
            key = "key2";
            value = "value2";
            note = "note2";
            secretResponse = bitwardenClient.secrets().update(secretId, key, value, note, organizationId,
                new UUID[]{projectId});

            SecretsDeleteResponse secretsDeleteResponse = bitwardenClient.secrets().delete(new UUID[]{secretId});
            ProjectsDeleteResponse projectsDeleteResponse = bitwardenClient.projects().delete(new UUID[]{projectId});
        }
    }
}
