using System.Runtime.InteropServices;

namespace Bit.Sdk;

internal static class BitwardenLibrary
{
    [DllImport("bitwarden_c", CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr init(string settings);

    [DllImport("bitwarden_c", CallingConvention = CallingConvention.Cdecl)]
    private static extern void free_mem(IntPtr clientPtr);

    [DllImport("bitwarden_c", CallingConvention = CallingConvention.Cdecl)]
    private static extern string run_command(string loginRequest, IntPtr clientPtr);

    internal static IntPtr Init(string settings)
    {
        return init(settings);
    }

    internal static void FreeMemory(IntPtr clientPtr)
    {
        free_mem(clientPtr);
    }

    internal static string RunCommand(string loginRequest, IntPtr clientPtr)
    {
        return run_command(loginRequest, clientPtr);
    }
}
