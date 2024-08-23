using System.Runtime.InteropServices;

namespace Bitwarden.Sdk;

internal static partial class BitwardenLibrary
{
    [LibraryImport("bitwarden_c", StringMarshalling = StringMarshalling.Utf8)]
    private static partial BitwardenSafeHandle init(string settings);

    [LibraryImport("bitwarden_c", StringMarshalling = StringMarshalling.Utf8)]
    private static partial void free_mem(IntPtr handle);

    [LibraryImport("bitwarden_c", StringMarshalling = StringMarshalling.Utf8)]
    private static partial string run_command(string json, BitwardenSafeHandle handle);

    internal delegate void OnCompleteCallback(IntPtr json);

    [LibraryImport("bitwarden_c", StringMarshalling = StringMarshalling.Utf8)]
    private static partial void run_command_async(string json, BitwardenSafeHandle handle, OnCompleteCallback on_completed_callback);

    internal static BitwardenSafeHandle Init(string settings) => init(settings);

    internal static void FreeMemory(IntPtr handle) => free_mem(handle);

    internal static string RunCommand(string json, BitwardenSafeHandle handle) => run_command(json, handle);

    internal static Task<string> RunCommandAsync(string json, BitwardenSafeHandle handle, CancellationToken token = default)
    {
        var tcs = new TaskCompletionSource<string>(TaskCreationOptions.RunContinuationsAsynchronously);

        try
        {
            run_command_async(json, handle, (pointer) =>
            {
                try
                {
                    var stringResult = Marshal.PtrToStringUTF8(pointer);
                    Console.WriteLine($"Setting Result {stringResult}");
                    tcs.SetResult(stringResult);
                    Console.WriteLine("Set Result");
                }
                finally
                {
                    Marshal.FreeCoTaskMem(pointer);
                }
            });
        }
        catch (Exception ex)
        {
            tcs.SetException(ex);
        }

        // TODO: Register cancellation on token

        return tcs.Task;
    }
}
