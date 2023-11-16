use serde::{Deserialize, Serialize};

const STATE_VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
struct State {
    version: u32,
}

/*
    ~/.bws is the main storage location

    Each profile will have the following

    [profiles.default]
    server_api = "http://localhost:4000"
    server_identity = "http://localhost:33656"
    state_file = "~/.bws/test-state-file.txt"

    Each state file will have a json like this...
    Version: 1
    List of access tokens and client secrets mapped to

    service_account_id.client_secret -> maps to some encrypted data

    encrypted data will be the refresh token

    For now... try to re-auth without hitting connect/token using basic state


    A different file per profile

    Each file contains all the access tokens used under that profile

    Profile

    -----



    #[derive(Serialize, Deserialize)]
    struct Test {
        name: String,
        age: i32,
    }

    // // -----
    // let path = Path::new("test.txt");
    // let mut state = StateManager::new(path)?;
    // println!("----- <loaded contents> -----");
    // println!("{:?}", state);

    // // edit state
    // let t = Test { name: String::from("Colton"), age: 42};
    // state.data = json!(["an", "array", "bro"]);
    // state.data = json!(t);
    // // end

    // println!("----- <save> -----");
    // println!("{:?}", state.save(path));

    // println!("----- <saved contents> -----");
    // println!("{:?}", state);
    // // -----



    ??

    // TODO: Add state manager stuff here
    //let mut state = StateManager::new();

    // Load session or return if no session exists
    // let _ = client
    //     .access_token_login(&AccessTokenLoginRequest { access_token })
    //     .await?;
*/
