using System;
using System.Runtime.InteropServices;
using System.Runtime.Versioning;
using System.Text;

namespace ExampleProject {
    public static class Program {
        [UnmanagedCallersOnly]
        public static unsafe int IndirectIncrement1(int n) {
            var module = NativeLibrary.GetMainProgramHandle();
            var rusty_increment = (delegate* unmanaged<int, int>)NativeLibrary.GetExport(module, "rusty_increment");
            return rusty_increment(n);
        }

        [SupportedOSPlatform("windows")]
        [UnmanagedCallersOnly]
        public static int IndirectIncrement2(int n) {
            return rusty_increment(n);
        }

        [SupportedOSPlatform("windows")]
        [DllImport("call-native-function.exe", ExactSpelling = true)]
        public static extern int rusty_increment(int n); 
    }
}
