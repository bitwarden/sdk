namespace Bitwarden.Sdk;
using System.Text.Json;

internal class CommandRunner
{

    private readonly BitwardenSafeHandle _handle;

    internal CommandRunner(BitwardenSafeHandle handle)
    {
        _handle = handle;
    }

    internal T? RunCommand<T>(Command command)
    {
        var req = command.ToJson();
        var result = BitwardenLibrary.RunCommand(req, _handle);
        return JsonSerializer.Deserialize<T>(result);
    }
}
