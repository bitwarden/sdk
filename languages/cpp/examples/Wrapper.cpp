#include "BitwardenClient.h"
#include <boost/uuid/string_generator.hpp>

int main() {
    ClientSettings clientSettings;
    // Initialize ClientSettings with default values
    clientSettings.set_api_url("https://api.bitwarden.com");
    clientSettings.set_identity_url("https://identity.bitwarden.com");
    clientSettings.set_device_type(quicktype::DeviceType::SDK);
    clientSettings.set_user_agent("Bitwarden CPP-SDK");

    // Create a Bitwarden client instance
    BitwardenClient bitwardenClient = BitwardenClient(clientSettings);
    // // Access token login
    ResponseForApiKeyLoginResponse responseForApiKeyLoginResponse = bitwardenClient.accessTokenLogin("<access-token>");
    // Organization ID
    boost::uuids::uuid organizationId = boost::uuids::string_generator()("<organization-id>");    
    
    // // Create a new project
    ResponseForProjectResponse responseForProjectResponseCreate = bitwardenClient.createProject(organizationId, "NewTestProject");
    boost::uuids::uuid projectId = boost::uuids::string_generator()(responseForProjectResponseCreate.get_data()->get_id());
   
    // List projects
    ResponseForProjectsResponse responseForProjectResponseList = bitwardenClient.listProjects(organizationId);

    // Get project details
    ResponseForProjectResponse responseForProjectResponseGet = bitwardenClient.getProject(projectId);

    // Update project
    ResponseForProjectResponse responseForProjectResponseUpdate = bitwardenClient.updateProject(projectId, organizationId, "NewTestProject2");

    // Secrets
    std::string key = "key";
    std::string value = "value";
    std::string note = "note";

    // Create a new secret
    ResponseForSecretResponse responseForSecretResponseCreate = bitwardenClient.createSecret(key, value, note, organizationId, {projectId});
    boost::uuids::uuid secretId = boost::uuids::string_generator()(responseForSecretResponseCreate.get_data()->get_id());

    // List secrets
    ResponseForSecretIdentifiersResponse responseForSecretIdentifiersResponse = bitwardenClient.listSecrets(organizationId);

    // Get secret details
    ResponseForSecretResponse responseForSecretResponseGet = bitwardenClient.getSecret(secretId);

    // Update secret
    key = "key2";
    value = "value2";
    note = "note2";
    ResponseForSecretResponse responseForSecretResponseUpdate = bitwardenClient.updateSecret(secretId, key, value, note, organizationId, {projectId});

    // Delete secrets
    ResponseForSecretsDeleteResponse responseForSecretsDeleteResponse = bitwardenClient.deleteSecrets({secretId});

    // Delete projects
    ResponseForProjectsDeleteResponse responseForProjectsDeleteResponse = bitwardenClient.deleteProjects({projectId});

    return 0;
}
