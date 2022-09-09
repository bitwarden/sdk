using Bit.Sdk;

var sdk = new BitwardenSdk();
sdk.PasswordLogin("test@bitwarden.com", "asdfasdf");
var apiKey = sdk.UserApiKey("asdfasdf");
Console.WriteLine(apiKey?.Data?.ApiKey ?? "api key was null");
