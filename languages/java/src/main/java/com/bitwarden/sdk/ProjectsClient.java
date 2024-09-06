package com.bitwarden.sdk;

import com.bitwarden.sdk.schema.*;

import java.util.UUID;

public class ProjectsClient {

    private final CommandRunner commandRunner;

    ProjectsClient(CommandRunner commandRunner) {
        this.commandRunner = commandRunner;
    }

    public ProjectResponse get(UUID id) {
        Command command = new Command();
        ProjectsCommand projectsCommand = new ProjectsCommand();
        ProjectGetRequest projectGetRequest = new ProjectGetRequest();
        projectGetRequest.setID(id);
        projectsCommand.setGet(projectGetRequest);
        command.setProjects(projectsCommand);

        ResponseForProjectResponse response = commandRunner.runCommand(command,
            BitwardenClient.throwingFunctionWrapper(Converter::ResponseForProjectResponseFromJsonString));

        if (response == null || !response.getSuccess()) {
            throw new BitwardenClientException(response != null ? response.getErrorMessage() : "Project not found");
        }

        return response.getData();
    }

    public ProjectResponse create(UUID organizationId, String name) {
        Command command = new Command();
        ProjectsCommand projectsCommand = new ProjectsCommand();
        ProjectCreateRequest projectCreateRequest = new ProjectCreateRequest();
        projectCreateRequest.setOrganizationID(organizationId);
        projectCreateRequest.setName(name);
        projectsCommand.setCreate(projectCreateRequest);
        command.setProjects(projectsCommand);

        ResponseForProjectResponse response = commandRunner.runCommand(command,
            BitwardenClient.throwingFunctionWrapper(Converter::ResponseForProjectResponseFromJsonString));

        if (response == null || !response.getSuccess()) {
            throw new BitwardenClientException(response != null ? response.getErrorMessage() : "Project create failed");
        }

        return response.getData();
    }

    public ProjectResponse update(UUID organizationId, UUID id, String name) {
        Command command = new Command();
        ProjectsCommand projectsCommand = new ProjectsCommand();
        ProjectPutRequest projectPutRequest = new ProjectPutRequest();
        projectPutRequest.setID(id);
        projectPutRequest.setOrganizationID(organizationId);
        projectPutRequest.setName(name);
        projectsCommand.setUpdate(projectPutRequest);
        command.setProjects(projectsCommand);

        ResponseForProjectResponse response = commandRunner.runCommand(command,
            BitwardenClient.throwingFunctionWrapper(Converter::ResponseForProjectResponseFromJsonString));

        if (response == null || !response.getSuccess()) {
            throw new BitwardenClientException(response != null ? response.getErrorMessage() : "Project update failed");
        }

        return response.getData();
    }

    public ProjectsDeleteResponse delete(UUID[] ids) {
        Command command = new Command();
        ProjectsCommand projectsCommand = new ProjectsCommand();
        ProjectsDeleteRequest projectsDeleteRequest = new ProjectsDeleteRequest();
        projectsDeleteRequest.setIDS(ids);
        projectsCommand.setDelete(projectsDeleteRequest);
        command.setProjects(projectsCommand);

        ResponseForProjectsDeleteResponse response = commandRunner.runCommand(command,
            BitwardenClient.throwingFunctionWrapper(Converter::ResponseForProjectsDeleteResponseFromJsonString));

        if (response == null || !response.getSuccess()) {
            throw new BitwardenClientException(response != null ?
                response.getErrorMessage() : "Projects update failed");
        }

        return response.getData();
    }

    public ProjectsResponse list(UUID organizationId) {
        Command command = new Command();
        ProjectsCommand projectsCommand = new ProjectsCommand();
        ProjectsListRequest projectsListRequest = new ProjectsListRequest();
        projectsListRequest.setOrganizationID(organizationId);
        projectsCommand.setList(projectsListRequest);
        command.setProjects(projectsCommand);

        ResponseForProjectsResponse response = commandRunner.runCommand(command,
            BitwardenClient.throwingFunctionWrapper(Converter::ResponseForProjectsResponseFromJsonString));

        if (response == null || !response.getSuccess()) {
            throw new BitwardenClientException(response != null ?
                response.getErrorMessage() : "No projects for given organization");
        }

        return response.getData();
    }
}
