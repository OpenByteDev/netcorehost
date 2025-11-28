using System;
using System.Runtime.InteropServices;
using System.Runtime.CompilerServices;

namespace ClassLibrary {
    public class Library {
        [ModuleInitializer]
        internal static void Init() {
            Console.WriteLine("Library DLL Loaded");
            Console.WriteLine($"Running under .NET {Environment.Version}");
        }

        [UnmanagedCallersOnly]
        public static int Hello() {
            Console.WriteLine("Hello from Library!");
            return 42;
        }
    }
}
