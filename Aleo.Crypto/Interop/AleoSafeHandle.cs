using System;
using System.Runtime.InteropServices;

namespace Aleo.Crypto.Interop
{
    public abstract class AleoSafeHandle : SafeHandle
    {
        protected AleoSafeHandle() : base(IntPtr.Zero, ownsHandle: true) { }

        public override bool IsInvalid => handle == IntPtr.Zero;
    }
}
