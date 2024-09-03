import java.lang.System;
import java.util.UUID;

import com.bitwarden.sdk.*;
import com.bitwarden.sdk.schema.*;

class Example {
    public static void main(String[] args) {
        if (!System.getenv().containsKey("ACCESS_TOKEN") || !System.getenv().containsKey("ORGANIZATION_ID")) {
            System.err.println("Missing environment variable ACCESS_TOKEN or ORGANIZATION_ID");
            System.exit(1);
        }

        String accessToken = System.getenv("ACCESS_TOKEN");
        UUID organizationId = UUID.fromString(System.getenv("ORGANIZATION_ID"));
        String apiUrl = System.getenv("API_URL");
        String identityUrl = System.getenv("IDENTITY_URL");

        // Configuring the URLS is optional, remove them to use the default values
        BitwardenSettings bitwardenSettings = new BitwardenSettings();
        bitwardenSettings.setApiUrl(apiUrl);
        bitwardenSettings.setIdentityUrl(identityUrl);

        try (BitwardenClient client = new BitwardenClient(bitwardenSettings)) {
            client.auth().loginAccessToken(accessToken, null);

            ProjectResponse project = client.projects().create(organizationId, "Test Project");
            System.out.println("Project id: " + project.getID());

            project = client.projects().get(project.getID());

            ProjectsResponse projects = client.projects().list(organizationId);
            System.out.println("Projects count: " + projects.getData().length);

            client.projects().update(project.getID(), organizationId, "Updated Test Project");

            SecretResponse secret = client.secrets().create("Secret Key", "Secret Value", "Secret Note",
                organizationId, new UUID[]{project.getID()});
            System.out.println("Secret id: " + secret.getID());

            secret = client.secrets().get(secret.getID());

            SecretIdentifiersResponse secrets = client.secrets().list(organizationId);
            System.out.println("Secrets count: " + secrets.getData().length);

            client.secrets().update(secret.getID(), "Updated Key", "Updated Value", "Updated Noye", organizationId, new UUID[]{project.getID()});

            client.secrets().delete(new UUID[]{secret.getID()});
            client.projects().delete(new UUID[]{project.getID()});
        }
    }
}
