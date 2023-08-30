using Newtonsoft.Json;

namespace Bit.Sdk;


public class ProjectsClient
{

    private readonly CommandRunner _commandRunner;

    internal ProjectsClient(CommandRunner commandRunner)
    {
        this._commandRunner = commandRunner;
    }

    public ResponseForProjectResponse Get(Guid id)
    {
        var command = new Command();
        var projectsCommand = new ProjectsCommand();
        var projectGetRequest = new ProjectGetRequest
        {
            Id = id
        };
        projectsCommand.Get = projectGetRequest;
        command.Projects = projectsCommand;
        return _commandRunner.RunCommand(command, JsonConvert.DeserializeObject<ResponseForProjectResponse>);
    }

    public ResponseForProjectResponse Create(Guid organizationId, string name)
    {
        var command = new Command();
        var projectsCommand = new ProjectsCommand();
        var projectCreateRequest = new ProjectCreateRequest
        {
            OrganizationId = organizationId,
            Name = name
        };
        projectsCommand.Create = projectCreateRequest;
        command.Projects = projectsCommand;
        return _commandRunner.RunCommand(command, JsonConvert.DeserializeObject<ResponseForProjectResponse>);
    }

    public ResponseForProjectResponse Update(Guid id, Guid organizationId, String name)
    {
        var command = new Command();
        var projectsCommand = new ProjectsCommand();
        var projectPutRequest = new ProjectPutRequest
        {
            Id = id,
            OrganizationId = organizationId,
            Name = name
        };
        projectsCommand.Update = projectPutRequest;
        command.Projects = projectsCommand;
        return _commandRunner.RunCommand(command, JsonConvert.DeserializeObject<ResponseForProjectResponse>);
    }

    public ResponseForProjectsDeleteResponse Delete(Guid[] ids)
    {
        var command = new Command();
        var projectsCommand = new ProjectsCommand();
        var projectsDeleteRequest = new ProjectsDeleteRequest
        {
            Ids = ids
        };
        projectsCommand.Delete = projectsDeleteRequest;
        command.Projects = projectsCommand;
        return _commandRunner.RunCommand(command, JsonConvert.DeserializeObject<ResponseForProjectsDeleteResponse>);
    }

    public ResponseForProjectsResponse List(Guid organizationId)
    {
        var command = new Command();
        var projectsCommand = new ProjectsCommand();
        var projectsListRequest = new ProjectsListRequest();
        projectsListRequest.OrganizationId = organizationId;
        projectsCommand.List = projectsListRequest;
        command.Projects = projectsCommand;
        return _commandRunner.RunCommand(command, JsonConvert.DeserializeObject<ResponseForProjectsResponse>);
    }
}
