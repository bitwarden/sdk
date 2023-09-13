namespace Bitwarden.Sdk;

public class SecretsClient
{
    private readonly CommandRunner _commandRunner;

    internal SecretsClient(CommandRunner commandRunner)
    {
        _commandRunner = commandRunner;
    }

    public ResponseForSecretResponse? Get(Guid id)
    {
        var command = new Command() { Secrets = new SecretsCommand { Get = new SecretGetRequest { Id = id } } };
        return _commandRunner.RunCommand<ResponseForSecretResponse>(command);
    }

    public ResponseForSecretResponse? Create(string key, string value, string note, Guid organizationId,
        Guid[] projectIds)
    {
        var command = new Command()
        {
            Secrets = new SecretsCommand
            {
                Create = new SecretCreateRequest
                {
                    Key = key,
                    Value = value,
                    Note = note,
                    OrganizationId = organizationId,
                    ProjectIds = projectIds
                }
            }
        };

        return _commandRunner.RunCommand<ResponseForSecretResponse>(command);
    }

    public ResponseForSecretResponse? Update(Guid id, string key, string value, string note, Guid organizationId,
        Guid[] projectIds)
    {
        var command = new Command()
        {
            Secrets = new SecretsCommand
            {
                Update = new SecretPutRequest
                {
                    Id = id,
                    Key = key,
                    Value = value,
                    Note = note,
                    OrganizationId = organizationId,
                    ProjectIds = projectIds
                }
            }
        };

        return _commandRunner.RunCommand<ResponseForSecretResponse>(command);
    }

    public ResponseForSecretsDeleteResponse? Delete(Guid[] ids)
    {
        var command = new Command()
        {
            Secrets = new SecretsCommand { Delete = new SecretsDeleteRequest { Ids = ids } }
        };
        return _commandRunner.RunCommand<ResponseForSecretsDeleteResponse>(command);
    }

    public ResponseForSecretIdentifiersResponse? List(Guid organizationId)
    {
        var command = new Command()
        {
            Secrets = new SecretsCommand { List = new SecretIdentifiersRequest { OrganizationId = organizationId } }
        };
        return _commandRunner.RunCommand<ResponseForSecretIdentifiersResponse>(command);
    }
}
