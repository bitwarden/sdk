package bit.sdk;

import bit.sdk.schema.ClientSettings;
import bit.sdk.schema.DeviceType;
import bit.sdk.schema.ResponseForPasswordLoginResponse;
import com.fasterxml.jackson.core.JsonProcessingException;

public class Wrapper {

    public static void main(String[] args) throws JsonProcessingException {
        ClientSettings clientSettings = new ClientSettings();
        clientSettings.setAPIURL("https://api.bitwarden.com");
        clientSettings.setIdentityURL("https://identity.bitwarden.com");
        clientSettings.setDeviceType(DeviceType.SDK);
        clientSettings.setUserAgent("Bitwarden SDK");
        BitwardenClient bitwardenClient = new BitwardenClient(clientSettings);
        ResponseForPasswordLoginResponse passwordLoginResponse = bitwardenClient.passwordLogin(
            "user@something.com",
            "Pass");
        bitwardenClient.close();
    }
}
