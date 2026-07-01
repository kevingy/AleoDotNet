using System.Runtime.InteropServices;
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
    public void ErrorCode_CryptoError_IsThree()
    {
        Assert.Equal(3, (int)AleoNative.AleoErrorCode.CryptoError);
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

    [Fact]
    public void AleoDecryptRecord_DefaultViewKey_ReturnsInvalidInput()
    {
        SkipIfNativeMissing();

        using var viewKey = new AleoViewKeyHandle();

        var result = AleoNative.aleo_decrypt_record(viewKey, "encrypted", out _);

        Assert.Equal(AleoNative.AleoErrorCode.InvalidInput, result);
    }

    [Fact]
    public void AleoDecryptRecord_NullEncryptedRecord_ReturnsInvalidInput()
    {
        SkipIfNativeMissing();

        AleoNative.aleo_generate_private_key(out var privateKey);
        AleoNative.aleo_derive_view_key(privateKey, out var viewKey);
        privateKey.Dispose();

        var result = AleoNative.aleo_decrypt_record(viewKey, null!, out _);
        Assert.Equal(AleoNative.AleoErrorCode.InvalidInput, result);

        viewKey.Dispose();
    }

    [Fact]
    public void AleoDecryptRecord_EmptyString_ReturnsCryptoError()
    {
        SkipIfNativeMissing();

        AleoNative.aleo_generate_private_key(out var privateKey);
        AleoNative.aleo_derive_view_key(privateKey, out var viewKey);
        privateKey.Dispose();

        var result = AleoNative.aleo_decrypt_record(viewKey, "", out var outHandle);
        Assert.Equal(AleoNative.AleoErrorCode.CryptoError, result);
        Assert.True(outHandle.IsInvalid);

        viewKey.Dispose();
    }

    [Fact]
    public void AleoDecryptRecord_InvalidCiphertext_ReturnsCryptoError()
    {
        SkipIfNativeMissing();

        AleoNative.aleo_generate_private_key(out var privateKey);
        AleoNative.aleo_derive_view_key(privateKey, out var viewKey);
        privateKey.Dispose();

        const string input = "not_a_real_ciphertext";
        var result = AleoNative.aleo_decrypt_record(viewKey, input, out var outHandle);
        Assert.Equal(AleoNative.AleoErrorCode.CryptoError, result);
        Assert.True(outHandle.IsInvalid);

        viewKey.Dispose();
    }

    [Fact]
    public void FreeString_NullPtr_DoesNotThrow()
    {
        SkipIfNativeMissing();

        AleoNative.aleo_free_string(IntPtr.Zero);
    }

    [Fact]
    public void GetLastError_AfterDecryptError_ReturnsNonEmpty()
    {
        SkipIfNativeMissing();

        using var viewKey = new AleoViewKeyHandle();

        AleoNative.aleo_decrypt_record(viewKey, "invalid", out _);

        var error = AleoNative.GetLastError();
        Assert.NotEqual(string.Empty, error);
    }

    [Fact]
    public void GetLastError_SubsequentCall_StaysSet()
    {
        SkipIfNativeMissing();

        using var viewKey = new AleoViewKeyHandle();

        AleoNative.aleo_decrypt_record(viewKey, "garbage", out _);

        var error1 = AleoNative.GetLastError();
        var error2 = AleoNative.GetLastError();
        Assert.Equal(error1, error2);
    }
}
