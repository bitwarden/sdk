namespace Bitwarden.Sdk;

public class SecretsClient
{
    private readonly CommandRunner _commandRunner;

    internal SecretsClient(CommandRunner commandRunner)
    {
        _commandRunner = commandRunner;
    }

    public async Task<SecretResponse> GetAsync(Guid id)
    {
        var command = new Command { Secrets = new SecretsCommand { Get = new SecretGetRequest { Id = id } } };
        var result = await _commandRunner.RunCommandAsync<ResponseForSecretResponse>(command);

        if (result is { Success: true })
        {
            return result.Data;
        }

        throw new BitwardenException(result != null ? result.ErrorMessage : "Secret not found");
    }

    public async Task<SecretsResponse> GetByIdsAsync(Guid[] ids)
    {
        var command = new Command { Secrets = new SecretsCommand { GetByIds = new SecretsGetRequest { Ids = ids } } };
        var result = await _commandRunner.RunCommandAsync<ResponseForSecretsResponse>(command);

        if (result is { Success: true })
        {
            return result.Data;
        }

        throw new BitwardenException(result != null ? result.ErrorMessage : "Secret not found");
    }

    public async Task<SecretResponse> CreateAsync(Guid organizationId, string key, string value, string note, Guid[] projectIds)
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

        var result = await _commandRunner.RunCommandAsync<ResponseForSecretResponse>(command);

        if (result is { Success: true })
        {
            return result.Data;
        }

        throw new BitwardenException(result != null ? result.ErrorMessage : "Secret create failed");
    }

    public async Task<SecretResponse> UpdateAsync(Guid organizationId, Guid id, string key, string value, string note, Guid[] projectIds)
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

        var result = await _commandRunner.RunCommandAsync<ResponseForSecretResponse>(command);

        if (result is { Success: true })
        {
            return result.Data;
        }

        throw new BitwardenException(result != null ? result.ErrorMessage : "Secret update failed");
    }

    public async Task<SecretsDeleteResponse> DeleteAsync(Guid[] ids)
    {
        var command = new Command { Secrets = new SecretsCommand { Delete = new SecretsDeleteRequest { Ids = ids } } };
        var result = await _commandRunner.RunCommandAsync<ResponseForSecretsDeleteResponse>(command);

        if (result is { Success: true })
        {
            return result.Data;
        }

        throw new BitwardenException(result != null ? result.ErrorMessage : "Secrets delete failed");
    }

    public async Task<SecretIdentifiersResponse> ListAsync(Guid organizationId)
    {
        var command = new Command
        {
            Secrets = new SecretsCommand { List = new SecretIdentifiersRequest { OrganizationId = organizationId } }
        };
        var result = await _commandRunner.RunCommandAsync<ResponseForSecretIdentifiersResponse>(command);

        if (result is { Success: true })
        {
            return result.Data;
        }

        throw new BitwardenException(result != null ? result.ErrorMessage : "No secrets for given organization");
    }

    public async Task<SecretsSyncResponse> SyncAsync(Guid organizationId, DateTimeOffset? lastSyncedDate)
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

        var result = await _commandRunner.RunCommandAsync<ResponseForSecretsSyncResponse>(command);

        if (result is { Success: true })
        {
            return result.Data;
        }

        throw new BitwardenException(result != null ? result.ErrorMessage : "Secret update failed");
    }
}
