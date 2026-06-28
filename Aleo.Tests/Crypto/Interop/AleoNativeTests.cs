using Aleo.Crypto.Interop;

namespace Aleo.Tests.Crypto.Interop;

public sealed class AleoNativeTests : NativeTestBase
{
    [Fact]
    public void ErrorCode_Success_IsZero()
    {
        Assert.Equal(0, (int)AleoNative.AleoErrorCode.Success);
    }

    [Fact]
    public void ErrorCode_InvalidInput_IsOne()
    {
        Assert.Equal(1, (int)AleoNative.AleoErrorCode.InvalidInput);
    }

    [Fact]
    public void ErrorCode_CryptoError_IsTwo()
    {
        Assert.Equal(2, (int)AleoNative.AleoErrorCode.CryptoError);
    }

    [Fact]
    public void NativeLibraryIsAvailable()
    {
        SkipIfNativeMissing();
    }

    [Fact]
    public void GeneratePrivateKey_ReturnsSuccess()
    {
        SkipIfNativeMissing();

        var result = AleoNative.aleo_generate_private_key(out var key);
        Assert.Equal(AleoNative.AleoErrorCode.Success, result);

        using (key)
        {
            Assert.False(key.IsInvalid);
            Assert.False(key.IsClosed);
        }

        Assert.True(key.IsClosed);
    }

    [Fact]
    public void DeriveViewKey_FromGeneratedKey_ReturnsSuccess()
    {
        SkipIfNativeMissing();

        AleoNative.aleo_generate_private_key(out var privateKey);
        var deriveResult = AleoNative.aleo_derive_view_key(privateKey, out var viewKey);
        Assert.Equal(AleoNative.AleoErrorCode.Success, deriveResult);

        using (viewKey)
        {
            Assert.False(viewKey.IsInvalid);
        }

        privateKey.Dispose();
    }

    [Fact]
    public void DeriveAddress_ReturnsSuccess()
    {
        SkipIfNativeMissing();

        AleoNative.aleo_generate_private_key(out var privateKey);
        AleoNative.aleo_derive_view_key(privateKey, out var viewKey);
        var deriveResult = AleoNative.aleo_derive_address(viewKey, out var address);
        Assert.Equal(AleoNative.AleoErrorCode.Success, deriveResult);

        using (address)
        {
            Assert.False(address.IsInvalid);
        }

        privateKey.Dispose();
        viewKey.Dispose();
    }

    [Fact]
    public void FullKeyGenerationFlow_AllHandlesClosedAfterDispose()
    {
        SkipIfNativeMissing();

        AleoNative.aleo_generate_private_key(out var privateKey);
        AleoNative.aleo_derive_view_key(privateKey, out var viewKey);
        AleoNative.aleo_derive_address(viewKey, out var address);

        privateKey.Dispose();
        viewKey.Dispose();
        address.Dispose();

        Assert.True(privateKey.IsClosed);
        Assert.True(viewKey.IsClosed);
        Assert.True(address.IsClosed);
    }
}
