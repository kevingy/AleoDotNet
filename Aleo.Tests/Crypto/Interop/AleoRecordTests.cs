using System.Runtime.InteropServices;
using Aleo.Crypto.Interop;

namespace Aleo.Tests.Crypto.Interop;

public sealed class AleoRecordTests : NativeTestBase
{
    [Fact]
    public void DecryptRecord_NullViewKey_ThrowsArgumentException()
    {
        var ex = Assert.Throws<ArgumentException>(() =>
            AleoRecord.DecryptRecord(null!, "encrypted"));
        Assert.Contains("viewKey", ex.Message);
    }

    [Fact]
    public void DecryptRecord_DefaultViewKey_ThrowsArgumentException()
    {
        using var viewKey = new AleoViewKeyHandle();

        var ex = Assert.Throws<ArgumentException>(() =>
            AleoRecord.DecryptRecord(viewKey, "encrypted"));
        Assert.Contains("viewKey", ex.Message);
    }

    [Fact]
    public void DecryptRecord_NullEncryptedRecord_ThrowsArgumentNullException()
    {
        SkipIfNativeMissing();

        AleoNative.aleo_generate_private_key(out var privateKey);
        AleoNative.aleo_derive_view_key(privateKey, out var viewKey);
        privateKey.Dispose();

        Assert.Throws<ArgumentNullException>(() =>
            AleoRecord.DecryptRecord(viewKey, null!));

        viewKey.Dispose();
    }

    [Fact]
    public void DecryptRecord_InvalidCiphertext_ThrowsAleoException()
    {
        SkipIfNativeMissing();

        AleoNative.aleo_generate_private_key(out var privateKey);
        AleoNative.aleo_derive_view_key(privateKey, out var viewKey);
        privateKey.Dispose();

        var ex = Assert.Throws<AleoException>(() =>
            AleoRecord.DecryptRecord(viewKey, "invalid_ciphertext"));
        Assert.Contains("decryption", ex.Message, StringComparison.OrdinalIgnoreCase);

        viewKey.Dispose();
    }
}
