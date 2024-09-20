namespace Bitwarden.Sdk;

public class AuthClient
{
    private readonly CommandRunner _commandRunner;

    internal AuthClient(CommandRunner commandRunner)
    {
        _commandRunner = commandRunner;
    }

    public void LoginAccessToken(string accessToken, string stateFile = "")
    {
        var command = new Command { LoginAccessToken = new AccessTokenLoginRequest { AccessToken = accessToken, StateFile = stateFile } };
        var response = _commandRunner.RunCommand<ResponseForApiKeyLoginResponse>(command);
        if (response is not { Success: true })
        {
            throw new BitwardenAuthException(response != null ? response.ErrorMessage : "Login failed");
        }
    }
}
