#include "BitwardenClient.h"
#include <iostream>
#include <string>

BitwardenClient::BitwardenClient(ClientSettings clientSettings) : library(nullptr), commandRunner(nullptr), isClientOpen(false), projects(nullptr), secrets(nullptr) {

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

ResponseForApiKeyLoginResponse BitwardenClient::accessTokenLogin(const std::string& accessToken) {
    Command command;
    AccessTokenLoginRequest accessTokenLoginRequest;
    accessTokenLoginRequest.set_access_token(accessToken);
    command.set_access_token_login(accessTokenLoginRequest);

    auto deserializer = [](const char* response) -> ResponseForApiKeyLoginResponse {
        nlohmann::json jsonResponse = nlohmann::json::parse(response);
        ResponseForApiKeyLoginResponse loginResponse;
        Bitwarden::Sdk::from_json(jsonResponse, loginResponse);
        return loginResponse;
    };
    try {
        return commandRunner->runCommand<ResponseForApiKeyLoginResponse>(command, deserializer);
    } catch (const std::exception& ex) {
        throw ex;
    }
}

ResponseForProjectResponse BitwardenClient::getProject(const boost::uuids::uuid& id){
    if (!isClientOpen) {
        throw std::runtime_error("Client is not open.");
    }
    return projects.get(id);
}

ResponseForProjectResponse BitwardenClient::createProject(const boost::uuids::uuid& organizationId, const std::string& name){
    if (!isClientOpen) {
        throw std::runtime_error("Client is not open.");
    }
    return projects.create(organizationId, name);
}

ResponseForProjectResponse BitwardenClient::updateProject(const boost::uuids::uuid& id, const boost::uuids::uuid& organizationId, const std::string& name){
    if (!isClientOpen) {
        throw std::runtime_error("Client is not open.");
    }
    return projects.update(id, organizationId, name);
}

ResponseForProjectsDeleteResponse BitwardenClient::deleteProjects(const std::vector<boost::uuids::uuid>& ids) {
    if (!isClientOpen) {
        throw std::runtime_error("Client is not open.");
    }
    return projects.deleteProjects(ids);

}

ResponseForProjectsResponse BitwardenClient::listProjects(const boost::uuids::uuid &organizationId) {
    if (!isClientOpen) {
        throw std::runtime_error("Client is not open.");
    }
    return projects.list(organizationId);

}

ResponseForSecretResponse BitwardenClient::getSecret(const boost::uuids::uuid& id){
    if (!isClientOpen) {
        throw std::runtime_error("Client is not open.");
    }
    return secrets.get(id);
}

ResponseForSecretResponse BitwardenClient::createSecret(const std::string& key, const std::string& value, const std::string& note, const boost::uuids::uuid& organizationId, const std::vector<boost::uuids::uuid>& projectIds){
    if (!isClientOpen) {
        throw std::runtime_error("Client is not open.");
    }
    return secrets.create(key, value, note, organizationId, projectIds);
}

ResponseForSecretResponse BitwardenClient::updateSecret(const boost::uuids::uuid& id, const std::string& key, const std::string& value, const std::string& note, const boost::uuids::uuid& organizationId, const std::vector<boost::uuids::uuid>& projectIds){
    if (!isClientOpen) {
        throw std::runtime_error("Client is not open.");
    }
    return secrets.update(id, key, value, note, organizationId, projectIds);
}

ResponseForSecretsDeleteResponse BitwardenClient::deleteSecrets(const std::vector<boost::uuids::uuid>& ids) {
    if (!isClientOpen) {
        throw std::runtime_error("Client is not open.");
    }
    return secrets.deleteSecrets(ids);

}

ResponseForSecretIdentifiersResponse BitwardenClient::listSecrets(const boost::uuids::uuid &organizationId) {
    if (!isClientOpen) {
        throw std::runtime_error("Client is not open.");
    }
    return secrets.list(organizationId);

}
