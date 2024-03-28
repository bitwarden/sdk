fn to_channel_wrapped<'a, F, P, R, M>(
    function: F,
    mapper: M,
) -> (
    async_lock::Mutex<futures::channel::mpsc::Sender<P>>,
    async_lock::Mutex<futures::channel::mpsc::Receiver<R>>,
)
where
    F: Fn(P) -> Promise + 'static,
    P: 'static,
    R: 'static,
    M: Fn(Promise) -> R,
{
    let (tx_wrapper, mut rx_task) = futures::channel::mpsc::channel::<P>(1);
    let (mut tx_task, rx_wrapper) = futures::channel::mpsc::channel::<R>(1);

    wasm_bindgen_futures::spawn_local(async move {
        let params = rx_task.next().await.unwrap();
        let result = mapper(function(params));
        tx_task.send(result).await.unwrap();
    });

    (
        async_lock::Mutex::new(tx_wrapper),
        async_lock::Mutex::new(rx_wrapper),
    )
}

impl JSFido2UserInterface {
    fn to_channel_wrapped(&self) -> JSFido2UserInterfaceWrapper {
        let (tx_wrapper, rx_wrapper) = to_channel_wrapped(
            |x| JSFido2UserInterface::confirm_new_credential(self, x),
            |r| JsNewCredentialResult {
                test: "test".to_owned(),
            },
        );
        JSFido2UserInterfaceWrapper {
            confirm_new_credential: (tx_wrapper, rx_wrapper),
        }
    }
}

struct JSFido2UserInterfaceWrapper {
    confirm_new_credential: (
        async_lock::Mutex<futures::channel::mpsc::Sender<JsNewCredentialParams>>,
        async_lock::Mutex<futures::channel::mpsc::Receiver<JsNewCredentialResult>>,
    ),
}
