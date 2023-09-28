// #include "BitwardenLibrary.h"
#include "BitwardenClient.h"
// #include "CommandRunner.h"
// #include "Projects.h"
// #include <boost/uuid/uuid.hpp>
#include <boost/uuid/string_generator.hpp>
// #include <boost/uuid/uuid_io.hpp>
#include <iostream>

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
    ResponseForApiKeyLoginResponse responseForApiKeyLoginResponse = bitwardenClient.accessTokenLogin("0.89fc2c58-05d0-4c37-9597-b075010f6a7e.mZWsTbpmIbh839tTQ2vBiqBJwbE72Q:k2/auV2ry+lLXVpZccqFsg==");
    // Organization ID
    boost::uuids::uuid organizationId = boost::uuids::string_generator()("70bd2aa9-c92d-47b3-b873-b074005d92e0");    
    
    // // Create a new project
    ResponseForProjectResponse responseForProjectResponseCreate = bitwardenClient.createProject(organizationId, "NewTestProject");
    boost::uuids::uuid projectId = boost::uuids::string_generator()(responseForProjectResponseCreate.get_data()->get_id());
    // nlohmann::json projectJsonCreate;
    // quicktype::to_json(projectJsonCreate, responseForProjectResponseCreate);
    // std::string projectIdString = projectJsonCreate["data"]["id"];
    // boost::uuids::uuid projectId = boost::uuids::string_generator()(projectIdString);
    // std::string projectJsonStringCreate = projectJsonCreate.dump();
    // std::cout << "\nprojectJsonStringCreate:" << projectJsonStringCreate << std::endl;
   
   
    // List projects
    ResponseForProjectsResponse responseForProjectResponseList = bitwardenClient.listProjects(organizationId);
    nlohmann::json projectJsonList;
    quicktype::to_json(projectJsonList, responseForProjectResponseList);
    std::string projectJsonStringList = projectJsonList.dump();
    std::cout << "\nprojectJsonStringList:" << projectJsonStringList << std::endl;


    // Get project details
    ResponseForProjectResponse responseForProjectResponseGet = bitwardenClient.getProject(projectId);
    nlohmann::json projectJsonGet;
    quicktype::to_json(projectJsonGet, responseForProjectResponseGet);
    std::string projectJsonStringGet = projectJsonGet.dump();
    std::cout << "\nprojectJsonStringGET:" << projectJsonStringGet << std::endl;

    // Update project
    ResponseForProjectResponse responseForProjectResponseUpdate = bitwardenClient.updateProject(projectId, organizationId, "NewTestProject2");
    nlohmann::json projectJsonUpdate;
    quicktype::to_json(projectJsonUpdate, responseForProjectResponseUpdate);
    std::string projectJsonStringUpdate = projectJsonUpdate.dump();
    std::cout << "\nprojectJsonStringUpdate:" << projectJsonStringUpdate << std::endl;

    // Secrets
    std::string key = "key";
    std::string value = "value";
    std::string note = "note";

    // Create a new secret
    ResponseForSecretResponse responseForSecretResponseCreate = bitwardenClient.createSecret(key, value, note, organizationId, {projectId});
    boost::uuids::uuid secretId = boost::uuids::string_generator()(responseForSecretResponseCreate.get_data()->get_id());
    // nlohmann::json secretJsonCreate;
    // quicktype::to_json(secretJsonCreate, responseForSecretResponseCreate);
    // std::string secretIdString = secretJsonCreate["data"]["id"];
    // boost::uuids::uuid secretId = boost::uuids::string_generator()(secretIdString);
    // std::string secretJsonStringCreate = secretJsonCreate.dump();
    // std::cout << "\nsecretJsonStringCreate:" << secretJsonStringCreate << std::endl;

    // List secrets
    ResponseForSecretIdentifiersResponse responseForSecretIdentifiersResponse = bitwardenClient.listSecrets(organizationId);
    nlohmann::json secretJsonList;
    quicktype::to_json(secretJsonList, responseForSecretIdentifiersResponse);
    std::string secretJsonStringList = secretJsonList.dump();
    std::cout << "\nsecretJsonStringList:" << secretJsonStringList << std::endl;

    // Get secret details
    ResponseForSecretResponse responseForSecretResponseGet = bitwardenClient.getSecret(secretId);
    nlohmann::json secretJsonGet;
    quicktype::to_json(secretJsonGet, responseForSecretResponseGet);
    std::string secretJsonStringGet = secretJsonGet.dump();
    std::cout << "\nsecretJsonStringGet:" << secretJsonStringGet << std::endl;

    // Update secret
    key = "key2";
    value = "value2";
    note = "note2";
    ResponseForSecretResponse responseForSecretResponseUpdate = bitwardenClient.updateSecret(secretId, key, value, note, organizationId, {projectId});
    nlohmann::json secretJsonUpdate;
    quicktype::to_json(secretJsonUpdate, responseForSecretResponseUpdate);
    std::string secretJsonStringUpdate = secretJsonUpdate.dump();
    std::cout << "\nsecretJsonStringUpdate:" << secretJsonStringUpdate << std::endl;

    // Delete secrets
    ResponseForSecretsDeleteResponse responseForSecretsDeleteResponse = bitwardenClient.deleteSecrets({secretId});
    nlohmann::json secretJsonDelete;
    quicktype::to_json(secretJsonDelete, responseForSecretsDeleteResponse);
    std::string secretJsonStringDelete = secretJsonDelete.dump();
    std::cout << "\nsecretJsonStringDelete:" << secretJsonStringDelete << std::endl;

    // Delete projects
    ResponseForProjectsDeleteResponse responseForProjectsDeleteResponse = bitwardenClient.deleteProjects({projectId});
    nlohmann::json projectJsonDelete;
    quicktype::to_json(projectJsonDelete, responseForProjectsDeleteResponse);
    std::string projectJsonStringDelete = projectJsonDelete.dump();
    std::cout << "\nprojectJsonStringDelete:" << projectJsonStringDelete << std::endl;

    return 0;
}
