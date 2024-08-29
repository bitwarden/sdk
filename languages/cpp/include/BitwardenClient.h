#pragma once

#include "CommandRunner.h"
#include "BitwardenSettings.h"
#include "Projects.h"
#include "Secrets.h"
#include <functional>
#include <string>

class BitwardenClient {
public:
    explicit BitwardenClient(const BitwardenSettings& bitwardenSettings = BitwardenSettings());
    ~BitwardenClient();

    void loginAccessToken(const std::string& accessToken, const std::string& stateFile = "");
    ProjectResponse getProject(const boost::uuids::uuid& id);
    ProjectResponse createProject(const boost::uuids::uuid& organizationId, const std::string& name);
    ProjectResponse updateProject(const boost::uuids::uuid& organizationId, const boost::uuids::uuid& id, const std::string& name);
    ProjectsDeleteResponse deleteProjects(const std::vector<boost::uuids::uuid>& ids);
    ProjectsResponse listProjects(const boost::uuids::uuid &organizationId);
    SecretResponse getSecret(const boost::uuids::uuid& id);
    SecretsResponse getSecretsByIds(const std::vector<boost::uuids::uuid>& ids);
    SecretResponse createSecret(const boost::uuids::uuid& organizationId, const std::string& key, const std::string& value, const std::string& note, const std::vector<boost::uuids::uuid>& projectIds);
    SecretResponse updateSecret(const boost::uuids::uuid& organizationId, const boost::uuids::uuid& id, const std::string& key, const std::string& value, const std::string& note, const std::vector<boost::uuids::uuid>& projectIds);
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
