use std::{
    alloc::{GlobalAlloc, Layout},
    ptr,
    sync::atomic,
};

/// Custom allocator that zeroizes memory before deallocating it
///
/// This is highly recommended to be enabled when using the Bitwarden crates to avoid sensitive data
/// persisting in memory after it has been deallocated.
///
/// This allocator is a decorator around another allocator that zeroizes memory before.
pub struct ZeroizingAllocator<Alloc: GlobalAlloc>(pub Alloc);

unsafe impl<T: GlobalAlloc> GlobalAlloc for ZeroizingAllocator<T> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // Zeroize the memory before propagating the deallocation to the underlying allocator
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

#[cfg(test)]
mod tests {
    #[global_allocator]
    static ALLOC: super::ZeroizingAllocator<std::alloc::System> =
        super::ZeroizingAllocator(std::alloc::System);

    #[test]
    fn string() {
        let s = String::from("hello");

        let p1 = s.as_str().as_ptr();
        let c1 = s.capacity();

        assert_eq!(
            unsafe { std::slice::from_raw_parts(p1, c1) },
            b"hello",
            "String is not at the expected memory location"
        );

        drop(s);

        assert_eq!(
            unsafe { std::slice::from_raw_parts(p1, c1) },
            [0, 0, 0, 0, 0],
            "memory was not zeroized after dropping the string"
        );
    }

    #[test]
    fn string_expand() {
        let mut s = String::from("hello");

        let p1 = s.as_str().as_ptr();
        let c1 = s.capacity();

        assert_eq!(unsafe { std::slice::from_raw_parts(p1, c1) }, b"hello");

        s.push_str(" world");

        let p2: *const u8 = s.as_str().as_ptr();
        let c2 = s.capacity();

        // We allocated a new string
        assert_ne!(p1, p2);
        assert_eq!(
            unsafe { std::slice::from_raw_parts(p1, c1) },
            [0, 0, 0, 0, 0],
            "old string was not zeroized"
        );

        assert_eq!(
            unsafe { std::slice::from_raw_parts(p2, c2) },
            b"hello world"
        );

        // Drop the expanded string
        drop(s);

        assert_eq!(
            unsafe { std::slice::from_raw_parts(p2, c2) },
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            "expanded string was not zeroized"
        );
    }

    #[test]
    fn vec() {
        let v = vec![1, 2, 3, 4, 5];

        let p1 = v.as_slice().as_ptr();
        let c1 = v.capacity();

        assert_eq!(
            unsafe { std::slice::from_raw_parts(p1, c1) },
            [1, 2, 3, 4, 5],
            "vec is not at the expected memory location"
        );

        drop(v);

        assert_eq!(
            unsafe { std::slice::from_raw_parts(p1, c1) },
            [0, 0, 0, 0, 0],
            "vec was not zeroized after dropping"
        );
    }
}
