# Bitwarden Mobile SDK

Auto generated documentation for the Bitwarden Mobile SDK. For more information please refer to the
rust crates `bitwarden` and `bitwarden-uniffi`. For code samples check the `languages/kotlin/app`
and `languages/swift/app` directories.

## Client

### `new`

Initialize a new instance of the SDK client

**Arguments**:

- settings: Option<ClientSettings>

**Output**: Arc<Self>

### `crypto`

Crypto operations

**Arguments**:

- self: Arc<Self>

**Output**: Arc<ClientCrypto>

### `vault`

Vault item operations

**Arguments**:

- self: Arc<Self>

**Output**: Arc<ClientVault>

### `generators`

Generator operations

**Arguments**:

- self: Arc<Self>

**Output**: Arc<ClientGenerators>

### `auth`

Auth operations

**Arguments**:

- self: Arc<Self>

**Output**: Arc<ClientAuth>

### `echo`

Test method, echoes back the input

**Arguments**:

- self:
- msg: String

**Output**: String

## ClientAuth

### `password_strength`

**API Draft:** Calculate Password Strength

**Arguments**:

- self:
- password: String
- email: String
- additional_inputs: Vec<String>

**Output**:

### `satisfies_policy`

**API Draft:** Evaluate if the provided password satisfies the provided policy

**Arguments**:

- self:
- password: String
- strength:
- policy: [MasterPasswordPolicyOptions](#masterpasswordpolicyoptions)

**Output**:

### `hash_password`

Hash the user password

**Arguments**:

- self:
- email: String
- password: String
- kdf_params: [Kdf](#kdf)

**Output**: std::result::Result<String,BitwardenError>

### `make_register_keys`

Generate keys needed for registration process

**Arguments**:

- self:
- email: String
- password: String
- kdf: [Kdf](#kdf)

**Output**: std::result::Result<RegisterKeyResponse,BitwardenError>

## ClientCiphers

### `encrypt`

Encrypt cipher

**Arguments**:

- self:
- cipher_view: [CipherView](#cipherview)

**Output**: std::result::Result<Cipher,BitwardenError>

### `decrypt`

Decrypt cipher

**Arguments**:

- self:
- cipher: [Cipher](#cipher)

**Output**: std::result::Result<CipherView,BitwardenError>

### `decrypt_list`

Decrypt cipher list

**Arguments**:

- self:
- ciphers: Vec<Cipher>

**Output**: std::result::Result<Vec,BitwardenError>

## ClientCollections

### `decrypt`

Decrypt collection

**Arguments**:

- self:
- collection: [Collection](#collection)

**Output**: std::result::Result<CollectionView,BitwardenError>

### `decrypt_list`

Decrypt collection list

**Arguments**:

- self:
- collections: Vec<Collection>

**Output**: std::result::Result<Vec,BitwardenError>

## ClientCrypto

### `initialize_user_crypto`

Initialization method for the user crypto. Needs to be called before any other crypto operations.

**Arguments**:

- self:
- req: [InitUserCryptoRequest](#initusercryptorequest)

**Output**: std::result::Result<,BitwardenError>

### `initialize_org_crypto`

Initialization method for the organization crypto. Needs to be called after
&#x60;initialize_user_crypto&#x60; but before any other crypto operations.

**Arguments**:

- self:
- req: [InitOrgCryptoRequest](#initorgcryptorequest)

**Output**: std::result::Result<,BitwardenError>

### `get_user_encryption_key`

Get the uses&#x27;s decrypted encryption key. Note: It&#x27;s very important to keep this key safe,
as it can be used to decrypt all of the user&#x27;s data

**Arguments**:

- self:

**Output**: std::result::Result<String,BitwardenError>

## ClientExporters

### `export_vault`

**API Draft:** Export user vault

**Arguments**:

- self:
- folders: Vec<Folder>
- ciphers: Vec<Cipher>
- format: [ExportFormat](#exportformat)

**Output**: std::result::Result<String,BitwardenError>

### `export_organization_vault`

**API Draft:** Export organization vault

**Arguments**:

- self:
- collections: Vec<Collection>
- ciphers: Vec<Cipher>
- format: [ExportFormat](#exportformat)

**Output**: std::result::Result<String,BitwardenError>

## ClientFolders

### `encrypt`

Encrypt folder

**Arguments**:

- self:
- folder: [FolderView](#folderview)

**Output**: std::result::Result<Folder,BitwardenError>

### `decrypt`

Decrypt folder

**Arguments**:

- self:
- folder: [Folder](#folder)

**Output**: std::result::Result<FolderView,BitwardenError>

### `decrypt_list`

Decrypt folder list

**Arguments**:

- self:
- folders: Vec<Folder>

**Output**: std::result::Result<Vec,BitwardenError>

## ClientGenerators

### `password`

**API Draft:** Generate Password

**Arguments**:

- self:
- settings: [PasswordGeneratorRequest](#passwordgeneratorrequest)

**Output**: std::result::Result<String,BitwardenError>

### `passphrase`

**API Draft:** Generate Passphrase

**Arguments**:

- self:
- settings: [PassphraseGeneratorRequest](#passphrasegeneratorrequest)

**Output**: std::result::Result<String,BitwardenError>

## ClientPasswordHistory

### `encrypt`

Encrypt password history

**Arguments**:

- self:
- password_history: [PasswordHistoryView](#passwordhistoryview)

**Output**: std::result::Result<PasswordHistory,BitwardenError>

### `decrypt_list`

Decrypt password history

**Arguments**:

- self:
- list: Vec<PasswordHistory>

**Output**: std::result::Result<Vec,BitwardenError>

## ClientSends

### `encrypt`

Encrypt send

**Arguments**:

- self:
- send: [SendView](#sendview)

**Output**: std::result::Result<Send,BitwardenError>

### `encrypt_buffer`

Encrypt a send file in memory

**Arguments**:

- self:
- send: [Send](#send)
- buffer: Vec<>

**Output**: std::result::Result<Vec,BitwardenError>

### `encrypt_file`

Encrypt a send file located in the file system

**Arguments**:

- self:
- send: [Send](#send)
- decrypted_file_path: String
- encrypted_file_path: String

**Output**: std::result::Result<,BitwardenError>

### `decrypt`

Decrypt send

**Arguments**:

- self:
- send: [Send](#send)

**Output**: std::result::Result<SendView,BitwardenError>

### `decrypt_list`

Decrypt send list

**Arguments**:

- self:
- sends: Vec<Send>

**Output**: std::result::Result<Vec,BitwardenError>

### `decrypt_buffer`

Decrypt a send file in memory

**Arguments**:

- self:
- send: [Send](#send)
- buffer: Vec<>

**Output**: std::result::Result<Vec,BitwardenError>

### `decrypt_file`

Decrypt a send file located in the file system

**Arguments**:

- self:
- send: [Send](#send)
- encrypted_file_path: String
- decrypted_file_path: String

**Output**: std::result::Result<,BitwardenError>

## ClientVault

### `folders`

Folder operations

**Arguments**:

- self: Arc<Self>

**Output**: Arc<folders::ClientFolders>

### `collections`

Collections operations

**Arguments**:

- self: Arc<Self>

**Output**: Arc<collections::ClientCollections>

### `ciphers`

Ciphers operations

**Arguments**:

- self: Arc<Self>

**Output**: Arc<ciphers::ClientCiphers>

### `password_history`

Password history operations

**Arguments**:

- self: Arc<Self>

**Output**: Arc<password_history::ClientPasswordHistory>

### `sends`

Sends operations

**Arguments**:

- self: Arc<Self>

**Output**: Arc<sends::ClientSends>

### `generate_totp`

Generate a TOTP code from a provided key.

The key can be either:

- A base32 encoded string
- OTP Auth URI
- Steam URI

**Arguments**:

- self:
- key: String
- time: Option<DateTime>

**Output**: [TotpResponse](#totpresponse)

# References

References are generated from the JSON schemas and should mostly match the kotlin and swift
implementations.

## `Cipher`

<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>id</th>
    <th>string,null</th>
    <th></th>
</tr>
<tr>
    <th>organizationId</th>
    <th>string,null</th>
    <th></th>
</tr>
<tr>
    <th>folderId</th>
    <th>string,null</th>
    <th></th>
</tr>
<tr>
    <th>collectionIds</th>
    <th>array</th>
    <th></th>
</tr>
<tr>
    <th>key</th>
    <th></th>
    <th>More recent ciphers uses individual encryption keys to encrypt the other fields of the Cipher.</th>
</tr>
<tr>
    <th>name</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>notes</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>type</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>login</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>identity</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>card</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>secureNote</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>favorite</th>
    <th>boolean</th>
    <th></th>
</tr>
<tr>
    <th>reprompt</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>organizationUseTotp</th>
    <th>boolean</th>
    <th></th>
</tr>
<tr>
    <th>edit</th>
    <th>boolean</th>
    <th></th>
</tr>
<tr>
    <th>viewPassword</th>
    <th>boolean</th>
    <th></th>
</tr>
<tr>
    <th>localData</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>attachments</th>
    <th>array,null</th>
    <th></th>
</tr>
<tr>
    <th>fields</th>
    <th>array,null</th>
    <th></th>
</tr>
<tr>
    <th>passwordHistory</th>
    <th>array,null</th>
    <th></th>
</tr>
<tr>
    <th>creationDate</th>
    <th>string</th>
    <th></th>
</tr>
<tr>
    <th>deletedDate</th>
    <th>string,null</th>
    <th></th>
</tr>
<tr>
    <th>revisionDate</th>
    <th>string</th>
    <th></th>
</tr>
</table>

## `CipherView`

<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>id</th>
    <th>string,null</th>
    <th></th>
</tr>
<tr>
    <th>organizationId</th>
    <th>string,null</th>
    <th></th>
</tr>
<tr>
    <th>folderId</th>
    <th>string,null</th>
    <th></th>
</tr>
<tr>
    <th>collectionIds</th>
    <th>array</th>
    <th></th>
</tr>
<tr>
    <th>key</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>name</th>
    <th>string</th>
    <th></th>
</tr>
<tr>
    <th>notes</th>
    <th>string,null</th>
    <th></th>
</tr>
<tr>
    <th>type</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>login</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>identity</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>card</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>secureNote</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>favorite</th>
    <th>boolean</th>
    <th></th>
</tr>
<tr>
    <th>reprompt</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>organizationUseTotp</th>
    <th>boolean</th>
    <th></th>
</tr>
<tr>
    <th>edit</th>
    <th>boolean</th>
    <th></th>
</tr>
<tr>
    <th>viewPassword</th>
    <th>boolean</th>
    <th></th>
</tr>
<tr>
    <th>localData</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>attachments</th>
    <th>array,null</th>
    <th></th>
</tr>
<tr>
    <th>fields</th>
    <th>array,null</th>
    <th></th>
</tr>
<tr>
    <th>passwordHistory</th>
    <th>array,null</th>
    <th></th>
</tr>
<tr>
    <th>creationDate</th>
    <th>string</th>
    <th></th>
</tr>
<tr>
    <th>deletedDate</th>
    <th>string,null</th>
    <th></th>
</tr>
<tr>
    <th>revisionDate</th>
    <th>string</th>
    <th></th>
</tr>
</table>

## `Collection`

<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>id</th>
    <th>string</th>
    <th></th>
</tr>
<tr>
    <th>organizationId</th>
    <th>string</th>
    <th></th>
</tr>
<tr>
    <th>name</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>externalId</th>
    <th>string,null</th>
    <th></th>
</tr>
<tr>
    <th>hidePasswords</th>
    <th>boolean</th>
    <th></th>
</tr>
<tr>
    <th>readOnly</th>
    <th>boolean</th>
    <th></th>
</tr>
</table>

## `ExportFormat`

<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>EncryptedJson</th>
    <th>object</th>
    <th></th>
</tr>
<tr>
    <td colspan="3">
        <table>
        <tr>
            <th>Key</th>
            <th>Type</th>
            <th>Description</th>
        </tr>
            <tr>
                <td>password</td>
                <td>string</td>
                <td></td>
            </tr>
        </table>
    </td>
</tr>
</table>

## `Folder`

<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>id</th>
    <th>string</th>
    <th></th>
</tr>
<tr>
    <th>name</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>revisionDate</th>
    <th>string</th>
    <th></th>
</tr>
</table>

## `FolderView`

<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>id</th>
    <th>string</th>
    <th></th>
</tr>
<tr>
    <th>name</th>
    <th>string</th>
    <th></th>
</tr>
<tr>
    <th>revisionDate</th>
    <th>string</th>
    <th></th>
</tr>
</table>

## `InitOrgCryptoRequest`

<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>organizationKeys</th>
    <th>object</th>
    <th>The encryption keys for all the organizations the user is a part of</th>
</tr>
</table>

## `InitUserCryptoMethod`

<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>password</th>
    <th>object</th>
    <th></th>
</tr>
<tr>
    <td colspan="3">
        <table>
        <tr>
            <th>Key</th>
            <th>Type</th>
            <th>Description</th>
        </tr>
            <tr>
                <td>password</td>
                <td>string</td>
                <td>The user's master password</td>
            </tr>
            <tr>
                <td>user_key</td>
                <td>string</td>
                <td>The user's encrypted symmetric crypto key</td>
            </tr>
        </table>
    </td>
</tr>
<tr>
    <th>decryptedKey</th>
    <th>object</th>
    <th></th>
</tr>
<tr>
    <td colspan="3">
        <table>
        <tr>
            <th>Key</th>
            <th>Type</th>
            <th>Description</th>
        </tr>
            <tr>
                <td>decrypted_user_key</td>
                <td>string</td>
                <td>The user's decrypted encryption key, obtained using `get_user_encryption_key`</td>
            </tr>
        </table>
    </td>
</tr>
</table>

## `InitUserCryptoRequest`

<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>kdfParams</th>
    <th></th>
    <th>The user&#x27;s KDF parameters, as received from the prelogin request</th>
</tr>
<tr>
    <th>email</th>
    <th>string</th>
    <th>The user&#x27;s email address</th>
</tr>
<tr>
    <th>privateKey</th>
    <th>string</th>
    <th>The user&#x27;s encrypted private key</th>
</tr>
<tr>
    <th>method</th>
    <th></th>
    <th>The initialization method to use</th>
</tr>
</table>

## `Kdf`

<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>pBKDF2</th>
    <th>object</th>
    <th></th>
</tr>
<tr>
    <td colspan="3">
        <table>
        <tr>
            <th>Key</th>
            <th>Type</th>
            <th>Description</th>
        </tr>
            <tr>
                <td>iterations</td>
                <td>integer</td>
                <td></td>
            </tr>
        </table>
    </td>
</tr>
<tr>
    <th>argon2id</th>
    <th>object</th>
    <th></th>
</tr>
<tr>
    <td colspan="3">
        <table>
        <tr>
            <th>Key</th>
            <th>Type</th>
            <th>Description</th>
        </tr>
            <tr>
                <td>iterations</td>
                <td>integer</td>
                <td></td>
            </tr>
            <tr>
                <td>memory</td>
                <td>integer</td>
                <td></td>
            </tr>
            <tr>
                <td>parallelism</td>
                <td>integer</td>
                <td></td>
            </tr>
        </table>
    </td>
</tr>
</table>

## `MasterPasswordPolicyOptions`

<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>min_complexity</th>
    <th>integer</th>
    <th></th>
</tr>
<tr>
    <th>min_length</th>
    <th>integer</th>
    <th></th>
</tr>
<tr>
    <th>require_upper</th>
    <th>boolean</th>
    <th></th>
</tr>
<tr>
    <th>require_lower</th>
    <th>boolean</th>
    <th></th>
</tr>
<tr>
    <th>require_numbers</th>
    <th>boolean</th>
    <th></th>
</tr>
<tr>
    <th>require_special</th>
    <th>boolean</th>
    <th></th>
</tr>
<tr>
    <th>enforce_on_login</th>
    <th>boolean</th>
    <th>Flag to indicate if the policy should be enforced on login. If true, and the user&#x27;s password does not meet the policy requirements, the user will be forced to update their password.</th>
</tr>
</table>

## `PassphraseGeneratorRequest`

<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>numWords</th>
    <th>integer,null</th>
    <th></th>
</tr>
<tr>
    <th>wordSeparator</th>
    <th>string,null</th>
    <th></th>
</tr>
<tr>
    <th>capitalize</th>
    <th>boolean,null</th>
    <th></th>
</tr>
<tr>
    <th>includeNumber</th>
    <th>boolean,null</th>
    <th></th>
</tr>
</table>

## `PasswordGeneratorRequest`

<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>lowercase</th>
    <th>boolean</th>
    <th></th>
</tr>
<tr>
    <th>uppercase</th>
    <th>boolean</th>
    <th></th>
</tr>
<tr>
    <th>numbers</th>
    <th>boolean</th>
    <th></th>
</tr>
<tr>
    <th>special</th>
    <th>boolean</th>
    <th></th>
</tr>
<tr>
    <th>length</th>
    <th>integer,null</th>
    <th></th>
</tr>
<tr>
    <th>avoidAmbiguous</th>
    <th>boolean,null</th>
    <th></th>
</tr>
<tr>
    <th>minLowercase</th>
    <th>boolean,null</th>
    <th></th>
</tr>
<tr>
    <th>minUppercase</th>
    <th>boolean,null</th>
    <th></th>
</tr>
<tr>
    <th>minNumber</th>
    <th>boolean,null</th>
    <th></th>
</tr>
<tr>
    <th>minSpecial</th>
    <th>boolean,null</th>
    <th></th>
</tr>
</table>

## `PasswordHistoryView`

<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>password</th>
    <th>string</th>
    <th></th>
</tr>
<tr>
    <th>lastUsedDate</th>
    <th>string</th>
    <th></th>
</tr>
</table>

## `Send`

<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>id</th>
    <th>string</th>
    <th></th>
</tr>
<tr>
    <th>accessId</th>
    <th>string</th>
    <th></th>
</tr>
<tr>
    <th>name</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>notes</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>key</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>password</th>
    <th>string,null</th>
    <th></th>
</tr>
<tr>
    <th>type</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>file</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>text</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>maxAccessCount</th>
    <th>integer,null</th>
    <th></th>
</tr>
<tr>
    <th>accessCount</th>
    <th>integer</th>
    <th></th>
</tr>
<tr>
    <th>disabled</th>
    <th>boolean</th>
    <th></th>
</tr>
<tr>
    <th>hideEmail</th>
    <th>boolean</th>
    <th></th>
</tr>
<tr>
    <th>revisionDate</th>
    <th>string</th>
    <th></th>
</tr>
<tr>
    <th>deletionDate</th>
    <th>string</th>
    <th></th>
</tr>
<tr>
    <th>expirationDate</th>
    <th>string,null</th>
    <th></th>
</tr>
</table>

## `SendView`

<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>id</th>
    <th>string</th>
    <th></th>
</tr>
<tr>
    <th>accessId</th>
    <th>string</th>
    <th></th>
</tr>
<tr>
    <th>name</th>
    <th>string</th>
    <th></th>
</tr>
<tr>
    <th>notes</th>
    <th>string,null</th>
    <th></th>
</tr>
<tr>
    <th>key</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>password</th>
    <th>string,null</th>
    <th></th>
</tr>
<tr>
    <th>type</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>file</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>text</th>
    <th></th>
    <th></th>
</tr>
<tr>
    <th>maxAccessCount</th>
    <th>integer,null</th>
    <th></th>
</tr>
<tr>
    <th>accessCount</th>
    <th>integer</th>
    <th></th>
</tr>
<tr>
    <th>disabled</th>
    <th>boolean</th>
    <th></th>
</tr>
<tr>
    <th>hideEmail</th>
    <th>boolean</th>
    <th></th>
</tr>
<tr>
    <th>revisionDate</th>
    <th>string</th>
    <th></th>
</tr>
<tr>
    <th>deletionDate</th>
    <th>string</th>
    <th></th>
</tr>
<tr>
    <th>expirationDate</th>
    <th>string,null</th>
    <th></th>
</tr>
</table>

## `TotpResponse`

<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>code</th>
    <th>string</th>
    <th>Generated TOTP code</th>
</tr>
<tr>
    <th>period</th>
    <th>integer</th>
    <th>Time period</th>
</tr>
</table>
