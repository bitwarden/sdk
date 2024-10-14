use std::{ffi::CStr, os::raw::c_char, str};

use bitwarden_json::client::Client;

use crate::{box_ptr, ffi_ref};

#[repr(C)]
pub struct CClient {
    /// Associates the tokio runtime to the `Client`, ensuring the runtime has the same lifecycle
    /// as the `Client`.
    runtime: tokio::runtime::Runtime,
    client: Client,
}

#[no_mangle]
pub extern "C" fn run_command(c_str_ptr: *const c_char, client_ptr: *const CClient) -> *mut c_char {
    let client = unsafe { ffi_ref!(client_ptr) };
    let input_str = str::from_utf8(unsafe { CStr::from_ptr(c_str_ptr) }.to_bytes())
        .expect("Input should be a valid string");

    let result = client
        .runtime
        .block_on(client.client.run_command(input_str));

    match std::ffi::CString::new(result) {
        Ok(cstr) => cstr.into_raw(),
        Err(_) => panic!("failed to return command result: null encountered"),
    }
}

// Init client, potential leak! You need to call free_mem after this!
#[no_mangle]
pub extern "C" fn init(c_str_ptr: *const c_char) -> *mut CClient {
    // This will only fail if another logger was already initialized, so we can ignore the result
    let _ = env_logger::try_init();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to build tokio runtime");

    let client = if c_str_ptr.is_null() {
        Client::new(None)
    } else {
        let input_string = str::from_utf8(unsafe { CStr::from_ptr(c_str_ptr) }.to_bytes())
            .expect("Input should be a valid string")
            .to_owned();
        Client::new(Some(input_string))
    };

    box_ptr!(CClient { runtime, client })
}

// Free mem
#[no_mangle]
pub extern "C" fn free_mem(client_ptr: *mut CClient) {
    std::mem::drop(unsafe { Box::from_raw(client_ptr) });
}
