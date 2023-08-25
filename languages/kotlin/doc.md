# Bitwarden Mobile SDK

Auto generated documentation for the Bitwarden Mobile SDK. For more information please refer to
the rust crates `bitwarden` and `bitwarden-uniffi`. For code samples check the
`languages/kotlin/app` and `languages/swift/app` directories.

## Client

### `new`

Initialize a new instance of the SDK client

**Arguments**:

- settings: Option<ClientSettings>

**Output**: Arc<Self>

### `kdf`

KDF operations

**Arguments**:

- self: Arc<Self>

**Output**: Arc<ClientKdf>

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

### `echo`

Test method, echoes back the input

**Arguments**:

- self:
- msg: String

**Output**: String

## ClientKdf

### `hash_password`

Hash the user password

**Arguments**:

- self:
- email: String
- password: String
- kdf_params: [Kdf](#kdf)

**Output**: std::result::Result<String,BitwardenError>

## ClientCrypto

### `initialize_crypto`

Initialization method for the crypto. Needs to be called before any other crypto operations.

**Arguments**:

- self:
- req: [InitCryptoRequest](#initcryptorequest)

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

Ciphers operations

**Arguments**:

- self: Arc<Self>

**Output**: Arc<password_history::ClientPasswordHistory>

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

## ClientGenerators

### `password`

Generate Password

**Arguments**:

- self:
- settings: PasswordGeneratorRequest

**Output**: std::result::Result<String,BitwardenError>

### `passphrase`

Generate Passphrase

**Arguments**:

- self:
- settings: PassphraseGeneratorRequest

**Output**: std::result::Result<String,BitwardenError>

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
    <th>array</th>
    <th></th>
</tr>
<tr>
    <th>fields</th>
    <th>array</th>
    <th></th>
</tr>
<tr>
    <th>passwordHistory</th>
    <th>array</th>
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
    <th>name</th>
    <th>string</th>
    <th></th>
</tr>
<tr>
    <th>notes</th>
    <th>string</th>
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
    <th>array</th>
    <th></th>
</tr>
<tr>
    <th>fields</th>
    <th>array</th>
    <th></th>
</tr>
<tr>
    <th>passwordHistory</th>
    <th>array</th>
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

## `InitCryptoRequest`

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
    <th>password</th>
    <th>string</th>
    <th>The user&#x27;s master password</th>
</tr>
<tr>
    <th>userKey</th>
    <th>string</th>
    <th>The user&#x27;s encrypted symmetric crypto key</th>
</tr>
<tr>
    <th>privateKey</th>
    <th>string</th>
    <th>The user&#x27;s encryptred private key</th>
</tr>
<tr>
    <th>organizationKeys</th>
    <th>object</th>
    <th>The encryption keys for all the organizations the user is a part of</th>
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
