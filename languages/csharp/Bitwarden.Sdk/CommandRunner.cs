using System.Text.Json;

namespace Bitwarden.Sdk;

internal class CommandRunner
{
    private readonly BitwardenSafeHandle _handle;

    internal CommandRunner(BitwardenSafeHandle handle)
    {
        _handle = handle;
    }

    internal T? RunCommand<T>(Command command)
    {
        var req = JsonSerializer.Serialize(command, Converter.Settings);
        var result = BitwardenLibrary.RunCommand(req, _handle);
        return JsonSerializer.Deserialize<T>(result, Converter.Settings);
    }

    internal async Task<T?> RunCommandAsync<T>(Command command)
    {
        var req = JsonSerializer.Serialize(command, Converter.Settings);
        var result = await BitwardenLibrary.RunCommandAsync(req, _handle);
        return JsonSerializer.Deserialize<T>(result, Converter.Settings);
    }
}
