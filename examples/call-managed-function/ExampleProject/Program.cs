using System;
using System.Runtime.InteropServices;

namespace ExampleProject {
    public static class Program {
        public delegate void HelloWorld1Delegate();
        public static void HelloWorld1() {
            Console.WriteLine("Hello from C#!");
        }

        [UnmanagedCallersOnly]
        public static void HelloWorld2() {
            Console.WriteLine("Hello from C#!");
        }
        
        public static int HelloWorld3(IntPtr arg, int argLength) {
            Console.WriteLine("Hello from C#!");
            return 0;
        }
    }
}
