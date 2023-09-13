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
        var command = new Command() { Projects = new ProjectsCommand { Get = new ProjectGetRequest { Id = id } } };
        return _commandRunner.RunCommand<ResponseForProjectResponse>(command);
    }

    public ResponseForProjectResponse? Create(Guid organizationId, string name)
    {
        var command = new Command()
        {
            Projects = new ProjectsCommand
            {
                Create = new ProjectCreateRequest { OrganizationId = organizationId, Name = name }
            }
        };
        return _commandRunner.RunCommand<ResponseForProjectResponse>(command);
    }

    public ResponseForProjectResponse? Update(Guid id, Guid organizationId, string name)
    {
        var command = new Command()
        {
            Projects = new ProjectsCommand
            {
                Update = new ProjectPutRequest { Id = id, OrganizationId = organizationId, Name = name }
            }
        };
        return _commandRunner.RunCommand<ResponseForProjectResponse>(command);
    }

    public ResponseForProjectsDeleteResponse? Delete(Guid[] ids)
    {
        var command = new Command()
        {
            Projects = new ProjectsCommand { Delete = new ProjectsDeleteRequest { Ids = ids } }
        };
        return _commandRunner.RunCommand<ResponseForProjectsDeleteResponse>(command);
    }

    public ResponseForProjectsResponse? List(Guid organizationId)
    {
        var command = new Command()
        {
            Projects = new ProjectsCommand { List = new ProjectsListRequest { OrganizationId = organizationId } }
        };
        return _commandRunner.RunCommand<ResponseForProjectsResponse>(command);
    }
}
