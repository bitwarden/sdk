#pragma once

#include <vector>
#include <chrono>
#include <boost/optional.hpp>
#include <boost/uuid/uuid.hpp>
#include "CommandRunner.h"

class Secrets {
public:
    Secrets(CommandRunner* commandRunner);

    SecretResponse get(const boost::uuids::uuid& id);
    SecretsResponse getByIds(const std::vector<boost::uuids::uuid> &ids);
    SecretResponse create(const boost::uuids::uuid& organizationId, const std::string& key, const std::string& value, const std::string& note, const std::vector<boost::uuids::uuid>& projectIds);
    SecretResponse update(const boost::uuids::uuid& organizationId, const boost::uuids::uuid& id, const std::string& key, const std::string& value, const std::string& note, const std::vector<boost::uuids::uuid>& projectIds);
    SecretsDeleteResponse deleteSecrets(const std::vector<boost::uuids::uuid>& ids);
    SecretIdentifiersResponse list(const boost::uuids::uuid& organizationId);
    SecretsSyncResponse sync(const boost::uuids::uuid& organizationId, const boost::optional<std::chrono::system_clock::time_point>& lastSyncedDate);

private:
    CommandRunner* commandRunner;
};

