using Newtonsoft.Json;

namespace Bit.Sdk
{
    public sealed class BitwardenClient : IDisposable
    {
        private readonly Projects _projects;
        private readonly Secrets _secrets;
        private readonly CommandRunner _commandRunner;
        private readonly IntPtr _ptr;
        private bool _disposedValue;

        public BitwardenClient(Settings? settings = null)
        {
            var clientSettings = new ClientSettings
            {
                ApiUrl = settings is { ApiUrl: not null } ? settings.ApiUrl : "https://api.bitwarden.com",
                IdentityUrl = settings is { IdentityUrl: not null } ? settings.IdentityUrl : "https://identity.bitwarden.com",
                DeviceType = DeviceType.Sdk,
                UserAgent = "Bitwarden DOTNET-SDK"
            };

            _ptr = BitwardenLibrary.init(clientSettings.ToJson());
            _commandRunner = new CommandRunner(_ptr);
            _projects = new Projects(_commandRunner);
            _secrets = new Secrets(_commandRunner);
        }

        public ResponseForApiKeyLoginResponse AccessTokenLogin(string accessToken) {
            var command = new Command();
            var accessTokenLoginRequest = new AccessTokenLoginRequest
            {
                AccessToken = accessToken
            };
            command.AccessTokenLogin = accessTokenLoginRequest;
            return _commandRunner.RunCommand(command, JsonConvert.DeserializeObject<ResponseForApiKeyLoginResponse>);
        }

        public Projects Projects() {
            return _projects;
        }

        public Secrets Secrets() {
            return _secrets;
        }

        private void Dispose(bool disposing)
        {
            if (!_disposedValue)
            {
                if (disposing)
                {
                    // TODO: dispose managed state (managed objects)
                }

                BitwardenLibrary.free_mem(_ptr);
                _disposedValue = true;
            }
        }

        ~BitwardenClient()
        {
            // Do not change this code. Put cleanup code in 'Dispose(bool disposing)' method
            Dispose(disposing: false);
        }

        public void Dispose()
        {
            // Do not change this code. Put cleanup code in 'Dispose(bool disposing)' method
            Dispose(disposing: true);
            GC.SuppressFinalize(this);
        }
    }
}
