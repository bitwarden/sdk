using System.Runtime.InteropServices;

namespace Bitwarden.Sdk;

internal class BitwardenSafeHandle : SafeHandle
{
    public BitwardenSafeHandle() : base(IntPtr.Zero, true)
    {
        SetHandle(handle);
    }

    public override bool IsInvalid
    {
        get { return handle == IntPtr.Zero; }
    }

    protected override bool ReleaseHandle()
    {
        if (IsClosed) return false;
        BitwardenLibrary.FreeMemory(this);
        return true;
    }
}
