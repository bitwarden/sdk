use crate::{Cipher, Folder};

pub(crate) fn export_json(folders: Vec<Folder>, ciphers: Vec<Cipher>) -> Result<String, String> {
    Ok("".to_owned())
}

struct JsonExport {
    encrypted: bool,
    folders: Vec<Folder>,
    ciphers: Vec<Cipher>,
}
