using Microsoft.Win32.SafeHandles;

namespace Bitwarden.Sdk;

internal class BitwardenSafeHandle : SafeHandleZeroOrMinusOneIsInvalid
{
    public BitwardenSafeHandle() : base(true) => SetHandle(handle);

    protected override bool ReleaseHandle()
    {
        BitwardenLibrary.FreeMemory(handle);
        return true;
    }
}
