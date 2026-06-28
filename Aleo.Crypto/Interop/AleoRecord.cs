using System.Runtime.InteropServices;

namespace Aleo.Crypto.Interop
{
    public static class AleoRecord
    {
        public static AleoRecordHandle DecryptRecord(
            AleoViewKeyHandle viewKey,
            string encryptedRecord)
        {
            if (viewKey is null || viewKey.IsInvalid)
                throw new ArgumentException("Invalid view key handle", nameof(viewKey));

            if (encryptedRecord is null)
                throw new ArgumentNullException(nameof(encryptedRecord));

            // Call native FFI
            var error = AleoNative.aleo_decrypt_record(
                viewKey,
                encryptedRecord,
                out var handle);

            if (error != AleoNative.AleoErrorCode.Success)
            {
                var msg = AleoNative.GetLastError();
                throw new AleoException($"Record decryption failed: {msg}");
            }

            return handle;
        }

        public static string EncryptRecord(
            AleoAddressHandle recipient,
            string plaintextRecord)
        {
            if (recipient is null || recipient.IsInvalid)
                throw new ArgumentException("Invalid recipient address handle", nameof(recipient));

            if (plaintextRecord is null)
                throw new ArgumentNullException(nameof(plaintextRecord));

            var error = AleoNative.EncryptRecord(
                recipient,
                plaintextRecord,
                out var ptr);

            if (error != AleoNative.AleoErrorCode.Success)
            {
                var msg = AleoNative.GetLastError();
                throw new AleoException($"Record encryption failed: {msg}");
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
