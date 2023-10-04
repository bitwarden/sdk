package bit.sdk;

import bit.sdk.schema.*;

import java.util.UUID;

public class ExampleProgram {

    public static void main(String[] args) {
        BitwardenSettings bitwardenSettings = new BitwardenSettings();
        bitwardenSettings.setApiUrl("https://api.bitwarden.com");
        bitwardenSettings.setIdentityUrl("https://identity.bitwarden.com");

        BitwardenClient bitwardenClient = new BitwardenClient(bitwardenSettings);
        ResponseForAPIKeyLoginResponse responseForAPIKeyLoginResponse = bitwardenClient.accessTokenLogin(
            "<access-token>");

        UUID organizationId = UUID.fromString("<organization-id>");
        ResponseForProjectResponse responseForProjectResponse = bitwardenClient.projects().create(organizationId,
            "NewTestProject");
        UUID projectId = responseForProjectResponse.getData().getID();
        ResponseForProjectsResponse responseForProjectsResponse = bitwardenClient.projects().list(organizationId);
        responseForProjectResponse = bitwardenClient.projects().get(projectId);
        responseForProjectResponse = bitwardenClient.projects().update(projectId, organizationId, "NewTestProject2");

        String key = "key";
        String value = "value";
        String note = "note";
        ResponseForSecretResponse responseForSecretResponse =
            bitwardenClient.secrets().create(key, value, note, organizationId, new UUID[]{projectId});
        UUID secretId = responseForSecretResponse.getData().getID();
        ResponseForSecretIdentifiersResponse responseForSecretIdentifiersResponse =
            bitwardenClient.secrets().list(organizationId);
        responseForSecretResponse = bitwardenClient.secrets().get(secretId);
        key = "key2";
        value = "value2";
        note = "note2";
        responseForSecretResponse = bitwardenClient.secrets().update(secretId, key, value, note, organizationId,
            new UUID[]{projectId});

        ResponseForSecretsDeleteResponse responseForSecretsDeleteResponse =
            bitwardenClient.secrets().delete(new UUID[]{secretId});
        ResponseForProjectsDeleteResponse responseForProjectsDeleteResponse = bitwardenClient.projects().delete(
            new UUID[]{projectId});

        bitwardenClient.close();
    }
}
