using System.Runtime.InteropServices;

namespace Bit.Sdk;

internal class BitwardenSafeHandle : SafeHandle
{
    public BitwardenSafeHandle(IntPtr handle) : base(IntPtr.Zero, true)
    {
        SetHandle(handle);
    }

    public override bool IsInvalid
    {
        get { return handle == IntPtr.Zero; }
    }

    protected override bool ReleaseHandle()
    {
        BitwardenLibrary.FreeMemory(handle);
        return true;
    }
}
