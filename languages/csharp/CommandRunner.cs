namespace Bit.Sdk;

internal class CommandRunner
{

    private readonly IntPtr _client;

    internal CommandRunner(IntPtr client)
    {
        _client = client;
    }

    internal TReturn? RunCommand<TReturn>(Command command, Func<string, TReturn> deserializer)
    {
        var req = command.ToJson();
        var result = BitwardenLibrary.RunCommand(req, _client);
        return deserializer(result);
    }
}
