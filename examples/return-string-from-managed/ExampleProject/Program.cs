using System;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Text;

namespace ExampleProject {
    public static class Method1 {
        private static unsafe delegate*<char*, int, byte*> CopyToCString;

        [UnmanagedCallersOnly]
        public static unsafe void SetCopyToCStringFunctionPtr(delegate*<char*, int, byte*> copyToCString) => CopyToCString = copyToCString;

        [UnmanagedCallersOnly]
        public unsafe static byte* GetNameAsCString() {
            var name = "Some string we want to return to Rust.";
            fixed (char* ptr = name) {
                return CopyToCString(ptr, name.Length);
            }
        }
    }

    public static class Method2 {
        [UnmanagedCallersOnly]
        public static IntPtr GetNameAsUnmanagedMemory() {
            var name = "Some string we want to return to Rust.";
            return StringToHGlobalUTF8(name);
        }

        [UnmanagedCallersOnly]
        public static void FreeUnmanagedMemory(IntPtr ptr) {
            Marshal.FreeHGlobal(ptr);
        }

        private static unsafe IntPtr StringToHGlobalUTF8(string? s) {
            if (s is null) {
                return IntPtr.Zero;
            }

            int nb = Encoding.UTF8.GetMaxByteCount(s.Length);

            IntPtr ptr = Marshal.AllocHGlobal(nb + 1);

            int nbWritten;
            byte* pbMem = (byte*)ptr;

            fixed (char* firstChar = s) {
                nbWritten = Encoding.UTF8.GetBytes(firstChar, s.Length, pbMem, nb);
            }

            pbMem[nbWritten] = 0;

            return ptr;
        }
    }

    public static class Method3 {
        [UnmanagedCallersOnly]
        public static IntPtr GetNameAsGCHandle() {
            var name = "Some string we want to return to Rust.";
            return StringToGCHandle(name);
        }

        public static unsafe IntPtr StringToGCHandle(string s) {
            var handle = GCHandle.Alloc(s, GCHandleType.Pinned);
            return GCHandle.ToIntPtr(handle);
        }

        [UnmanagedCallersOnly]
        public static void FreeGCHandleString(IntPtr handle_ptr) {
            GCHandle.FromIntPtr(handle_ptr).Free();
        }

        [UnmanagedCallersOnly]
        public static nuint GetStringDataOffset() => (nuint)RuntimeHelpers.OffsetToStringData;
    }

    public static class Method4 {
        private static unsafe delegate*<nuint, RawVec*, void> RustAllocateMemory;

        [UnmanagedCallersOnly]
        public static unsafe void SetRustAllocateMemory(delegate*<nuint, RawVec*, void> rustAllocateMemory) => RustAllocateMemory = rustAllocateMemory;

        [UnmanagedCallersOnly]
        public unsafe static void GetNameIntoRustVec(RawVec* vec) {
            var name = "Some string we want to return to Rust.";
            *vec = StringToRustVec(name);
        }

        private unsafe static RawVec StringToRustVec(string s) {
            var num_bytes = Encoding.UTF8.GetMaxByteCount(s.Length);

            var vec = new RawVec();
            RustAllocateMemory((nuint)num_bytes, &vec);

            fixed (char* chars = s) {
                vec.Len = (nuint) Encoding.UTF8.GetBytes(chars, s.Length, vec.Data, (int)vec.Capacity);
            }

            return vec;
        }

        [StructLayout(LayoutKind.Sequential)]
        public unsafe struct RawVec {
            public byte* Data;
            public nuint Len;
            public nuint Capacity;
        }
    }
}
