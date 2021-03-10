using System;

namespace Test {
    public static class Program {
        public static int Hello(IntPtr arg, int argLength) {
            Console.WriteLine("Hello from C#!");
            return 42;
        }

        public static int Main() => Hello(default, default);
    } 
}
