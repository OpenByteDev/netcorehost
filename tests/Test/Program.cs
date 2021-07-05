using System;
using System.Runtime.InteropServices;

namespace Test {
    public static class Program {
        public static int Hello(IntPtr arg, int argLength) {
            Console.WriteLine("Hello from C#!");
            return 42;
        }

        [UnmanagedCallersOnly]
        public static int UnmanagedHello() {
            return Hello(default, default);
        }

        public static int Main() => Hello(default, default);
    } 
}
