namespace Bitwarden.Sdk;

public class BitwardenAuthException : Exception
{
    public BitwardenAuthException(string message) : base(message)
    {
    }

    public BitwardenAuthException(string message, Exception innerException)
        : base(message, innerException)
    {
    }
}
