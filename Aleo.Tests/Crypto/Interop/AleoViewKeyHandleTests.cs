using Aleo.Crypto.Interop;

namespace Aleo.Tests.Crypto.Interop;

public sealed class AleoViewKeyHandleTests : NativeTestBase
{
    [Fact]
    public void Default_IsInvalid()
    {
        using var handle = new AleoViewKeyHandle();
        Assert.True(handle.IsInvalid);
    }

    [Fact]
    public void DisposeMultipleTimes_DoesNotThrow()
    {
        SkipIfNativeMissing();

        AleoNative.aleo_generate_private_key(out var privateKey);
        AleoNative.aleo_derive_view_key(privateKey, out var viewKey);
        privateKey.Dispose();
        viewKey.Dispose();
        viewKey.Dispose();
    }

    [Fact]
    public void DeriveAddress_DisposedViewKey_ThrowsObjectDisposed()
    {
        SkipIfNativeMissing();

        AleoNative.aleo_generate_private_key(out var privateKey);
        AleoNative.aleo_derive_view_key(privateKey, out var viewKey);
        privateKey.Dispose();
        viewKey.Dispose();

        Assert.Throws<ObjectDisposedException>(() =>
            AleoNative.aleo_derive_address(viewKey, out _));
    }
}
