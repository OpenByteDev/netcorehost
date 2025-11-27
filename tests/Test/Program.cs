using System;
using System.Runtime.InteropServices;
using System.Runtime.CompilerServices;

namespace Test {
    public static class Program {
        [ModuleInitializer]
        internal static void Init() {
            Console.WriteLine("C# Module Loaded");
            Console.WriteLine($"Running under .NET {Environment.Version}");
        }

        public static int Hello(IntPtr arg, int argLength) {
            Console.WriteLine("Hello from C#!");
            return 42;
        }
        public static int Hello2(IntPtr arg, int argLength) {
            Console.WriteLine("Hello again?");
            return 0;
        }

        public delegate void CustomHelloFunc();
        public static void CustomHello() {
            Console.WriteLine("Hello from C#!");
        }

        [UnmanagedCallersOnly]
        public static int UnmanagedHello() {
            return Hello(default, default);
        }

        public static int Main() => Hello(default, default);
    } 
}
