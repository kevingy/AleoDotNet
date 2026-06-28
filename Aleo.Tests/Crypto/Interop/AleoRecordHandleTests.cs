using System.Runtime.InteropServices;
using Aleo.Crypto.Interop;

namespace Aleo.Tests.Crypto.Interop;

public sealed class AleoRecordHandleTests : NativeTestBase
{
    [Fact]
    public void Default_IsInvalid()
    {
        using var handle = new AleoRecordHandle();
        Assert.True(handle.IsInvalid);
    }

    [Fact]
    public void DisposeMultipleTimes_DoesNotThrow()
    {
        var handle = new AleoRecordHandle();
        handle.Dispose();
        handle.Dispose();
    }

    [Fact]
    public void RecordToJson_NullRecordHandle_ReturnsInvalidInput()
    {
        using var handle = new AleoRecordHandle();

        var result = AleoNative.aleo_record_to_json(handle, out _);

        Assert.Equal(AleoNative.AleoErrorCode.InvalidInput, result);
    }
}
