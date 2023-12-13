import java.lang.System;
import java.util.UUID;

import com.bitwarden.sdk.*;
import com.bitwarden.sdk.schema.*;

class Example {
    public static void main(String[] args) {

        String accessToken = System.getenv("ACCESS_TOKEN");
        UUID organizationId = UUID.fromString(System.getenv("ORGANIZATION_ID"));
        String apiUrl = System.getenv("API_URL");
        String identityUrl = System.getenv("IDENTITY_URL");

        // Configuring the URLS is optional, remove them to use the default values
        BitwardenSettings bitwardenSettings = new BitwardenSettings();
        bitwardenSettings.setApiUrl(apiUrl);
        bitwardenSettings.setIdentityUrl(identityUrl);
        BitwardenClient client = new BitwardenClient(bitwardenSettings);
        client.accessTokenLogin(accessToken);


        ProjectResponse project = client.projects().create(organizationId, "Test Project");
        ProjectsResponse list = client.projects().list(organizationId);
        
        SecretResponse secret = client.secrets().create("Secret Key", "Secret Value", "Secret Note", organizationId, new UUID[] { project.getID() });

        System.out.println("Secret: " + secret.getValue());

        client.secrets().delete(new UUID[] { secret.getID() });
        client.projects().delete(new UUID[] { project.getID() });
    }
}
