using System.Runtime.InteropServices;

namespace Aleo.Crypto.Interop
{
    public static partial class AleoNative
    {
        public enum AleoErrorCode : int
        {
            Success = 0,
            InvalidInput = 1,
            CryptoError = 3,
        }

        [LibraryImport("aleo_dotnet_engine", EntryPoint = "aleo_generate_private_key")]
        public static partial AleoErrorCode aleo_generate_private_key(
            out AleoPrivateKeyHandle out_key);

        [LibraryImport("aleo_dotnet_engine", EntryPoint = "aleo_derive_view_key")]
        public static partial AleoErrorCode aleo_derive_view_key(
            AleoPrivateKeyHandle private_key,
            out AleoViewKeyHandle out_view_key);

        [LibraryImport("aleo_dotnet_engine", EntryPoint = "aleo_derive_address")]
        public static partial AleoErrorCode aleo_derive_address(
            AleoViewKeyHandle view_key,
            out AleoAddressHandle out_address);

        [LibraryImport("aleo_dotnet_engine", EntryPoint = "aleo_decrypt_record", StringMarshalling = StringMarshalling.Utf8)]
        public static partial AleoErrorCode aleo_decrypt_record(
            AleoViewKeyHandle view_key,
            string encrypted_record,
            out AleoRecordHandle out_decrypted_record);

        [LibraryImport("aleo_dotnet_engine", EntryPoint = "aleo_free_string")]
        public static partial void aleo_free_string(IntPtr s);

        [LibraryImport("aleo_dotnet_engine", EntryPoint = "aleo_record_to_json", StringMarshalling = StringMarshalling.Utf8)]
        public static partial AleoErrorCode aleo_record_to_json(
            AleoRecordHandle record,
            out IntPtr out_string);

        [LibraryImport("aleo_dotnet_engine", EntryPoint = "aleo_get_last_error")]
        private static partial nint aleo_get_last_error();

        public static string GetLastError()
        {
            var ptr = aleo_get_last_error();
            return ptr == 0 ? string.Empty : Marshal.PtrToStringUTF8(ptr) ?? string.Empty;
        }

        [LibraryImport("aleo_dotnet_engine", EntryPoint = "aleo_encrypt_record")]
        private static partial AleoErrorCode aleo_encrypt_record(
            AleoAddressHandle recipient,
            [MarshalAs(UnmanagedType.LPUTF8Str)] string plaintextRecord,
            out nint outEncryptedRecord);

        public static AleoErrorCode EncryptRecord(
            AleoAddressHandle recipient,
            string plaintextRecord,
            out nint outEncryptedRecord)
        {
            return aleo_encrypt_record(recipient, plaintextRecord, out outEncryptedRecord);
        }
    }
}
