namespace Bitwarden.Sdk;

public class ProjectsClient
{
    private readonly CommandRunner _commandRunner;

    internal ProjectsClient(CommandRunner commandRunner) => _commandRunner = commandRunner;

    public ProjectResponse Get(Guid id)
    {
        var command = new Command { Projects = new ProjectsCommand { Get = new ProjectGetRequest { Id = id } } };
        var result = _commandRunner.RunCommand<ResponseForProjectResponse>(command);

        if (result is { Success: true })
        {
            return result.Data;
        }

        throw new BitwardenException(result != null ? result.ErrorMessage : "Project not found");
    }

    public ProjectResponse Create(Guid organizationId, string name)
    {
        var command = new Command
        {
            Projects = new ProjectsCommand
            {
                Create = new ProjectCreateRequest { OrganizationId = organizationId, Name = name }
            }
        };
        var result = _commandRunner.RunCommand<ResponseForProjectResponse>(command);

        if (result is { Success: true })
        {
            return result.Data;
        }

        throw new BitwardenException(result != null ? result.ErrorMessage : "Project create failed");
    }

    public ProjectResponse Update(Guid id, Guid organizationId, string name)
    {
        var command = new Command
        {
            Projects = new ProjectsCommand
            {
                Update = new ProjectPutRequest { Id = id, OrganizationId = organizationId, Name = name }
            }
        };
        var result = _commandRunner.RunCommand<ResponseForProjectResponse>(command);

        if (result is { Success: true })
        {
            return result.Data;
        }

        throw new BitwardenException(result != null ? result.ErrorMessage : "Project update failed");
    }

    public ProjectsDeleteResponse Delete(Guid[] ids)
    {
        var command = new Command
        {
            Projects = new ProjectsCommand { Delete = new ProjectsDeleteRequest { Ids = ids } }
        };
        var result = _commandRunner.RunCommand<ResponseForProjectsDeleteResponse>(command);

        if (result is { Success: true })
        {
            return result.Data;
        }

        throw new BitwardenException(result != null ? result.ErrorMessage : "Project delete failed");
    }

    public ProjectsResponse List(Guid organizationId)
    {
        var command = new Command
        {
            Projects = new ProjectsCommand { List = new ProjectsListRequest { OrganizationId = organizationId } }
        };
        var result = _commandRunner.RunCommand<ResponseForProjectsResponse>(command);

        if (result is { Success: true })
        {
            return result.Data;
        }

        throw new BitwardenException(result != null ? result.ErrorMessage : "No projects for given organization");
    }
}
