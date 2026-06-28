using Aleo.Crypto.Interop;

namespace Aleo.Tests.Crypto.Interop;

public sealed class AleoAddressHandleTests : NativeTestBase
{
    [Fact]
    public void Default_IsInvalid()
    {
        using var handle = new AleoAddressHandle();
        Assert.True(handle.IsInvalid);
    }

    [Fact]
    public void DisposeMultipleTimes_DoesNotThrow()
    {
        SkipIfNativeMissing();

        AleoNative.aleo_generate_private_key(out var privateKey);
        AleoNative.aleo_derive_view_key(privateKey, out var viewKey);
        AleoNative.aleo_derive_address(viewKey, out var address);
        privateKey.Dispose();
        viewKey.Dispose();
        address.Dispose();
        address.Dispose();
    }
}
