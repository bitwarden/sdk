#pragma once

#include <vector>
#include <boost/uuid/uuid.hpp>
#include "CommandRunner.h"

class CommandRunner; // Forward declaration

class Secrets {
public:
    Secrets(CommandRunner* commandRunner);

    ResponseForSecretResponse get(const boost::uuids::uuid& id);
    ResponseForSecretResponse create(const std::string& key, const std::string& value, const std::string& note, const boost::uuids::uuid& organizationId, const std::vector<boost::uuids::uuid>& projectIds);
    ResponseForSecretResponse update(const boost::uuids::uuid& id, const std::string& key, const std::string& value, const std::string& note, const boost::uuids::uuid& organizationId, const std::vector<boost::uuids::uuid>& projectIds);
    ResponseForSecretsDeleteResponse deleteSecrets(const std::vector<boost::uuids::uuid>& ids);
    ResponseForSecretIdentifiersResponse list(const boost::uuids::uuid& organizationId);

private:
    CommandRunner* commandRunner;
};

