using System;
using System.Runtime.InteropServices;

namespace Aleo.Crypto.Interop
{
    public sealed partial class AleoViewKeyHandle : AleoSafeHandle
    {
        [LibraryImport("aleo_dotnet_engine", EntryPoint = "aleo_free_view_key")]
        private static partial void aleo_free_view_key(nint ptr);

        protected override bool ReleaseHandle()
        {
            aleo_free_view_key(handle);
            return true;
        }
    }
}
