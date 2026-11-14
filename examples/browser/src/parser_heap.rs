//! Use the application's allocator for the embedded parser on bare WebAssembly.
//! The scanner runtime's default C heap expects an external host to initialize it.

use std::alloc::{Layout, alloc, alloc_zeroed, dealloc, realloc};
use std::ffi::c_void;
use std::ptr;
use std::sync::Once;

// wasm32's C ABI gives max_align_t 16-byte alignment. Reserve one aligned
// header so C callers can free or resize a block without passing its layout.
const ALIGN: usize = 16;

pub fn install() {
    static INSTALLED: Once = Once::new();
    INSTALLED.call_once(|| {
        // Called before creating any Tree-sitter object. The pointer family is
        // installed as a unit, including for the grammar's external scanner.
        unsafe {
            tree_sitter::set_allocator(Some(reserve), Some(zeroed), Some(resize), Some(release))
        };
    });
}

fn layout(size: usize) -> Option<Layout> {
    Layout::from_size_align(size.checked_add(ALIGN)?, ALIGN).ok()
}

fn allocate(size: usize, clear: bool) -> *mut c_void {
    let Some(layout) = layout(size) else {
        return ptr::null_mut();
    };
    // The header and user bytes share one live allocation. The first word
    // retains the size required by Rust's allocator even if C requests zero.
    unsafe {
        let base = if clear {
            alloc_zeroed(layout)
        } else {
            alloc(layout)
        };
        if base.is_null() {
            return ptr::null_mut();
        }
        base.cast::<usize>().write(size);
        base.add(ALIGN).cast()
    }
}

unsafe extern "C" fn reserve(size: usize) -> *mut c_void {
    allocate(size, false)
}

unsafe extern "C" fn zeroed(count: usize, size: usize) -> *mut c_void {
    count
        .checked_mul(size)
        .map_or(ptr::null_mut(), |bytes| allocate(bytes, true))
}

// SAFETY: a non-null argument must have been returned by this allocator and
// must still be live. The caller grants exclusive ownership to free/resize.
unsafe fn block(pointer: *mut c_void) -> (*mut u8, Layout) {
    unsafe {
        let base = pointer.cast::<u8>().sub(ALIGN);
        let size = base.cast::<usize>().read();
        // The header is initialized only after this layout has been validated.
        (base, Layout::from_size_align_unchecked(size + ALIGN, ALIGN))
    }
}

unsafe extern "C" fn release(pointer: *mut c_void) {
    if !pointer.is_null() {
        unsafe {
            let (base, layout) = block(pointer);
            dealloc(base, layout);
        }
    }
}

unsafe extern "C" fn resize(pointer: *mut c_void, size: usize) -> *mut c_void {
    if pointer.is_null() {
        return allocate(size, false);
    }
    if size == 0 {
        unsafe { release(pointer) };
        return ptr::null_mut();
    }
    let Some(next) = layout(size) else {
        return ptr::null_mut();
    };
    unsafe {
        let (base, previous) = block(pointer);
        let moved = realloc(base, previous, next.size());
        if moved.is_null() {
            return ptr::null_mut();
        }
        moved.cast::<usize>().write(size);
        moved.add(ALIGN).cast()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_allocation_contract() {
        unsafe {
            let p = zeroed(4, 8).cast::<u8>();
            assert!(!p.is_null());
            assert_eq!(p as usize % ALIGN, 0);
            for i in 0..32 {
                assert_eq!(*p.add(i), 0);
                p.add(i).write(i as u8);
            }
            assert!(resize(p.cast(), usize::MAX).is_null());
            let grown = resize(p.cast(), 96).cast::<u8>();
            assert!(!grown.is_null());
            let shrunk = resize(grown.cast(), 12).cast::<u8>();
            assert!(!shrunk.is_null());
            for i in 0..12 {
                assert_eq!(*shrunk.add(i), i as u8);
            }
            release(shrunk.cast());
            assert!(zeroed(usize::MAX, 2).is_null());
            let p = resize(ptr::null_mut(), 32);
            assert!(!p.is_null());
            assert!(resize(p, 0).is_null());
            release(ptr::null_mut());
        }
    }
}
