package bit.sdk;

import static bit.sdk.BitwardenClient.throwingFunctionWrapper;

import bit.sdk.schema.Command;
import bit.sdk.schema.Converter;
import bit.sdk.schema.ProjectCreateRequest;
import bit.sdk.schema.ProjectGetRequest;
import bit.sdk.schema.ProjectPutRequest;
import bit.sdk.schema.ProjectsCommand;
import bit.sdk.schema.ProjectsDeleteRequest;
import bit.sdk.schema.ProjectsListRequest;
import bit.sdk.schema.ResponseForProjectResponse;
import bit.sdk.schema.ResponseForProjectsDeleteResponse;
import bit.sdk.schema.ResponseForProjectsResponse;
import java.util.UUID;

public class Projects {

    private CommandRunner commandRunner;

    Projects(CommandRunner commandRunner) {
        this.commandRunner = commandRunner;
    }

    public ResponseForProjectResponse get(UUID id) {
        Command command = new Command();
        ProjectsCommand projectsCommand = new ProjectsCommand();
        ProjectGetRequest projectGetRequest = new ProjectGetRequest();
        projectGetRequest.setID(id);
        projectsCommand.setGet(projectGetRequest);
        command.setProjects(projectsCommand);
        return commandRunner.runCommand(command,
            throwingFunctionWrapper(Converter::ResponseForProjectResponseFromJsonString));
    }

    public ResponseForProjectResponse create(UUID organizationId, String name) {
        Command command = new Command();
        ProjectsCommand projectsCommand = new ProjectsCommand();
        ProjectCreateRequest projectCreateRequest = new ProjectCreateRequest();
        projectCreateRequest.setOrganizationID(organizationId);
        projectCreateRequest.setName(name);
        projectsCommand.setCreate(projectCreateRequest);
        command.setProjects(projectsCommand);
        return commandRunner.runCommand(command,
            throwingFunctionWrapper(Converter::ResponseForProjectResponseFromJsonString));
    }

    public ResponseForProjectResponse update(UUID id, UUID organizationId, String name) {
        Command command = new Command();
        ProjectsCommand projectsCommand = new ProjectsCommand();
        ProjectPutRequest projectPutRequest = new ProjectPutRequest();
        projectPutRequest.setID(id);
        projectPutRequest.setOrganizationID(organizationId);
        projectPutRequest.setName(name);
        projectsCommand.setUpdate(projectPutRequest);
        command.setProjects(projectsCommand);
        return commandRunner.runCommand(command,
            throwingFunctionWrapper(Converter::ResponseForProjectResponseFromJsonString));
    }

    public ResponseForProjectsDeleteResponse delete(UUID[] ids) {
        Command command = new Command();
        ProjectsCommand projectsCommand = new ProjectsCommand();
        ProjectsDeleteRequest projectsDeleteRequest = new ProjectsDeleteRequest();
        projectsDeleteRequest.setIDS(ids);
        projectsCommand.setDelete(projectsDeleteRequest);
        command.setProjects(projectsCommand);
        return commandRunner.runCommand(command,
            throwingFunctionWrapper(Converter::ResponseForProjectsDeleteResponseFromJsonString));
    }

    public ResponseForProjectsResponse list(UUID organizationId) {
        Command command = new Command();
        ProjectsCommand projectsCommand = new ProjectsCommand();
        ProjectsListRequest projectsListRequest = new ProjectsListRequest();
        projectsListRequest.setOrganizationID(organizationId);
        projectsCommand.setList(projectsListRequest);
        command.setProjects(projectsCommand);
        return commandRunner.runCommand(command,
            throwingFunctionWrapper(Converter::ResponseForProjectsResponseFromJsonString));
    }
}
