namespace Bitwarden.Sdk;


public class ProjectsClient
{

    private readonly CommandRunner _commandRunner;

    internal ProjectsClient(CommandRunner commandRunner)
    {
        _commandRunner = commandRunner;
    }

    public ResponseForProjectResponse? Get(Guid id)
    {
        var command = new Command();
        var projectsCommand = new ProjectsCommand();
        var projectGetRequest = new ProjectGetRequest
        {
            Id = id
        };
        projectsCommand.Get = projectGetRequest;
        command.Projects = projectsCommand;
        return _commandRunner.RunCommand<ResponseForProjectResponse>(command);
    }

    public ResponseForProjectResponse? Create(Guid organizationId, string name)
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
        return _commandRunner.RunCommand<ResponseForProjectResponse>(command);
    }

    public ResponseForProjectResponse? Update(Guid id, Guid organizationId, string name)
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
        return _commandRunner.RunCommand<ResponseForProjectResponse>(command);
    }

    public ResponseForProjectsDeleteResponse? Delete(Guid[] ids)
    {
        var command = new Command();
        var projectsCommand = new ProjectsCommand();
        var projectsDeleteRequest = new ProjectsDeleteRequest
        {
            Ids = ids
        };
        projectsCommand.Delete = projectsDeleteRequest;
        command.Projects = projectsCommand;
        return _commandRunner.RunCommand<ResponseForProjectsDeleteResponse>(command);
    }

    public ResponseForProjectsResponse? List(Guid organizationId)
    {
        var command = new Command();
        var projectsCommand = new ProjectsCommand();
        var projectsListRequest = new ProjectsListRequest { OrganizationId = organizationId };
        projectsCommand.List = projectsListRequest;
        command.Projects = projectsCommand;
        return _commandRunner.RunCommand<ResponseForProjectsResponse>(command);
    }
}
