namespace Bit.Sdk;

public class BitwardenSettings
{
    /// <summary>
    /// The api url of the targeted Bitwarden instance. Defaults to `https://api.bitwarden.com`
    /// </summary>
    public string? ApiUrl { get; set; }

    /// <summary>
    /// The identity url of the targeted Bitwarden instance. Defaults to
    /// `https://identity.bitwarden.com`
    /// </summary>
    public string? IdentityUrl { get; set; }
}
