#include "BitwardenClient.h"
#include <boost/uuid/string_generator.hpp>
#include <cstdlib>

int main() {
    // Retrieve access token and organization ID from environment variables
    const char* accessTokenEnv = std::getenv("ACCESS_TOKEN");
    const char* organizationIdEnv = std::getenv("ORGANIZATION_ID");

    boost::optional<std::string> apiUrl("https://api.bitwarden.com");
    boost::optional<std::string> identityUrl("https://identity.bitwarden.com");
    boost::optional<std::string> user_agent("Bitwarden CPP-SDK");

    if (!accessTokenEnv || !organizationIdEnv) {
        std::cerr << "Error: Environment variables ACCESS_TOKEN or ORGANIZATION_ID not set." << std::endl;
        return 1;
    }

    std::string accessToken = accessTokenEnv;
    std::string organizationId = organizationIdEnv;

    ClientSettings clientSettings;
    // Initialize ClientSettings with default values
    clientSettings.set_api_url(apiUrl);
    clientSettings.set_identity_url(identityUrl);
    clientSettings.set_device_type(Bitwarden::Sdk::DeviceType::SDK);
    clientSettings.set_user_agent(user_agent);

    // Create a Bitwarden client instance
    BitwardenClient bitwardenClient = BitwardenClient(clientSettings);
    // // Access token login
    ResponseForApiKeyLoginResponse responseForApiKeyLoginResponse = bitwardenClient.accessTokenLogin(accessToken);
    // Organization ID
    boost::uuids::uuid organizationUuid = boost::uuids::string_generator()(organizationId);    
    
    // // Create a new project
    ResponseForProjectResponse responseForProjectResponseCreate = bitwardenClient.createProject(organizationUuid, "NewTestProject");
    boost::uuids::uuid projectId = boost::uuids::string_generator()(responseForProjectResponseCreate.get_data()->get_id());
   
    // List projects
    ResponseForProjectsResponse responseForProjectResponseList = bitwardenClient.listProjects(organizationUuid);

    // Get project details
    ResponseForProjectResponse responseForProjectResponseGet = bitwardenClient.getProject(projectId);

    // Update project
    ResponseForProjectResponse responseForProjectResponseUpdate = bitwardenClient.updateProject(projectId, organizationUuid, "NewTestProject2");

    // Secrets
    std::string key = "key";
    std::string value = "value";
    std::string note = "note";

    // Create a new secret
    ResponseForSecretResponse responseForSecretResponseCreate = bitwardenClient.createSecret(key, value, note, organizationUuid, {projectId});
    boost::uuids::uuid secretId = boost::uuids::string_generator()(responseForSecretResponseCreate.get_data()->get_id());

    // List secrets
    ResponseForSecretIdentifiersResponse responseForSecretIdentifiersResponse = bitwardenClient.listSecrets(organizationUuid);

    // Get secret details
    ResponseForSecretResponse responseForSecretResponseGet = bitwardenClient.getSecret(secretId);

    // Update secret
    key = "key2";
    value = "value2";
    note = "note2";
    ResponseForSecretResponse responseForSecretResponseUpdate = bitwardenClient.updateSecret(secretId, key, value, note, organizationUuid, {projectId});

    // Delete secrets
    ResponseForSecretsDeleteResponse responseForSecretsDeleteResponse = bitwardenClient.deleteSecrets({secretId});

    // Delete projects
    ResponseForProjectsDeleteResponse responseForProjectsDeleteResponse = bitwardenClient.deleteProjects({projectId});

    return 0;
}
