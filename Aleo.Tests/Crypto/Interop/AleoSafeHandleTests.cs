using Aleo.Crypto.Interop;

namespace Aleo.Tests.Crypto.Interop;

public sealed class AleoSafeHandleTests
{
    [Fact]
    public void IsInvalid_ReturnsTrue_ForDefaultConstructedHandle()
    {
        using AleoSafeHandle handle = new AleoPrivateKeyHandle();
        Assert.True(handle.IsInvalid);
    }

    [Fact]
    public void IsClosed_ReturnsFalse_BeforeDisposal()
    {
        using AleoSafeHandle handle = new AleoPrivateKeyHandle();
        Assert.False(handle.IsClosed);
    }

    [Fact]
    public void CanBeDisposed_MultipleTimes_WithoutThrow()
    {
        AleoSafeHandle handle = new AleoPrivateKeyHandle();
        handle.Dispose();
        handle.Dispose();
    }
}
