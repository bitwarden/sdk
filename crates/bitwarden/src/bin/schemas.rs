/*
use std::{fs::{self, create_dir_all}, path::Path};

use bitwarden::sdk::{
    request::PasswordLoginRequest,
    response::{captcha_response::CaptchaResponse, password_login_response::PasswordLoginResponse},
};
use schemars::{schema_for, JsonSchema};

fn main() {
    write_to_file::<PasswordLoginRequest>("schemas/request/password-login-request.json".into());
    write_to_file::<PasswordLoginResponse>("schemas/response/password-login-response.json".into());
    write_to_file::<CaptchaResponse>("schemas/response/captcha-response.json".into());
}

#[allow(unused_must_use)]
fn write_to_file<T: JsonSchema>(filename: String) {
    let path = Path::new(&filename);
    create_dir_all(path.parent().unwrap());

    let schema = schema_for!(T);
    fs::write(filename, serde_json::to_string_pretty(&schema).unwrap()).unwrap();
}
*/

fn main() {}
