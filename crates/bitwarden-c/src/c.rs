use std::{ffi::CStr, os::raw::c_char, str};

use crate::{box_ptr, ffi_ref};
use bitwarden_json::client::Client;

pub struct BitwardenClient {
    client: Client,
    f: Option<extern "C" fn(*const c_char)>,
}

#[no_mangle]
#[tokio::main]
pub async extern "C" fn run_command(
    c_str_ptr: *const c_char,
    client_ptr: *mut BitwardenClient,
) -> *mut c_char {
    let client = unsafe { ffi_ref!(client_ptr) };
    let input_str = str::from_utf8(unsafe { CStr::from_ptr(c_str_ptr).to_bytes() }).unwrap();
    //println!("{}", input_str);
    client.f.unwrap()(c_str_ptr);

    let result = client.client.run_command(input_str).await;
    return match std::ffi::CString::new(result) {
        Ok(cstr) => cstr.into_raw(),
        Err(_) => panic!("failed to return command result: null encountered"),
    };
}

#[no_mangle]
pub extern "C" fn subscribe(f: extern "C" fn(*const c_char), client_ptr: *mut BitwardenClient) {
    let client = unsafe { ffi_ref!(client_ptr) };
    client.f = Some(f);
}

// Init client, potential leak! You need to call free_mem after this!
#[no_mangle]
pub extern "C" fn init(c_str_ptr: *const c_char) -> *mut BitwardenClient {
    env_logger::init();
    if c_str_ptr.is_null() {
        return box_ptr!(BitwardenClient {
            client: Client::new(None),
            f: None,
        });
    } else {
        let input_string = str::from_utf8(unsafe { CStr::from_ptr(c_str_ptr).to_bytes() })
            .unwrap()
            .to_owned();
        return box_ptr!(BitwardenClient {
            client: Client::new(Some(input_string)),
            f: None,
        });
    }
}

// Free mem
#[no_mangle]
pub extern "C" fn free_mem(client_ptr: *mut BitwardenClient) {
    std::mem::drop(unsafe { Box::from_raw(client_ptr) });
}
