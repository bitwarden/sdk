using Microsoft.Win32.SafeHandles;

namespace Bitwarden.Sdk;

internal class BitwardenSafeHandle : SafeHandleZeroOrMinusOneIsInvalid
{
    public BitwardenSafeHandle() : base(true)
    {
        SetHandle(handle);
    }

    protected override bool ReleaseHandle()
    {
        if (IsClosed) return false;
        BitwardenLibrary.FreeMemory(this);
        return true;
    }
}
