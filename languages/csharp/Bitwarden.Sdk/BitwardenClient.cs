namespace Bitwarden.Sdk;

public sealed class BitwardenClient : IDisposable
{
    private readonly CommandRunner _commandRunner;
    private readonly BitwardenSafeHandle _handle;

    public BitwardenClient(BitwardenSettings? settings = null)
    {
        var clientSettings = new ClientSettings
        {
            ApiUrl = settings?.ApiUrl!,
            IdentityUrl = settings?.IdentityUrl!,
            UserAgent = "Bitwarden DOTNET-SDK"
        };

        _handle = BitwardenLibrary.Init(clientSettings.ToJson());
        _commandRunner = new CommandRunner(_handle);
        Projects = new ProjectsClient(_commandRunner);
        Secrets = new SecretsClient(_commandRunner);
    }

    public async Task AccessTokenLoginAsync(string accessToken)
    {
        var command = new Command { LoginAccessToken = new AccessTokenLoginRequest { AccessToken = accessToken } };
        var response = await _commandRunner.RunCommandAsync<ResponseForApiKeyLoginResponse>(command);
        if (response is not { Success: true })
        {
            throw new BitwardenAuthException(response != null ? response.ErrorMessage : "Login failed");
        }
    }

    public ProjectsClient Projects { get; }

    public SecretsClient Secrets { get; }

    public void Dispose() => _handle.Dispose();
}
