// We create a new function that takes the wasm object, spawns a local task and creates the communication channels
// This function will wrap the channels in a struct to allow us to implement the trait there
impl JSFido2MakeCredentialUserInterface {
    fn to_channel_wrapped(self) -> JSFido2MakeCredentialUserInterfaceWrapper {
        let (tx_wrapper, mut rx_task) = futures::channel::mpsc::channel::<NewCredentialParams>(1);
        let (mut tx_task, rx_wrapper) = futures::channel::mpsc::channel::<NewCredentialResult>(1);

        // Spawn the local task which just waits until we receive input from the trait, note that this is not Send but we don't care
        wasm_bindgen_futures::spawn_local(async move {
            let params = rx_task.next().await.unwrap();

            let picked_id_promise = self.confirm_new_credential(
                params.credential_name,
                params.user_name,
                params.user_verification,
            );
            let picked_id = wasm_bindgen_futures::JsFuture::from(picked_id_promise).await;
            let picked_id = picked_id.unwrap().as_string().unwrap();

            tx_task
                .send(NewCredentialResult {
                    cipher_id: picked_id,
                    user_verified: false,
                })
                .await
                .unwrap();
        });

        JSFido2MakeCredentialUserInterfaceWrapper {
            tx: async_lock::Mutex::new(tx_wrapper),
            rx: async_lock::Mutex::new(rx_wrapper),
        }
    }
}

struct JSFido2MakeCredentialUserInterfaceWrapper {
    tx: async_lock::Mutex<futures::channel::mpsc::Sender<NewCredentialParams>>,
    rx: async_lock::Mutex<futures::channel::mpsc::Receiver<NewCredentialResult>>,
}

// This is implemented over the wrapper now, which only contains the channels, and should be Send
#[async_trait::async_trait]
impl Fido2MakeCredentialUserInterface for JSFido2MakeCredentialUserInterfaceWrapper {
    async fn confirm_new_credential(
        &self,
        params: NewCredentialParams,
    ) -> bitwarden_json::Result<NewCredentialResult> {
        log::debug!("JSFido2MakeCredentialUserInterface.pick_credential");

        self.tx.lock().await.send(params).await.unwrap();

        let result = self.rx.lock().await.next().await.unwrap();

        Ok(result)
    }
}

// impl Fido2GetAssertionUserInterface for JSFido2GetAssertionUserInterface {
//     async fn pick_credential(
//         &self,
//         cipher_ids: Vec<String>,
//         rp_id: &str,
//     ) -> bitwarden_json::Result<VaultItem> {
//         log::debug!("JSFido2GetAssertionUserInterface.pick_credential");
//         let picked_id_promise = self.pick_credential(cipher_ids.clone(), rp_id.to_string());

//         let picked_id = wasm_bindgen_futures::JsFuture::from(picked_id_promise).await;

//         Ok(VaultItem::new(
//             picked_id.unwrap().as_string().unwrap(),
//             "name".to_string(),
//         ))
//     }
// }
