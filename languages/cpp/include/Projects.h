#pragma once

#include <vector>
#include <boost/uuid/uuid.hpp>
#include "CommandRunner.h"

class Projects {
public:
    Projects(CommandRunner* commandRunner);

    ProjectResponse get(const boost::uuids::uuid& id);
    ProjectResponse create(const boost::uuids::uuid& organizationId, const std::string& name);
    ProjectResponse update(const boost::uuids::uuid& id, const boost::uuids::uuid& organizationId, const std::string& name);
    ProjectsDeleteResponse deleteProjects(const std::vector<boost::uuids::uuid>& ids);
    ProjectsResponse list(const boost::uuids::uuid& organizationId);

private:
    CommandRunner* commandRunner;
};
