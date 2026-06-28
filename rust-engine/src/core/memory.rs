// Memory management utilities for FFI layer
// Provides safe allocation and deallocation helpers

use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

/// Allocate memory of a given size
/// 
/// # Safety
/// Caller must ensure the allocated memory is properly freed using aleo_free
#[no_mangle]
pub extern "C" fn aleo_alloc(size: usize) -> *mut std::os::raw::c_void {
    if size == 0 {
        return ptr::null_mut();
    }
    
    let layout = Layout::from_size_align(size, 1).unwrap_or_else(|_| {
        // Fallback to size 1 if layout creation fails
        Layout::from_size_align(1, 1).unwrap()
    });
    
    unsafe {
        let ptr = alloc(layout);
        if ptr.is_null() {
            ptr::null_mut()
        } else {
            ptr as *mut std::os::raw::c_void
        }
    }
}

/// Free memory allocated by aleo_alloc
/// 
/// # Safety
/// ptr must have been allocated by aleo_alloc
#[no_mangle]
pub extern "C" fn aleo_free(ptr: *mut std::os::raw::c_void) {
    if ptr.is_null() {
        return;
    }
    
    // Note: We can't deallocate without knowing the original layout
    // This is a simple implementation - in production, you'd need to track sizes
    // or use a different allocation strategy
    // TODO: Implement proper memory tracking for deallocation
    // For now, we can't safely deallocate without tracking size
    // This is a placeholder - the actual implementation will need
    // to track allocation sizes or use a different strategy
}

/// Allocate and copy a string into memory suitable for FFI
/// 
/// Returns a pointer to a null-terminated C string
/// Caller must free using aleo_free_string
pub fn alloc_string(s: &str) -> *mut std::os::raw::c_char {
    std::ffi::CString::new(s)
        .map(|c_string| c_string.into_raw())
        .unwrap_or_else(|_| ptr::null_mut())
}

/// Allocate and copy bytes into memory
/// 
/// Returns a pointer to the allocated memory
/// Caller must free using aleo_free_bytes
pub fn alloc_bytes(data: &[u8]) -> *mut u8 {
    if data.is_empty() {
        return ptr::null_mut();
    }
    
    let layout = Layout::from_size_align(data.len(), 1).unwrap();
    unsafe {
        let ptr = alloc(layout);
        if !ptr.is_null() {
            ptr::copy_nonoverlapping(data.as_ptr(), ptr, data.len());
        }
        ptr
    }
}

/// Free bytes allocated by alloc_bytes
#[no_mangle]
pub extern "C" fn aleo_free_bytes(ptr: *mut u8, size: usize) {
    if ptr.is_null() || size == 0 {
        return;
    }
    
    let layout = Layout::from_size_align(size, 1).unwrap();
    unsafe {
        dealloc(ptr, layout);
    }
}

/// Safe wrapper for FFI-allocated strings
pub struct FfiString {
    ptr: *mut std::os::raw::c_char,
}

impl FfiString {
    /// Create a new FfiString from a Rust string
    pub fn new(s: &str) -> Self {
        FfiString {
            ptr: alloc_string(s),
        }
    }
    
    /// Get the raw pointer (for FFI)
    pub fn as_ptr(&self) -> *mut std::os::raw::c_char {
        self.ptr
    }
    
    /// Check if the pointer is null
    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

impl Drop for FfiString {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let _ = std::ffi::CString::from_raw(self.ptr);
            }
        }
    }
}

unsafe impl Send for FfiString {}
unsafe impl Sync for FfiString {}
