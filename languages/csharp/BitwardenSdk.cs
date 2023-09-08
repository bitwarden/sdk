using System.Runtime.InteropServices;

namespace Bitwarden.Sdk
{
    internal class BitwardenSdk : IDisposable
    {
        [DllImport("bitwarden_c", CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr init(string settings);
        [DllImport("bitwarden_c", CallingConvention = CallingConvention.Cdecl)]
        private static extern void free_mem(IntPtr clientPtr);

        [DllImport("bitwarden_c", CallingConvention = CallingConvention.Cdecl)]
        private static extern string run_command(string loginRequest, IntPtr clientPtr);

        private readonly IntPtr _ptr;
        private bool disposedValue;

        public BitwardenSdk(ClientSettings? settings = null)
        {
            _ptr = init(settings?.ToJson());
        }

        public ResponseForPasswordLoginResponse? PasswordLogin(string email, string password) => RunCommand<ResponseForPasswordLoginResponse>(new Command
        {
            PasswordLogin = new PasswordLoginRequest {
                Email = email,
                Password = password
            },
        }, ResponseForPasswordLoginResponse.FromJson);

        public ResponseForUserApiKeyResponse? UserApiKey(string password) => RunCommand<ResponseForUserApiKeyResponse>(new Command
        {
            GetUserApiKey = new SecretVerificationRequest {
                MasterPassword = password
            },
        }, ResponseForUserApiKeyResponse.FromJson);

        private TReturn? RunCommand<TReturn>(Command input, Func<string, TReturn> deserializer)
        {
            var req = input.ToJson();

            var json_result = run_command(req, _ptr);
            Console.WriteLine(json_result);
            return deserializer(json_result);
        }

        protected virtual void Dispose(bool disposing)
        {
            if (!disposedValue)
            {
                if (disposing)
                {
                    // TODO: dispose managed state (managed objects)
                }

                free_mem(_ptr);
                disposedValue = true;
            }
        }


        ~BitwardenSdk()
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
