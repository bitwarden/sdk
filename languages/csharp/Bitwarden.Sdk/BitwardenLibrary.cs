using System.Runtime.InteropServices;

namespace Bitwarden.Sdk;

internal static class BitwardenLibrary
{
    [DllImport("bitwarden_c", CallingConvention = CallingConvention.Cdecl)]
    private static extern BitwardenSafeHandle init(string settings);

    [DllImport("bitwarden_c", CallingConvention = CallingConvention.Cdecl)]
    private static extern void free_mem(BitwardenSafeHandle handle);

    [DllImport("bitwarden_c", CallingConvention = CallingConvention.Cdecl)]
    private static extern string run_command(string json, BitwardenSafeHandle handle);

    internal static BitwardenSafeHandle Init(string settings)
    {
        return init(settings);
    }

    internal static void FreeMemory(BitwardenSafeHandle handle)
    {
        free_mem(handle);
    }

    internal static string RunCommand(string json, BitwardenSafeHandle handle)
    {
        return run_command(json, handle);
    }
}
