#pragma once

#include "CommandRunner.h"
#include "Projects.h"
#include "Secrets.h"
#include <functional>
#include <string>

class BitwardenClient {
public:
    BitwardenClient(ClientSettings clientSettings);
    ~BitwardenClient();
    
    ResponseForApiKeyLoginResponse accessTokenLogin(const std::string& accessToken);
    ResponseForProjectResponse getProject(const boost::uuids::uuid& id);
    ResponseForProjectResponse createProject(const boost::uuids::uuid& organizationId, const std::string& name);
    ResponseForProjectResponse updateProject(const boost::uuids::uuid& id, const boost::uuids::uuid& organizationId, const std::string& name);
    ResponseForProjectsDeleteResponse deleteProjects(const std::vector<boost::uuids::uuid>& ids);
    ResponseForProjectsResponse listProjects(const boost::uuids::uuid &organizationId);
    ResponseForSecretResponse getSecret(const boost::uuids::uuid& id);
    ResponseForSecretResponse createSecret(const std::string& key, const std::string& value, const std::string& note, const boost::uuids::uuid& organizationId, const std::vector<boost::uuids::uuid>& projectIds);
    ResponseForSecretResponse updateSecret(const boost::uuids::uuid& id, const std::string& key, const std::string& value, const std::string& note, const boost::uuids::uuid& organizationId, const std::vector<boost::uuids::uuid>& projectIds);
    ResponseForSecretsDeleteResponse deleteSecrets(const std::vector<boost::uuids::uuid>& ids);
    ResponseForSecretIdentifiersResponse listSecrets(const boost::uuids::uuid& organizationId);

private:
    BitwardenLibrary* library;
    void* client;
    CommandRunner* commandRunner;
    Projects projects;
    Secrets secrets;
    bool isClientOpen;
    ClientSettings clientSettings;

};
