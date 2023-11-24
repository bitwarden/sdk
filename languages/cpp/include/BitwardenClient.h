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
    
    void accessTokenLogin(const std::string& accessToken);
    ProjectResponse getProject(const boost::uuids::uuid& id);
    ProjectResponse createProject(const boost::uuids::uuid& organizationId, const std::string& name);
    ProjectResponse updateProject(const boost::uuids::uuid& id, const boost::uuids::uuid& organizationId, const std::string& name);
    ProjectsDeleteResponse deleteProjects(const std::vector<boost::uuids::uuid>& ids);
    ProjectsResponse listProjects(const boost::uuids::uuid &organizationId);
    SecretResponse getSecret(const boost::uuids::uuid& id);
    SecretResponse createSecret(const std::string& key, const std::string& value, const std::string& note, const boost::uuids::uuid& organizationId, const std::vector<boost::uuids::uuid>& projectIds);
    SecretResponse updateSecret(const boost::uuids::uuid& id, const std::string& key, const std::string& value, const std::string& note, const boost::uuids::uuid& organizationId, const std::vector<boost::uuids::uuid>& projectIds);
    SecretsDeleteResponse deleteSecrets(const std::vector<boost::uuids::uuid>& ids);
    SecretIdentifiersResponse listSecrets(const boost::uuids::uuid& organizationId);

private:
    BitwardenLibrary* library;
    void* client;
    CommandRunner* commandRunner;
    Projects projects;
    Secrets secrets;
    bool isClientOpen;
    ClientSettings clientSettings;

};
