#pragma once

#include <vector>
#include <boost/uuid/uuid.hpp>
#include "CommandRunner.h"

using namespace quicktype;

class Projects {
public:
    Projects(CommandRunner* commandRunner);

    ResponseForProjectResponse get(const boost::uuids::uuid& id);
    ResponseForProjectResponse create(const boost::uuids::uuid& organizationId, const std::string& name);
    ResponseForProjectResponse update(const boost::uuids::uuid& id, const boost::uuids::uuid& organizationId, const std::string& name);
    ResponseForProjectsDeleteResponse deleteProjects(const std::vector<boost::uuids::uuid>& ids);
    ResponseForProjectsResponse list(const boost::uuids::uuid& organizationId);

private:
    CommandRunner* commandRunner;
};
