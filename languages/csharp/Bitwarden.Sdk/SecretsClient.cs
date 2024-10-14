﻿namespace Bitwarden.Sdk;

public class SecretsClient
{
    private readonly CommandRunner _commandRunner;

    internal SecretsClient(CommandRunner commandRunner)
    {
        _commandRunner = commandRunner;
    }

    public SecretResponse Get(Guid id)
    {
        var command = new Command { Secrets = new SecretsCommand { Get = new SecretGetRequest { Id = id } } };
        var result = _commandRunner.RunCommand<ResponseForSecretResponse>(command);

        if (result is { Success: true })
        {
            return result.Data;
        }

        throw new BitwardenException(result != null ? result.ErrorMessage : "Secret not found");
    }

    public SecretsResponse GetByIds(Guid[] ids)
    {
        var command = new Command { Secrets = new SecretsCommand { GetByIds = new SecretsGetRequest { Ids = ids } } };
        var result = _commandRunner.RunCommand<ResponseForSecretsResponse>(command);

        if (result is { Success: true })
        {
            return result.Data;
        }

        throw new BitwardenException(result != null ? result.ErrorMessage : "Secret not found");
    }

    public SecretResponse Create(Guid organizationId, string key, string value, string note, Guid[] projectIds)
    {
        var command = new Command
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

        var result = _commandRunner.RunCommand<ResponseForSecretResponse>(command);

        if (result is { Success: true })
        {
            return result.Data;
        }

        throw new BitwardenException(result != null ? result.ErrorMessage : "Secret create failed");
    }

    public SecretResponse Update(Guid organizationId, Guid id, string key, string value, string note, Guid[] projectIds)
    {
        var command = new Command
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

        var result = _commandRunner.RunCommand<ResponseForSecretResponse>(command);

        if (result is { Success: true })
        {
            return result.Data;
        }

        throw new BitwardenException(result != null ? result.ErrorMessage : "Secret update failed");
    }

    public SecretsDeleteResponse Delete(Guid[] ids)
    {
        var command = new Command { Secrets = new SecretsCommand { Delete = new SecretsDeleteRequest { Ids = ids } } };
        var result = _commandRunner.RunCommand<ResponseForSecretsDeleteResponse>(command);

        if (result is { Success: true })
        {
            return result.Data;
        }

        throw new BitwardenException(result != null ? result.ErrorMessage : "Secrets delete failed");
    }

    public SecretIdentifiersResponse List(Guid organizationId)
    {
        var command = new Command
        {
            Secrets = new SecretsCommand { List = new SecretIdentifiersRequest { OrganizationId = organizationId } }
        };
        var result = _commandRunner.RunCommand<ResponseForSecretIdentifiersResponse>(command);

        if (result is { Success: true })
        {
            return result.Data;
        }

        throw new BitwardenException(result != null ? result.ErrorMessage : "No secrets for given organization");
    }

    public SecretsSyncResponse Sync(Guid organizationId, DateTimeOffset? lastSyncedDate)
    {
        var command = new Command
        {
            Secrets = new SecretsCommand
            {
                Sync = new SecretsSyncRequest
                {
                    OrganizationId = organizationId,
                    LastSyncedDate = lastSyncedDate
                }
            }
        };

        var result = _commandRunner.RunCommand<ResponseForSecretsSyncResponse>(command);

        if (result is { Success: true })
        {
            return result.Data;
        }

        throw new BitwardenException(result != null ? result.ErrorMessage : "Secret update failed");
    }
}
