using System;
using System.Runtime.InteropServices;

namespace Aleo.Crypto.Interop
{
    public static class AleoRecordHandleExtensions
    {
        public static string ToJson(this AleoRecordHandle handle)
        {
            var error = AleoNative.aleo_record_to_json(handle, out var ptr);

            if (error != AleoNative.AleoErrorCode.Success)
            {
                var msg = AleoNative.GetLastError();
                throw new AleoException($"Failed to serialize record: {msg}");
            }

            try
            {
                return Marshal.PtrToStringUTF8(ptr) ?? string.Empty;
            }
            finally
            {
                AleoNative.aleo_free_string(ptr);
            }
        }
    }
}
