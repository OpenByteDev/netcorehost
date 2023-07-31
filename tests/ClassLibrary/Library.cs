using System;
using System.Runtime.InteropServices;

namespace ClassLibrary {
    public class Library {
        [UnmanagedCallersOnly]
        public static int Hello() {
            Console.WriteLine("Hello from Library!");
            return 42;
        }
    }
}
