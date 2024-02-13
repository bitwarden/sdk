// Get a reference to an object from a pointer
#[macro_export]
macro_rules! ffi_ref {
    ($name:ident) => {{
        assert!(!$name.is_null());
        &*$name
    }};
}

// Returns a raw pointer from an object
#[macro_export]
macro_rules! box_ptr {
    ($x:expr) => {
        Box::into_raw(Box::new($x))
    };
}
