#include "BitwardenClient.h"
#include <boost/uuid/string_generator.hpp>
#include <cstdlib>

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
    BitwardenClient bitwardenClient = BitwardenClient(bitwardenSettings);
    // // Access token login
    bitwardenClient.loginAccessToken(accessToken, stateFile);
    // Organization ID
    boost::uuids::uuid organizationUuid = boost::uuids::string_generator()(organizationId);

    // // Create a new project
    ProjectResponse projectResponseCreate = bitwardenClient.createProject(organizationUuid, "NewTestProject");
    boost::uuids::uuid projectId = boost::uuids::string_generator()(projectResponseCreate.get_id());

    printf("Created project: '%s'\n", projectResponseCreate.get_name().c_str());

    // List projects
    ProjectsResponse projectResponseList = bitwardenClient.listProjects(organizationUuid);

    printf("List of projects:\n");
    for (const auto &project: projectResponseList.get_data()) {
        printf("Project ID: %s, Name: %s\n", project.get_id().c_str(), project.get_name().c_str());
    }

    // Get project details
    ProjectResponse projectResponseGet = bitwardenClient.getProject(projectId);

    printf("Project ID: %s, Name: %s\n", projectResponseGet.get_id().c_str(), projectResponseGet.get_name().c_str());

    // Update project
    ProjectResponse ProjectResponseUpdate = bitwardenClient.updateProject(
        projectId, organizationUuid, "NewTestProject2");

    printf("Updated project '%s'\n", ProjectResponseUpdate.get_name().c_str());

    // Secrets
    std::string key = "key";
    std::string value = "value";
    std::string note = "note";

    // Create a new secret
    SecretResponse secretResponseCreate = bitwardenClient.createSecret(key, value, note, organizationUuid, {projectId});
    boost::uuids::uuid secretId = boost::uuids::string_generator()(secretResponseCreate.get_id());

    printf("Created secret: '%s'\n", secretResponseCreate.get_key().c_str());

    // List secrets
    SecretIdentifiersResponse secretIdentifiersResponse = bitwardenClient.listSecrets(organizationUuid);

    // Get secret details
    SecretResponse secretResponseGet = bitwardenClient.getSecret(secretId);

    printf("List of secrets:\n");
    for (const auto &secret: secretIdentifiersResponse.get_data()) {
        printf("Secret ID: %s, Key: %s\n", secret.get_id().c_str(), secret.get_key().c_str());
    }

    // Update secret
    key = "key2";
    value = "value2";
    note = "note2";
    SecretResponse responseForSecretResponseUpdate = bitwardenClient.updateSecret(
        secretId, key, value, note, organizationUuid, {projectId});

    printf("Updated secret: '%s'\n", responseForSecretResponseUpdate.get_key().c_str());

    // Delete secrets
    SecretsDeleteResponse secretsDeleteResponse = bitwardenClient.deleteSecrets({secretId});

    printf("Deleted secret: '%s'\n", secretsDeleteResponse.get_data()[0].get_id().c_str());

    // Delete projects
    ProjectsDeleteResponse projectsDeleteResponse = bitwardenClient.deleteProjects({projectId});

    printf("Deleted project: '%s'\n", projectsDeleteResponse.get_data()[0].get_id().c_str());

    return 0;
}
