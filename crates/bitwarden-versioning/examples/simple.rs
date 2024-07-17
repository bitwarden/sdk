use bitwarden_crypto::{CryptoError, SymmetricCryptoKey};
use bitwarden_versioning::{Migrator, Versioned};

#[derive(Clone, Debug)]
struct DataV1(u32);

#[derive(Clone, Debug)]
struct DataV2 {
    #[allow(dead_code)]
    value: String,
}

#[derive(Debug)]
enum Data {
    V1(DataV1),
    #[allow(dead_code)]
    V2(DataV2),
}

impl Migrator<DataV2> for Data {
    fn migrate(&self, _key: &SymmetricCryptoKey) -> Result<DataV2, CryptoError> {
        match self {
            Data::V1(DataV1(value)) => Ok(DataV2 {
                value: value.to_string(),
            }),
            Data::V2(data) => Ok(data.clone()),
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    example().await.unwrap();
}

async fn example() -> Result<(), CryptoError> {
    let input = Data::V1(DataV1(42));

    println!("Input: {:?}", input);

    let versioned = Versioned::new(input);
    let key = SymmetricCryptoKey::try_from(
        "UY4B5N4DA4UisCNClgZtRr6VLy9ZF5BXXC7cDZRqourKi4ghEMgISbCsubvgCkHf5DZctQjVot11/vVvN9NNHQ=="
            .to_owned(),
    )
    .unwrap();
    let output: DataV2 = versioned.migrate(&key)?;

    println!("Output: {:?}", output);

    Ok(())
}
