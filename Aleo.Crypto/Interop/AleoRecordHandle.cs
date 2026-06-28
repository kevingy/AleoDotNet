using System;
using System.Runtime.InteropServices;

namespace Aleo.Crypto.Interop
{
    public sealed partial class AleoRecordHandle : AleoSafeHandle
    {
        public AleoRecordHandle() { }
        public AleoRecordHandle(IntPtr ptr) { SetHandle(ptr); }

        [LibraryImport("aleo_dotnet_engine", EntryPoint = "aleo_free_record")]
        private static partial void aleo_free_record(nint ptr);

        protected override bool ReleaseHandle()
        {
            aleo_free_record(handle);
            return true;
        }
    }
}
