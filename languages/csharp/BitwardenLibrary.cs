using System.Runtime.InteropServices;

namespace Bit.Sdk;

internal static class BitwardenLibrary {

    [DllImport("bitwarden_c", CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr init(string settings);

    [DllImport("bitwarden_c", CallingConvention = CallingConvention.Cdecl)]
    internal static extern void free_mem(IntPtr clientPtr);

    [DllImport("bitwarden_c", CallingConvention = CallingConvention.Cdecl)]
    internal static extern string run_command(string loginRequest, IntPtr clientPtr);
}
