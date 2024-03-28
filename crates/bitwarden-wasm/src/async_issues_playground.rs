extern crate console_error_panic_hook;

use std::cell::Cell;

use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[derive(Default)]
struct VaultItem {
    // cipher_id: String,
    // name: String,
}

unsafe impl Send for VaultItem {}

impl VaultItem {
    pub fn new(cipher_id: String, name: String) -> Self {
        Self {}
        // Self { cipher_id, name }
    }
}
trait Fido2GetAssertionUserInterface {
    fn pick_credential(
        &self,
        rp_id: &str,
    ) -> impl std::future::Future<Output = bitwarden_json::Result<VaultItem>> + Send;
    // async fn pick_credential(&self, rp_id: &str) -> bitwarden_json::Result<VaultItem>;
}

#[wasm_bindgen]
extern "C" {
    pub type JSFido2GetAssertionUserInterface;

    #[wasm_bindgen(structural, method)]
    pub fn pick_credential(this: &JSFido2GetAssertionUserInterface, rp_id: String) -> Promise;
}

impl Fido2GetAssertionUserInterface for JSFido2GetAssertionUserInterface {
    // async fn pick_credential(&self, rp_id: &str) -> bitwarden_json::Result<VaultItem> {
    fn pick_credential(
        &self,
        rp_id: &str,
    ) -> impl std::future::Future<Output = bitwarden_json::Result<VaultItem>> + Send {
        log::debug!("JSFido2GetAssertionUserInterface.pick_credential");
        let picked_id_promise = self.pick_credential(rp_id.to_string());

        async move {
            // let picked_id = wasm_bindgen_futures::JsFuture::from(picked_id_promise).await; // <-- causes issue

            Ok(VaultItem::new("cipher_id".to_owned(), "name".to_owned()))
            // Ok(VaultItem::new(
            //     picked_id.unwrap().as_string().unwrap(),
            //     "name".to_string(),
            // ))
        }
    }
}

#[derive(Default)]
struct Wrap<T>(T);
unsafe impl<T> Sync for Wrap<T> {}

struct Fido2Session<U> {
    user_interface: Wrap<U>,
    user_presence: Wrap<Cell<bool>>,
}

impl<U> Fido2Session<U>
where
    U: Fido2GetAssertionUserInterface,
{
    fn new(user_interface: U) -> Self {
        Self {
            user_interface: Wrap(user_interface),
            user_presence: Wrap(Cell::new(false)),
        }
    }
}

struct Fido2CredentialStore<'a, U>
where
    U: Fido2GetAssertionUserInterface,
{
    session: &'a Fido2Session<U>,
}

#[async_trait::async_trait]
trait Fido2CredentialStoreTrait {
    async fn find_credentials(&self, rp_id: &str) -> Result<Vec<VaultItem>, String>;
}

#[async_trait::async_trait]
impl<'a, U> Fido2CredentialStoreTrait for Fido2CredentialStore<'a, U>
where
    U: Fido2GetAssertionUserInterface,
{
    async fn find_credentials(&self, rp_id: &str) -> Result<Vec<VaultItem>, String> {
        let session = self.session.clone();
        let picked = session.user_interface.0.pick_credential(rp_id).await;

        let result = match picked {
            Ok(item) => Ok(vec![item]),
            Err(e) => Err("Something went wrong".to_owned()),
        };

        // todo!()
        result
    }
}

fn uuid_raw_to_standard_format(uuid: &Vec<u8>) -> String {
    let mut uuid_str = String::with_capacity(36);
    uuid_str.push_str(&format!(
        "{:02X}{:02X}{:02X}{:02X}-",
        uuid[0], uuid[1], uuid[2], uuid[3]
    ));
    uuid_str.push_str(&format!("{:02X}{:02X}-", uuid[4], uuid[5]));
    uuid_str.push_str(&format!("{:02X}{:02X}-", uuid[6], uuid[7]));
    uuid_str.push_str(&format!("{:02X}{:02X}-", uuid[8], uuid[9]));
    for i in 10..uuid.len() {
        uuid_str.push_str(&format!("{:02X}", uuid[i]));
    }
    uuid_str
}
