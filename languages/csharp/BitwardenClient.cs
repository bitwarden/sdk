using Newtonsoft.Json;

namespace Bit.Sdk;

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

        var ptr = BitwardenLibrary.Init(clientSettings.ToJson());
        _handle = new BitwardenSafeHandle(ptr);
        _commandRunner = new CommandRunner(ptr);
        _projectsClient = new ProjectsClient(_commandRunner);
        _secretsClient = new SecretsClient(_commandRunner);
    }

    public ResponseForApiKeyLoginResponse AccessTokenLogin(string accessToken)
    {
        var command = new Command();
        var accessTokenLoginRequest = new AccessTokenLoginRequest
        {
            AccessToken = accessToken
        };
        command.AccessTokenLogin = accessTokenLoginRequest;
        return _commandRunner.RunCommand(command, JsonConvert.DeserializeObject<ResponseForApiKeyLoginResponse>);
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
        _handle.Dispose();
    }
}
