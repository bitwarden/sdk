use crate::error::{Error, Result};
use std::fmt::Write;

/// Convert raw 16 byte array to standard format (XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX) UUID.
pub fn guid_to_standard_format(buffer_source: &Vec<u8>) -> Result<String> {
    if buffer_source.len() != 16 {
        return Err(Error::Internal("Input should be a 16 byte array".into()));
    }

    let mut guid = String::with_capacity(36);
    for (i, byte) in buffer_source.iter().enumerate() {
        if i == 4 || i == 6 || i == 8 || i == 10 {
            write!(&mut guid, "-").unwrap();
        }
        write!(&mut guid, "{:02x}", byte).unwrap();
    }

    // Consistency check for valid UUID.
    if !is_valid_guid(&guid) {
        return Err(Error::Internal("Converted GUID is invalid".into()));
    }

    Ok(guid)
}

/// Perform format validation, without enforcing any variant restrictions as Utils.isGuid does
fn is_valid_guid(guid: &str) -> bool {
    let re = regex::Regex::new(r"^[0-9a-f]{8}-(?:[0-9a-f]{4}-){3}[0-9a-f]{12}$").unwrap();
    re.is_match(guid)
}
