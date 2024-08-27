#include "BitwardenClient.h"
#include <iostream>
#include <string>

BitwardenClient::BitwardenClient(const BitwardenSettings& bitwardenSettings)
    : library(nullptr), commandRunner(nullptr), isClientOpen(false), projects(nullptr), secrets(nullptr) {

    // Set default values for optional strings
    boost::optional<std::string> apiUrl = bitwardenSettings.get_api_url().empty()
        ? boost::optional<std::string>("https://api.bitwarden.com")
        : boost::optional<std::string>(bitwardenSettings.get_api_url());

    boost::optional<std::string> identityUrl = bitwardenSettings.get_identity_url().empty()
        ? boost::optional<std::string>("https://identity.bitwarden.com")
        : boost::optional<std::string>(bitwardenSettings.get_identity_url());

    boost::optional<std::string> user_agent = boost::optional<std::string>("Bitwarden CPP-SDK");

    // Set values in clientSettings
    clientSettings.set_device_type(Bitwarden::Sdk::DeviceType::SDK);
    clientSettings.set_user_agent(user_agent);
    clientSettings.set_api_url(apiUrl);
    clientSettings.set_identity_url(identityUrl);

    nlohmann::json jsonClientSettings;
    Bitwarden::Sdk::to_json(jsonClientSettings, clientSettings);

    std::string jsonClientSettingsString = jsonClientSettings.dump();
    const char* jsonClientSettingsCStr = jsonClientSettingsString.c_str();

    try {
        library = new BitwardenLibrary("./");
        client = library->init(jsonClientSettingsCStr);
        commandRunner = new CommandRunner(library, client);
        projects = Projects(commandRunner);
        secrets = Secrets(commandRunner);
        isClientOpen = true;
    } catch (const std::exception& ex) {
        std::cerr << "Failed to initialize: " << ex.what() << std::endl;
        throw ex;
    }
}

BitwardenClient::~BitwardenClient() {
    if (library) {
        delete commandRunner;
        library->free_mem(client);
        delete library;
        isClientOpen = false;
    }
}

void BitwardenClient::loginAccessToken(const std::string& accessToken, const std::string& stateFile) {
    Command command;
    AccessTokenLoginRequest accessTokenLoginRequest;
    accessTokenLoginRequest.set_access_token(accessToken);
    command.set_login_access_token(accessTokenLoginRequest);

    auto deserializer = [](const char* response) -> ResponseForApiKeyLoginResponse {
        nlohmann::json jsonResponse = nlohmann::json::parse(response);
        ResponseForApiKeyLoginResponse loginResponse;
        Bitwarden::Sdk::from_json(jsonResponse, loginResponse);
        return loginResponse;
    };
    try {
        commandRunner->runCommand<ResponseForApiKeyLoginResponse, ApiKeyLoginResponse>(command, deserializer);
    } catch (const std::exception& ex) {
        std::cerr << "Error in accessTokenLogin: " << ex.what() << std::endl;
        throw ex;
    }
}

ProjectResponse BitwardenClient::getProject(const boost::uuids::uuid& id){
    if (!isClientOpen) {
        throw std::runtime_error("Client is not open.");
    }
    return projects.get(id);
}

ProjectResponse BitwardenClient::createProject(const boost::uuids::uuid& organizationId, const std::string& name){
    if (!isClientOpen) {
        throw std::runtime_error("Client is not open.");
    }
    return projects.create(organizationId, name);
}

ProjectResponse BitwardenClient::updateProject(const boost::uuids::uuid& id, const boost::uuids::uuid& organizationId, const std::string& name){
    if (!isClientOpen) {
        throw std::runtime_error("Client is not open.");
    }
    return projects.update(id, organizationId, name);
}

ProjectsDeleteResponse BitwardenClient::deleteProjects(const std::vector<boost::uuids::uuid>& ids) {
    if (!isClientOpen) {
        throw std::runtime_error("Client is not open.");
    }
    return projects.deleteProjects(ids);

}

ProjectsResponse BitwardenClient::listProjects(const boost::uuids::uuid &organizationId) {
    if (!isClientOpen) {
        throw std::runtime_error("Client is not open.");
    }
    return projects.list(organizationId);

}

SecretResponse BitwardenClient::getSecret(const boost::uuids::uuid& id){
    if (!isClientOpen) {
        throw std::runtime_error("Client is not open.");
    }
    return secrets.get(id);
}

SecretResponse BitwardenClient::createSecret(const std::string& key, const std::string& value, const std::string& note, const boost::uuids::uuid& organizationId, const std::vector<boost::uuids::uuid>& projectIds){
    if (!isClientOpen) {
        throw std::runtime_error("Client is not open.");
    }
    return secrets.create(key, value, note, organizationId, projectIds);
}

SecretResponse BitwardenClient::updateSecret(const boost::uuids::uuid& id, const std::string& key, const std::string& value, const std::string& note, const boost::uuids::uuid& organizationId, const std::vector<boost::uuids::uuid>& projectIds){
    if (!isClientOpen) {
        throw std::runtime_error("Client is not open.");
    }
    return secrets.update(id, key, value, note, organizationId, projectIds);
}

SecretsDeleteResponse BitwardenClient::deleteSecrets(const std::vector<boost::uuids::uuid>& ids) {
    if (!isClientOpen) {
        throw std::runtime_error("Client is not open.");
    }
    return secrets.deleteSecrets(ids);

}

SecretIdentifiersResponse BitwardenClient::listSecrets(const boost::uuids::uuid &organizationId) {
    if (!isClientOpen) {
        throw std::runtime_error("Client is not open.");
    }
    return secrets.list(organizationId);

}
