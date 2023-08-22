package bit.sdk;

import bit.sdk.schema.ClientSettings;
import bit.sdk.schema.DeviceType;
import bit.sdk.schema.ResponseForAPIKeyLoginResponse;
import bit.sdk.schema.ResponseForProjectResponse;
import bit.sdk.schema.ResponseForProjectsDeleteResponse;
import bit.sdk.schema.ResponseForProjectsResponse;
import bit.sdk.schema.ResponseForSecretIdentifiersResponse;
import bit.sdk.schema.ResponseForSecretResponse;
import bit.sdk.schema.ResponseForSecretsDeleteResponse;
import com.fasterxml.jackson.core.JsonProcessingException;
import java.util.UUID;

public class Wrapper {

    public static void main(String[] args) throws JsonProcessingException {
        ClientSettings clientSettings = new ClientSettings();
        clientSettings.setAPIURL("https://api.bitwarden.com");
        clientSettings.setIdentityURL("https://identity.bitwarden.com");
        clientSettings.setDeviceType(DeviceType.SDK);
        clientSettings.setUserAgent("Bitwarden SDK");

        BitwardenClient bitwardenClient = new BitwardenClient(clientSettings);
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
