using System;
using System.Runtime.InteropServices;

namespace Aleo.Crypto.Interop
{
    public sealed partial class AleoAddressHandle : AleoSafeHandle
    {
        [LibraryImport("aleo_dotnet_engine", EntryPoint = "aleo_free_address")]
        private static partial void aleo_free_address(nint ptr);

        protected override bool ReleaseHandle()
        {
            aleo_free_address(handle);
            return true;
        }
    }
}
