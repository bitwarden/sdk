#include "BitwardenClient.h"
#include <boost/uuid/string_generator.hpp>
#include <cstdlib>
#include <chrono>

int main() {
    // Retrieve access token and organization ID from environment variables
    const char *accessTokenEnv = std::getenv("ACCESS_TOKEN");
    const char *organizationIdEnv = std::getenv("ORGANIZATION_ID");

    // Use optional state file for authentication
    const char *stateFile = std::getenv("STATE_PATH");

    const char *apiUrl = std::getenv("API_URL");
    const char *identityUrl = std::getenv("IDENTITY_URL");

    if (!accessTokenEnv || !organizationIdEnv) {
        std::cerr << "Error: Environment variables ACCESS_TOKEN or ORGANIZATION_ID not set." << std::endl;
        return 1;
    }

    std::string accessToken = accessTokenEnv;
    std::string organizationId = organizationIdEnv;

    // Configuring the URLS is optional, remove them to use the default values
    BitwardenSettings bitwardenSettings;
    bitwardenSettings.set_api_url(apiUrl);
    bitwardenSettings.set_identity_url(identityUrl);

    // Create a Bitwarden client instance
    BitwardenClient bitwardenClient(bitwardenSettings);

    // Access token login
    bitwardenClient.loginAccessToken(accessToken, stateFile);

    // Convert organization ID to UUID
    boost::uuids::uuid organizationUuid = boost::uuids::string_generator()(organizationId);

    // Create a new project
    std::cout << "Projects:\n";
    ProjectResponse projectResponseCreate = bitwardenClient.createProject(organizationUuid, "NewTestProject");
    boost::uuids::uuid projectId = boost::uuids::string_generator()(projectResponseCreate.get_id());

    std::cout << "\tcreateProject: '" << projectResponseCreate.get_name() << "'\n\n";

    // List projects
    ProjectsResponse projectResponseList = bitwardenClient.listProjects(organizationUuid);
    std::cout << "\tlistProjects:\n";
    for (const ProjectResponse& project : projectResponseList.get_data()) {
        std::cout << "\t\tID: '" << project.get_id() << "', Name: '" << project.get_name() << "'\n";
    }
    std::cout << '\n';

    // Get project details
    ProjectResponse projectResponseGet = bitwardenClient.getProject(projectId);
    std::cout << "\tgetProject:\n\t\tID: '" << projectResponseGet.get_id() << "', Name: '" << projectResponseGet.get_name() << "'\n\n";

    // Update project
    ProjectResponse projectResponseUpdate = bitwardenClient.updateProject(organizationUuid, projectId, "NewTestProject2");
    std::cout << "\tupdateProject: '" << projectResponseUpdate.get_name() << "'\n\n";

    // Secrets
    std::string key = "key";
    std::string value = "value";
    std::string note = "note";

    // Sync secrets
    std::cout << "Secrets:\n";
    std::cout << "\tSyncing secrets...\n";
    std::chrono::system_clock::time_point lastSyncedDate = std::chrono::system_clock::now();
    SecretsSyncResponse secretsSyncResponse = bitwardenClient.sync(organizationUuid, lastSyncedDate);
    std::cout << "\tSync has changes: '" << (secretsSyncResponse.get_has_changes() ? "true" : "false") << "'\n\n";

    // Create a new secret
    SecretResponse secretResponseCreate = bitwardenClient.createSecret(organizationUuid, key, value, note, {projectId});
    boost::uuids::uuid secretId = boost::uuids::string_generator()(secretResponseCreate.get_id());

    std::cout << "\tcreateSecret: '" << secretResponseCreate.get_key() << "'\n\n";

    // List secrets
    SecretIdentifiersResponse secretIdentifiersResponse = bitwardenClient.listSecrets(organizationUuid);

    // Get secret details
    SecretResponse secretResponseGet = bitwardenClient.getSecret(secretId);
    std::cout << "\tgetSecret: '" << secretResponseGet.get_key() << "'\n\n";

    // Get secrets by IDs
    std::cout << "\tgetSecretsByIds:\n";
    SecretsResponse secretsResponseGetByIds = bitwardenClient.getSecretsByIds({secretId});
    for (const SecretResponse& secret : secretsResponseGetByIds.get_data()) {
        std::cout << "\t\tID: '" << secret.get_id() << "', Key: '" << secret.get_key() << "'\n";
    }
    std::cout << '\n';

    // Update secret
    key = "updated-key";
    value = "updated-value";
    note = "updated-note";
    SecretResponse responseForSecretResponseUpdate = bitwardenClient.updateSecret(
        organizationUuid, secretId, key, value, note, {projectId});

    std::cout << "\tupdateSecret: '" << responseForSecretResponseUpdate.get_key() << "'\n\n";

    // Sync changes to secrets
    SecretsSyncResponse secretsSyncResponseAfterChanges = bitwardenClient.sync(organizationUuid, lastSyncedDate);
    std::cout << "\tSync has changes after update: '" << (secretsSyncResponseAfterChanges.get_has_changes() ? "true" : "false") << "'\n\n";

    // Delete secrets
    std::cout << "Deleting projects and secrets...\n";
    SecretsDeleteResponse secretsDeleteResponse = bitwardenClient.deleteSecrets({secretId});
    std::cout << "\tdeleteSecrets: '" << secretsDeleteResponse.get_data()[0].get_id() << "'\n\n";

    // Delete projects
    ProjectsDeleteResponse projectsDeleteResponse = bitwardenClient.deleteProjects({projectId});
    std::cout << "\tdeleteProjects: '" << projectsDeleteResponse.get_data()[0].get_id() << "'\n\n";

    return 0;
}
