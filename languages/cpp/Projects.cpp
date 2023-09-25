#include "Projects.h"
#include <vector>
#include <boost/uuid/uuid.hpp>
#include <boost/uuid/string_generator.hpp>
#include <boost/uuid/uuid_io.hpp>
#include <nlohmann/json.hpp>

Projects::Projects(CommandRunner* commandRunner) : commandRunner(commandRunner) {}

auto projectsDeserializer = [](const char* response) -> ResponseForProjectResponse {
    nlohmann::json jsonResponse = nlohmann::json::parse(response);
    ResponseForProjectResponse projectResponse;
    quicktype::from_json(jsonResponse, projectResponse);
    return projectResponse;
};

auto deleteProjectsDeserializer = [](const char* response) -> ResponseForProjectsDeleteResponse {
    nlohmann::json jsonResponse = nlohmann::json::parse(response);
    ResponseForProjectsDeleteResponse deleteProjectsResponse;
    quicktype::from_json(jsonResponse, deleteProjectsResponse);
    return deleteProjectsResponse;
};

auto listDeserializer = [](const char* response) -> ResponseForProjectsResponse {
    nlohmann::json jsonResponse = nlohmann::json::parse(response);
    ResponseForProjectsResponse listResponse;
    quicktype::from_json(jsonResponse, listResponse);
    return listResponse;
};

ResponseForProjectResponse Projects::get(const boost::uuids::uuid& id) {
    Command command;
    ProjectsCommand projectsCommand;
    ProjectGetRequest projectGetRequest;

    std::string idStr = boost::uuids::to_string(id);
    projectGetRequest.set_id(idStr);

    projectsCommand.set_get(projectGetRequest);
    command.set_projects(projectsCommand);

    return commandRunner->runCommand<ResponseForProjectResponse>(command, projectsDeserializer);
}

ResponseForProjectResponse Projects::create(const boost::uuids::uuid& organizationId, const std::string& name) {
    Command command;
    ProjectsCommand projectsCommand;
    ProjectCreateRequest projectCreateRequest;

    std::string orgIdStr = boost::uuids::to_string(organizationId);
    projectCreateRequest.set_organization_id(orgIdStr);

    projectCreateRequest.set_name(name);
    projectsCommand.set_create(projectCreateRequest);
    command.set_projects(projectsCommand);

    try {
        return commandRunner->runCommand<ResponseForProjectResponse>(command, projectsDeserializer);
    } catch (const std::exception& ex) {
        throw ex;
    }
}

ResponseForProjectResponse Projects::update(const boost::uuids::uuid& id, const boost::uuids::uuid& organizationId, const std::string& name) {
    Command command;
    ProjectsCommand projectsCommand;
    ProjectPutRequest projectPutRequest;

    std::string idStr = boost::uuids::to_string(id);
    projectPutRequest.set_id(idStr);

    std::string orgIdStr = boost::uuids::to_string(organizationId);
    projectPutRequest.set_organization_id(orgIdStr);

    projectPutRequest.set_name(name);
    projectsCommand.set_update(projectPutRequest);
    command.set_projects(projectsCommand);

    try {
        return commandRunner->runCommand<ResponseForProjectResponse>(command, projectsDeserializer);
    } catch (const std::exception& ex) {
        throw ex;
    }
}

ResponseForProjectsDeleteResponse Projects::deleteProjects(const std::vector<boost::uuids::uuid>& ids) {
    Command command;
    ProjectsCommand projectsCommand;
    ProjectsDeleteRequest projectsDeleteRequest;

    std::vector<std::string> idStrs;
    for (const auto& id : ids) {
        idStrs.push_back(boost::uuids::to_string(id));
    }
    projectsDeleteRequest.set_ids(idStrs);

    projectsCommand.set_projects_command_delete(projectsDeleteRequest);
    command.set_projects(projectsCommand);

    try {
        return commandRunner->runCommand<ResponseForProjectsDeleteResponse>(command, deleteProjectsDeserializer);
    } catch (const std::exception& ex) {
        throw ex;
    }
}

ResponseForProjectsResponse Projects::list(const boost::uuids::uuid& organizationId) {
    Command command;
    ProjectsCommand projectsCommand;
    ProjectsListRequest projectsListRequest;

    std::string orgIdStr = boost::uuids::to_string(organizationId);
    projectsListRequest.set_organization_id(orgIdStr);

    projectsCommand.set_list(projectsListRequest);
    command.set_projects(projectsCommand);

    try {
        return commandRunner->runCommand<ResponseForProjectsResponse>(command, listDeserializer);
    } catch (const std::exception& ex) {
        throw ex;
    }
}
