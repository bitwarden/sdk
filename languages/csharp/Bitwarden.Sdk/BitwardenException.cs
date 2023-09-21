namespace Bitwarden.Sdk;

public class BitwardenException : Exception
{
    public BitwardenException(string message) : base(message)
    {
    }

    public BitwardenException(string message, Exception innerException)
        : base(message, innerException)
    {
    }
}
