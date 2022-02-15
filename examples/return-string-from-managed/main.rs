use core::slice;
use std::{
    ffi::{CStr, CString},
    mem::{self, MaybeUninit},
    os::raw::c_char,
    str::Utf8Error,
    string::FromUtf16Error,
};

use netcorehost::{
    cast_managed_fn, hostfxr::AssemblyDelegateLoader, nethost, pdcstr, pdcstring::PdCStr,
};

#[path = "../helpers/dotnet-build.rs"]
mod dotnet_build;

fn main() {
    dotnet_build::build_example_project("return-string-from-managed");

    let hostfxr = nethost::load_hostfxr().unwrap();
    let context = hostfxr.initialize_for_runtime_config(pdcstr!("examples/return-string-from-managed/ExampleProject/bin/Debug/net5.0/ExampleProject.runtimeconfig.json")).unwrap();
    let delegate_loader = context
        .get_delegate_loader_for_assembly(pdcstr!(
            "examples/return-string-from-managed/ExampleProject/bin/Debug/net5.0/ExampleProject.dll"
        ))
        .unwrap();

    print_string_from_csharp_using_c_string(&delegate_loader);
    print_string_from_csharp_using_unmanaged_alloc(&delegate_loader);
    print_string_from_csharp_using_gc_handle(&delegate_loader);
    print_string_from_csharp_using_rust_allocate(&delegate_loader);
}

// Method 1: using CString
fn print_string_from_csharp_using_c_string<A: AsRef<PdCStr>>(
    delegate_loader: &AssemblyDelegateLoader<A>,
) {
    let set_copy_to_c_string = delegate_loader
        .get_function_pointer_for_unmanaged_callers_only_method(
            pdcstr!("ExampleProject.Method1, ExampleProject"),
            pdcstr!("SetCopyToCStringFunctionPtr"),
        )
        .unwrap();
    let set_copy_to_c_string = unsafe {
        cast_managed_fn!(
            set_copy_to_c_string,
            fn(f: extern "system" fn(*const u16, i32) -> *mut c_char)
        )
    };
    set_copy_to_c_string(copy_to_c_string);

    let get_name = delegate_loader
        .get_function_pointer_for_unmanaged_callers_only_method(
            pdcstr!("ExampleProject.Method1, ExampleProject"),
            pdcstr!("GetNameAsCString"),
        )
        .unwrap();
    let get_name = unsafe { cast_managed_fn!(get_name, fn() -> *mut c_char) };
    let name_ptr = get_name();
    let name = unsafe { CString::from_raw(name_ptr) };
    println!("{}", name.to_string_lossy());
}

pub extern "system" fn copy_to_c_string(ptr: *const u16, length: i32) -> *mut c_char {
    let wide_chars = unsafe { slice::from_raw_parts(ptr, length as usize) };
    let string = String::from_utf16_lossy(wide_chars);
    let c_string = match CString::new(string) {
        Ok(c_string) => c_string,
        Err(_) => return std::ptr::null_mut(),
    };
    c_string.into_raw()
}

// Method 2: using GCHandle
fn print_string_from_csharp_using_unmanaged_alloc<A: AsRef<PdCStr>>(
    delegate_loader: &AssemblyDelegateLoader<A>,
) {
    // one time setup
    let free_h_global = delegate_loader
        .get_function_pointer_for_unmanaged_callers_only_method(
            pdcstr!("ExampleProject.Method2, ExampleProject"),
            pdcstr!("FreeUnmanagedMemory"),
        )
        .unwrap();
    let free_h_global = unsafe { cast_managed_fn!(free_h_global, fn(*const u8)) };
    unsafe { FREE_H_GLOBAL = Some(free_h_global) };

    // actual usage
    let get_name = delegate_loader
        .get_function_pointer_for_unmanaged_callers_only_method(
            pdcstr!("ExampleProject.Method2, ExampleProject"),
            pdcstr!("GetNameAsUnmanagedMemory"),
        )
        .unwrap();
    let get_name = unsafe { cast_managed_fn!(get_name, fn() -> *const u8) };
    let name_h_global = get_name();
    let name = unsafe { HGlobalString::from_h_global(name_h_global) };
    println!("{}", name.as_str().unwrap());
}

// use OnceCell or similar instead if possible.
static mut FREE_H_GLOBAL: Option<extern "system" fn(*const u8)> = None;

struct HGlobalString {
    ptr: *const u8,
    len: usize,
}

impl HGlobalString {
    pub unsafe fn from_h_global(ptr: *const u8) -> Self {
        let len = CStr::from_ptr(ptr.cast()).to_bytes().len();
        Self { ptr, len }
    }
    #[allow(dead_code)]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
    pub fn as_bytes_with_nul(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len + 1) }
    }
    pub fn as_c_str(&self) -> &CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(self.as_bytes_with_nul()) }
    }
    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        self.as_c_str().to_str()
    }
}

impl Drop for HGlobalString {
    fn drop(&mut self) {
        unsafe { FREE_H_GLOBAL }.expect("string interop not init")(self.ptr);
    }
}

// Method 3: using GCHandle
fn print_string_from_csharp_using_gc_handle<A: AsRef<PdCStr>>(
    delegate_loader: &AssemblyDelegateLoader<A>,
) {
    // one time setup
    let free_gc_handle_string = delegate_loader
        .get_function_pointer_for_unmanaged_callers_only_method(
            pdcstr!("ExampleProject.Method3, ExampleProject"),
            pdcstr!("FreeGCHandleString"),
        )
        .unwrap();
    let free_gc_handle_string =
        unsafe { cast_managed_fn!(free_gc_handle_string, fn(*const *const u16)) };
    unsafe { FREE_GC_HANDLE_STRING = Some(free_gc_handle_string) };

    let get_string_data_offset = delegate_loader
        .get_function_pointer_for_unmanaged_callers_only_method(
            pdcstr!("ExampleProject.Method3, ExampleProject"),
            pdcstr!("GetStringDataOffset"),
        )
        .unwrap();
    let get_string_data_offset = unsafe { cast_managed_fn!(get_string_data_offset, fn() -> usize) };
    let string_data_offset = get_string_data_offset();
    unsafe { STRING_DATA_OFFSET = Some(string_data_offset) };

    // actual usage
    let get_name = delegate_loader
        .get_function_pointer_for_unmanaged_callers_only_method(
            pdcstr!("ExampleProject.Method3, ExampleProject"),
            pdcstr!("GetNameAsGCHandle"),
        )
        .unwrap();
    let get_name = unsafe { cast_managed_fn!(get_name, fn() -> *const *const u16) };
    let name_gc_handle = get_name();
    let name = unsafe { GcHandleString::from_gc_handle(name_gc_handle) };
    println!("{}", name.to_string_lossy());
}

// use OnceCell or similar instead if possible.
static mut FREE_GC_HANDLE_STRING: Option<extern "system" fn(*const *const u16)> = None;
static mut STRING_DATA_OFFSET: Option<usize> = None;

struct GcHandleString(*const *const u16);

impl GcHandleString {
    pub unsafe fn from_gc_handle(ptr: *const *const u16) -> Self {
        Self(ptr)
    }
    pub fn data_ptr(&self) -> *const u16 {
        // convert the handle pointer to the actual string pointer by removing the mark.
        let unmarked_ptr = (self.0 as usize & !1usize) as *const *const u16;
        (unsafe { *unmarked_ptr } as usize
            + unsafe { STRING_DATA_OFFSET }.expect("string interop not init")) as *const u16
    }
    pub fn len(&self) -> usize {
        // read the length of the string which is stored in front of the data.
        let len_ptr = (self.data_ptr() as usize - mem::size_of::<i32>()) as *const i32;
        unsafe { *len_ptr as usize }
    }
    pub fn wide_chars(&self) -> &[u16] {
        unsafe { slice::from_raw_parts(self.data_ptr(), self.len()) }
    }
    #[allow(dead_code)]
    pub fn to_string(&self) -> Result<String, FromUtf16Error> {
        String::from_utf16(self.wide_chars())
    }
    pub fn to_string_lossy(&self) -> String {
        String::from_utf16_lossy(self.wide_chars())
    }
}

impl Drop for GcHandleString {
    fn drop(&mut self) {
        unsafe { FREE_GC_HANDLE_STRING }.expect("string interop not init")(self.0);
    }
}

// Method 4: using rust allocate
fn print_string_from_csharp_using_rust_allocate<A: AsRef<PdCStr>>(
    delegate_loader: &AssemblyDelegateLoader<A>,
) {
    // one time setup
    let set_rust_allocate_memory = delegate_loader
        .get_function_pointer_for_unmanaged_callers_only_method(
            pdcstr!("ExampleProject.Method4, ExampleProject"),
            pdcstr!("SetRustAllocateMemory"),
        )
        .unwrap();
    let set_rust_allocate_memory = unsafe {
        cast_managed_fn!(
            set_rust_allocate_memory,
            fn(extern "system" fn(usize, *mut RawVec<u8>))
        )
    };
    set_rust_allocate_memory(rust_allocate_memory);

    // actual usage
    let get_name = delegate_loader
        .get_function_pointer_for_unmanaged_callers_only_method(
            pdcstr!("ExampleProject.Method4, ExampleProject"),
            pdcstr!("GetNameIntoRustVec"),
        )
        .unwrap();
    let get_name = unsafe { cast_managed_fn!(get_name, fn(*mut RawVec<u8>)) };

    let mut name_raw_vec = MaybeUninit::uninit();
    get_name(name_raw_vec.as_mut_ptr());
    let name_raw_vec = unsafe { name_raw_vec.assume_init() };
    let name_vec =
        unsafe { Vec::from_raw_parts(name_raw_vec.data, name_raw_vec.len, name_raw_vec.capacity) };
    let name = String::from_utf8(name_vec).unwrap();
    println!("{}", name);
}

extern "system" fn rust_allocate_memory(size: usize, vec: *mut RawVec<u8>) {
    let mut buf = Vec::<u8>::with_capacity(size);
    unsafe {
        *vec = RawVec {
            data: buf.as_mut_ptr(),
            len: buf.len(),
            capacity: buf.capacity(),
        }
    };
    mem::forget(buf);
}

#[repr(C)]
struct RawVec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}
