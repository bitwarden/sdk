struct PersistedState {
    version: u16,
    data: &'a RawValue,
}

struct ContentV1 {
    profile: ProfileV1,
    vault: VaultV1,
}

struct ProfileV1 {
    email: String,
    name: String,
}

struct VaultV1 {
    folders: Vec<FolderV1>,
    collections: Vec<CollectionV1>,
    items: Vec<ItemV1>,
    ciphers: Vec<CipherV1>,
    attachments: Vec<AttachmentV1>,
}

struct ContentV2 {
    profile: ProfileV2,
    vault: VaultV1,
}

struct ProfileV2 {
    email: String,
    first_name: String,
    last_name: String,
}

impl From<ProfileV1> for ProfileV2 {
    fn from(profile: ProfileV1) -> Self {
        let mut name = profile.name.split(' ');
        ProfileV2 {
            email: profile.email,
            first_name: name.next().unwrap_or("").to_owned(),
            last_name: name.next().unwrap_or("").to_owned(),
        }
    }
}

impl From<ContentV1> for ContentV2 {
    fn from(content: ContentV1) -> Self {
        ContentV2 {
            profile: content.profile.into(),
            vault: content.vault,
        }
    }
}
