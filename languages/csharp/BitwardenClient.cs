namespace Bitwarden.Sdk;

public sealed class BitwardenClient : IDisposable
{
    private readonly ProjectsClient _projectsClient;
    private readonly SecretsClient _secretsClient;
    private readonly CommandRunner _commandRunner;
    private readonly BitwardenSafeHandle _handle;

    public BitwardenClient(BitwardenSettings? settings = null)
    {
        var clientSettings = new ClientSettings
        {
            ApiUrl = settings is { ApiUrl: not null } ? settings.ApiUrl : "https://api.bitwarden.com",
            IdentityUrl = settings is { IdentityUrl: not null } ? settings.IdentityUrl : "https://identity.bitwarden.com",
            DeviceType = DeviceType.Sdk,
            UserAgent = "Bitwarden DOTNET-SDK"
        };

        _handle = BitwardenLibrary.Init(clientSettings.ToJson());
        _commandRunner = new CommandRunner(_handle);
        _projectsClient = new ProjectsClient(_commandRunner);
        _secretsClient = new SecretsClient(_commandRunner);
    }

    public ResponseForApiKeyLoginResponse? AccessTokenLogin(string accessToken)
    {
        var command = new Command() { AccessTokenLogin = new AccessTokenLoginRequest { AccessToken = accessToken } };
        return _commandRunner.RunCommand<ResponseForApiKeyLoginResponse>(command);
    }

    public ProjectsClient Projects()
    {
        return _projectsClient;
    }

    public SecretsClient Secrets()
    {
        return _secretsClient;
    }

    public void Dispose()
    {
        BitwardenLibrary.FreeMemory(_handle);
        _handle.Dispose();
    }
}
