use core::slice;
use std::alloc::{GlobalAlloc, Layout};

use zeroize::Zeroize;

/// Custom allocator that zeroizes memory before deallocating it
///
/// This is highly recommended to be enabled when using the Bitwarden crates to avoid sensitive data
/// persisting in memory after it has been deallocated.
///
/// This allocator is a decorator around another allocator.
pub struct ZeroizingAllocator<Alloc: GlobalAlloc>(pub Alloc);

unsafe impl<T: GlobalAlloc> GlobalAlloc for ZeroizingAllocator<T> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        slice::from_raw_parts_mut(ptr, layout.size()).zeroize();

        self.0.dealloc(ptr, layout);
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        self.0.alloc_zeroed(layout)
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

        let p2 = s.as_str().as_ptr();
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

    #[test]
    fn vec_expand() {
        let mut v = vec![1, 2, 3, 4, 5];

        let p1 = v.as_slice().as_ptr();
        let c1 = v.capacity();

        assert_eq!(
            unsafe { std::slice::from_raw_parts(p1, c1) },
            [1, 2, 3, 4, 5],
            "vec is not at the expected memory location"
        );

        v.extend_from_slice(&[6, 7, 8, 9, 10]);

        let p2 = v.as_slice().as_ptr();
        let c2 = v.capacity();

        // We allocated a new vector
        assert_ne!(p1, p2);
        assert_eq!(
            unsafe { std::slice::from_raw_parts(p1, c1) },
            [0, 0, 0, 0, 0],
            "old vec was not zeroized"
        );

        assert_eq!(
            unsafe { std::slice::from_raw_parts(p2, c2) },
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        );

        // Drop the expanded vector
        drop(v);

        assert_eq!(
            unsafe { std::slice::from_raw_parts(p2, c2) },
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            "expanded vec was not zeroized"
        );
    }
}
