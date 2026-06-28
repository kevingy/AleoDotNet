using System.Runtime.InteropServices;
using Aleo.Crypto.Interop;

namespace Aleo.Tests.Crypto.Interop;

public abstract class NativeTestBase
{
    private static readonly bool s_nativeAvailable;
    private static readonly string? s_nativeLibPath;

    static NativeTestBase()
    {
        var rustDir = Path.Combine(
            AppContext.BaseDirectory, "..", "..", "..", "..",
            "rust-engine", "target", "debug");

        var libName = OperatingSystem.IsWindows()
            ? "aleo_dotnet_engine.dll"
            : "libaleo_dotnet_engine.so";

        s_nativeLibPath = Directory.Exists(rustDir)
            ? Path.GetFullPath(Path.Combine(rustDir, libName))
            : null;

        s_nativeAvailable = File.Exists(s_nativeLibPath);

        if (s_nativeAvailable)
        {
            NativeLibrary.SetDllImportResolver(
                typeof(AleoNative).Assembly,
                (name, assembly, path) =>
                {
                    if (name == "aleo_dotnet_engine" && s_nativeLibPath is not null)
                        return NativeLibrary.Load(s_nativeLibPath);
                    return IntPtr.Zero;
                });
        }
    }

    protected static bool NativeLibraryAvailable => s_nativeAvailable;

    protected static string? NativeLibraryPath => s_nativeLibPath;

    protected static void SkipIfNativeMissing()
    {
        if (!s_nativeAvailable)
            Assert.Skip(
                $"Native library not found. Build the Rust engine first: " +
                $"expected at {s_nativeLibPath}");
    }
}
