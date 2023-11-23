using System.Runtime.InteropServices;

namespace Bitwarden.Sdk;

internal static class BitwardenLibrary
{
    [DllImport("bitwarden_c", CallingConvention = CallingConvention.Cdecl)]
    private static extern BitwardenSafeHandle init(string settings);

    [DllImport("bitwarden_c", CallingConvention = CallingConvention.Cdecl)]
    private static extern void free_mem(IntPtr handle);

    [DllImport("bitwarden_c", CallingConvention = CallingConvention.Cdecl)]
    private static extern string run_command(string json, BitwardenSafeHandle handle);

    internal static BitwardenSafeHandle Init(string settings) => init(settings);

    internal static void FreeMemory(IntPtr handle) => free_mem(handle);

    internal static string RunCommand(string json, BitwardenSafeHandle handle) => run_command(json, handle);
}
