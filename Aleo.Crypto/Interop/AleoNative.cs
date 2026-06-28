using System.Runtime.InteropServices;

namespace Aleo.Crypto.Interop
{
    public static partial class AleoNative
    {
        public enum AleoErrorCode : int
        {
            Success = 0,
            InvalidInput = 1,
            CryptoError = 2,
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
    }
}
