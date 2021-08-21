using System;
using System.Runtime.InteropServices;

namespace ExampleProject {
    public static class Program {
        [UnmanagedCallersOnly]
        public static void PrintUtf8(/* byte* */ IntPtr textPtr, int textLength) {
            var text = Marshal.PtrToStringUTF8(textPtr, textLength);
            Console.WriteLine(text);
        }
        [UnmanagedCallersOnly]
        public static void PrintUtf16(/* char* */ IntPtr textPtr, int textLength) {
            var text = Marshal.PtrToStringUni(textPtr, textLength);
            Console.WriteLine(text);
        }

        [UnmanagedCallersOnly]
        public static unsafe int IsPalindrom(char* textPtr, int textLength) {
            var text = new ReadOnlySpan<char>(textPtr, textLength); // this does not copy the string like the methods on Marshal do.

            for (var i=0; i < text.Length / 2; i++) {
                if (char.ToLower(text[i]) != char.ToLower(text[text.Length - i - 1])) {
                    return 0;
                }
            }

            return 1;
        }

        [UnmanagedCallersOnly]
        public static unsafe float GetLength(Vector2f* vector) {
            return (float) Math.Sqrt(vector->x*vector->x + vector->y*vector->y);
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct Vector2f {
            public float x;
            public float y;
        }
    }
}
