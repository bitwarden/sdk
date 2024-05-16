use std::{
    alloc::{GlobalAlloc, Layout},
    ptr,
    sync::atomic,
};

pub struct ZeroizingAllocator<Alloc: GlobalAlloc>(pub Alloc);

unsafe impl<T: GlobalAlloc> GlobalAlloc for ZeroizingAllocator<T> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { volatile_set(ptr, 0, layout.size()) }
        atomic_fence();

        self.0.dealloc(ptr, layout);
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        self.0.alloc_zeroed(layout)
    }
}

/// Borrowed from `zeroize` crate
/// <https://github.com/RustCrypto/utils/blob/2e3ad4106ff01b3b3a7ca651fc793267e69d1e47/zeroize/src/lib.rs#L761>
///
/// MIT License
/// Copyright (c) 2018-2021 The RustCrypto Project Developers
///
/// Use fences to prevent accesses from being reordered before this
/// point, which should hopefully help ensure that all accessors
/// see zeroes after this point.
#[inline(always)]
fn atomic_fence() {
    atomic::compiler_fence(atomic::Ordering::SeqCst);
}

/// Borrowed from `zeroize` crate
/// <https://github.com/RustCrypto/utils/blob/2e3ad4106ff01b3b3a7ca651fc793267e69d1e47/zeroize/src/lib.rs#L780>
///
/// MIT License
/// Copyright (c) 2018-2021 The RustCrypto Project Developers
///
/// Perform a volatile `memset` operation which fills a slice with a value
///
/// Safety:
/// The memory pointed to by `dst` must be a single allocated object that is valid for `count`
/// contiguous elements of `T`.
/// `count` must not be larger than an `isize`.
/// `dst` being offset by `mem::size_of::<T> * count` bytes must not wrap around the address space.
/// Also `dst` must be properly aligned.
#[inline(always)]
unsafe fn volatile_set<T: Copy + Sized>(dst: *mut T, src: T, count: usize) {
    // TODO(tarcieri): use `volatile_set_memory` when stabilized
    for i in 0..count {
        // Safety:
        //
        // This is safe because there is room for at least `count` objects of type `T` in the
        // allocation pointed to by `dst`, because `count <= isize::MAX` and because
        // `dst.add(count)` must not wrap around the address space.
        let ptr = dst.add(i);

        // Safety:
        //
        // This is safe, because the pointer is valid and because `dst` is well aligned for `T` and
        // `ptr` is an offset of `dst` by a multiple of `mem::size_of::<T>()` bytes.
        ptr::write_volatile(ptr, src);
    }
}
