#include "Secrets.h"
#include <nlohmann/json.hpp>
#include <boost/uuid/uuid_io.hpp>
#include <boost/uuid/string_generator.hpp> 
#include <boost/uuid/uuid_io.hpp>

Secrets::Secrets(CommandRunner* commandRunner) : commandRunner(commandRunner) {}

auto secretsDeserializer = [](const std::string& response) -> ResponseForSecretResponse {
    nlohmann::json jsonResponse = nlohmann::json::parse(response);
    ResponseForSecretResponse secretResponse;
    Bitwarden::Sdk::from_json(jsonResponse, secretResponse);
    return secretResponse;
};

auto deleteSecretsDeserializer = [](const std::string& response) -> ResponseForSecretsDeleteResponse {
    nlohmann::json jsonResponse = nlohmann::json::parse(response);
    ResponseForSecretsDeleteResponse deleteSecretsResponse;
    Bitwarden::Sdk::from_json(jsonResponse, deleteSecretsResponse);
    return deleteSecretsResponse;
};

auto secretListDeserializer = [](const std::string& response) -> ResponseForSecretIdentifiersResponse {
    nlohmann::json jsonResponse = nlohmann::json::parse(response);
    ResponseForSecretIdentifiersResponse listResponse;
    Bitwarden::Sdk::from_json(jsonResponse, listResponse);
    return listResponse;
};

ResponseForSecretResponse Secrets::get(const boost::uuids::uuid& id) {
    Command command;
    SecretsCommand secretsCommand;
    SecretGetRequest secretGetRequest;

    std::string idStr = boost::uuids::to_string(id);
    secretGetRequest.set_id(idStr);

    secretsCommand.set_get(secretGetRequest);
    command.set_secrets(secretsCommand);

    try {
        return commandRunner->runCommand<ResponseForSecretResponse>(command, secretsDeserializer);
    } catch (const std::exception& ex) {
        throw ex;
    }
}

ResponseForSecretResponse Secrets::create(const std::string& key, const std::string& value, const std::string& note, const boost::uuids::uuid& organizationId, const std::vector<boost::uuids::uuid>& projectIds) {
    Command command;
    SecretsCommand secretsCommand;
    SecretCreateRequest secretCreateRequest;

    std::string orgIdStr = boost::uuids::to_string(organizationId);
    secretCreateRequest.set_organization_id(orgIdStr);

    secretCreateRequest.set_key(key);
    secretCreateRequest.set_value(value);
    secretCreateRequest.set_note(note);

    std::vector<std::string> projectIdsStr;
    for (const auto& projectId : projectIds) {
        projectIdsStr.push_back(boost::uuids::to_string(projectId));
    }
    secretCreateRequest.set_project_ids(projectIdsStr);

    secretsCommand.set_create(secretCreateRequest);
    command.set_secrets(secretsCommand);

    try {
        return commandRunner->runCommand<ResponseForSecretResponse>(command, secretsDeserializer);
    } catch (const std::exception& ex) {
        throw ex;
    }
}

ResponseForSecretResponse Secrets::update(const boost::uuids::uuid& id, const std::string& key, const std::string& value, const std::string& note, const boost::uuids::uuid& organizationId, const std::vector<boost::uuids::uuid>& projectIds) {
    Command command;
    SecretsCommand secretsCommand;
    SecretPutRequest secretPutRequest;

    std::string idStr = boost::uuids::to_string(id);
    secretPutRequest.set_id(idStr);

    std::string orgIdStr = boost::uuids::to_string(organizationId);
    secretPutRequest.set_organization_id(orgIdStr);

    secretPutRequest.set_key(key);
    secretPutRequest.set_value(value);
    secretPutRequest.set_note(note);

    std::vector<std::string> projectIdsStr;
    for (const auto& projectId : projectIds) {
        projectIdsStr.push_back(boost::uuids::to_string(projectId));
    }
    secretPutRequest.set_project_ids(projectIdsStr);

    secretsCommand.set_update(secretPutRequest);
    command.set_secrets(secretsCommand);

    try {
        return commandRunner->runCommand<ResponseForSecretResponse>(command, secretsDeserializer);
    } catch (const std::exception& ex) {
        throw ex;
    }
}

ResponseForSecretsDeleteResponse Secrets::deleteSecrets(const std::vector<boost::uuids::uuid>& ids) {
    Command command;
    SecretsCommand secretsCommand;
    SecretsDeleteRequest secretsDeleteRequest;

    std::vector<std::string> idsStr;
    for (const auto& id : ids) {
        idsStr.push_back(boost::uuids::to_string(id));
    }
    secretsDeleteRequest.set_ids(idsStr);

    secretsCommand.set_secrets_command_delete(secretsDeleteRequest);
    command.set_secrets(secretsCommand);

    try {
        return commandRunner->runCommand<ResponseForSecretsDeleteResponse>(command, deleteSecretsDeserializer);
    } catch (const std::exception& ex) {
        throw ex;
    }
}

ResponseForSecretIdentifiersResponse Secrets::list(const boost::uuids::uuid& organizationId) {
    Command command;
    SecretsCommand secretsCommand;
    SecretIdentifiersRequest secretIdentifiersRequest;

    std::string orgIdStr = boost::uuids::to_string(organizationId);
    secretIdentifiersRequest.set_organization_id(orgIdStr);

    secretsCommand.set_list(secretIdentifiersRequest);
    command.set_secrets(secretsCommand);

    try {
        return commandRunner->runCommand<ResponseForSecretIdentifiersResponse>(command, secretListDeserializer);
    } catch (const std::exception& ex) {
        throw ex;
    }
}
