using Aleo.Crypto.Interop;

namespace Aleo.Tests.Crypto.Interop;

public sealed class AleoPrivateKeyHandleTests : NativeTestBase
{
    [Fact]
    public void Default_IsInvalid()
    {
        using var handle = new AleoPrivateKeyHandle();
        Assert.True(handle.IsInvalid);
        Assert.False(handle.IsClosed);
    }

    [Fact]
    public void DisposeMultipleTimes_DoesNotThrow()
    {
        SkipIfNativeMissing();

        AleoNative.aleo_generate_private_key(out var key);
        key.Dispose();
        key.Dispose();
    }

    [Fact]
    public void DeriveViewKey_DisposedKey_ThrowsObjectDisposed()
    {
        SkipIfNativeMissing();

        AleoNative.aleo_generate_private_key(out var privateKey);
        privateKey.Dispose();

        Assert.Throws<ObjectDisposedException>(() =>
            AleoNative.aleo_derive_view_key(privateKey, out _));
    }
}
