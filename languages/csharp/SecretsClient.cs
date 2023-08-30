using Newtonsoft.Json;

namespace Bit.Sdk;


public class SecretsClient
{

    private readonly CommandRunner _commandRunner;

    internal SecretsClient(CommandRunner commandRunner)
    {
        _commandRunner = commandRunner;
    }

    public ResponseForSecretResponse Get(Guid id)
    {
        var command = new Command();
        var secretsCommand = new SecretsCommand();
        var secretGetRequest = new SecretGetRequest
        {
            Id = id
        };
        secretsCommand.Get = secretGetRequest;
        command.Secrets = secretsCommand;
        return _commandRunner.RunCommand(command, JsonConvert.DeserializeObject<ResponseForSecretResponse>);
    }

    public ResponseForSecretResponse Create(string key, string value, string note, Guid organizationId,
        Guid[] projectIds)
    {
        var command = new Command();
        var secretsCommand = new SecretsCommand();
        var secretCreateRequest = new SecretCreateRequest
        {
            Key = key,
            Value = value,
            Note = note,
            OrganizationId = organizationId,
            ProjectIds = projectIds
        };
        secretsCommand.Create = secretCreateRequest;
        command.Secrets = secretsCommand;
        return _commandRunner.RunCommand(command, JsonConvert.DeserializeObject<ResponseForSecretResponse>);
    }

    public ResponseForSecretResponse Update(Guid id, string key, string value, string note, Guid organizationId,
        Guid[] projectIds)
    {
        var command = new Command();
        var secretsCommand = new SecretsCommand();
        var secretPutRequest = new SecretPutRequest
        {
            Id = id,
            Key = key,
            Value = value,
            Note = note,
            OrganizationId = organizationId,
            ProjectIds = projectIds
        };
        secretsCommand.Update = secretPutRequest;
        command.Secrets = secretsCommand;
        return _commandRunner.RunCommand(command, JsonConvert.DeserializeObject<ResponseForSecretResponse>);
    }

    public ResponseForSecretsDeleteResponse Delete(Guid[] ids)
    {
        var command = new Command();
        var secretsCommand = new SecretsCommand();
        var secretsDeleteRequest = new SecretsDeleteRequest
        {
            Ids = ids
        };
        secretsCommand.Delete = secretsDeleteRequest;
        command.Secrets = secretsCommand;
        return _commandRunner.RunCommand(command, JsonConvert.DeserializeObject<ResponseForSecretsDeleteResponse>);
    }

    public ResponseForSecretIdentifiersResponse List(Guid organizationId)
    {
        var command = new Command();
        var secretsCommand = new SecretsCommand();
        var secretIdentifiersRequest = new SecretIdentifiersRequest
        {
            OrganizationId = organizationId
        };
        secretsCommand.List = secretIdentifiersRequest;
        command.Secrets = secretsCommand;
        return _commandRunner.RunCommand(command, JsonConvert.DeserializeObject<ResponseForSecretIdentifiersResponse>);
    }
}
