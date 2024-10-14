import java.lang.System;
import java.util.UUID;
import java.time.OffsetDateTime;

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
        String stateFile = System.getenv("STATE_FILE");

        // Configuring the URLS is optional, remove them to use the default values
        BitwardenSettings bitwardenSettings = new BitwardenSettings();
        bitwardenSettings.setApiUrl(apiUrl);
        bitwardenSettings.setIdentityUrl(identityUrl);

        try (BitwardenClient client = new BitwardenClient(bitwardenSettings)) {
            client.auth().loginAccessToken(accessToken, stateFile);

            ProjectResponse project = client.projects().create(organizationId, "Test Project from Java SDK");
            System.out.println("Project CREATE, id: " + project.getID());

            project = client.projects().get(project.getID());
            System.out.println("Project GET, id: " + project.getID());

            ProjectsResponse projects = client.projects().list(organizationId);
            System.out.println("Projects LIST, count: " + projects.getData().length);

            client.projects().update(organizationId, project.getID(), "Updated Test Project");
            project = client.projects().get(project.getID());
            System.out.println("Project UPDATE, new name: " + project.getName());

            SecretResponse secret = client.secrets().create(organizationId, "Secret Key", "Secret Value", "Secret Note", new UUID[]{project.getID()});
            System.out.println("Secret CREATE, id: " + secret.getID());

            secret = client.secrets().get(secret.getID());
            System.out.println("Secret GET, id: " + secret.getID());

            SecretIdentifiersResponse secrets = client.secrets().list(organizationId);
            System.out.println("Secrets LIST, count: " + secrets.getData().length);

            client.secrets().update(organizationId, secret.getID(), "Updated Key", "Updated Value", "Updated Note", new UUID[]{project.getID()});
            secret = client.secrets().get(secret.getID());
            System.out.println("Secret UPDATE, new key: " + secret.getKey());

            SecretsResponse secretsByIds = client.secrets().getByIds(new UUID[]{secret.getID()});
            System.out.println("Getting secrets by ids, here are the keys of the retrieved secrets...");
            for (SecretResponse sr : secretsByIds.getData()) {
                System.out.println("  " + sr.getKey());
            }

            SecretsSyncResponse syncResponse = client.secrets().sync(organizationId, OffsetDateTime.now());
            System.out.println("Running a secrets sync request based on the current time...");
            System.out.println("Has changes: " + syncResponse.getHasChanges());

            System.out.println("Deleting the created secret and project...");
            client.secrets().delete(new UUID[]{secret.getID()});
            client.projects().delete(new UUID[]{project.getID()});

            System.out.println("Execution complete.");
        }
    }
}
