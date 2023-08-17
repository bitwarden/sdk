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
- req: [PasswordHashRequest](#passwordhashrequest)

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
- req: [CipherEncryptRequest](#cipherencryptrequest)

**Output**: std::result::Result<CipherEncryptResponse,BitwardenError>

### `decrypt`
Decrypt cipher

**Arguments**:
- self: 
- req: [CipherDecryptRequest](#cipherdecryptrequest)

**Output**: std::result::Result<CipherDecryptResponse,BitwardenError>

### `decrypt_list`
Decrypt cipher list

**Arguments**:
- self: 
- req: [CipherDecryptListRequest](#cipherdecryptlistrequest)

**Output**: std::result::Result<CipherDecryptListResponse,BitwardenError>


## ClientCollections

### `decrypt`
Decrypt collection

**Arguments**:
- self: 
- req: [CollectionDecryptRequest](#collectiondecryptrequest)

**Output**: std::result::Result<CollectionDecryptResponse,BitwardenError>

### `decrypt_list`
Decrypt collection list

**Arguments**:
- self: 
- req: [CollectionDecryptListRequest](#collectiondecryptlistrequest)

**Output**: std::result::Result<CollectionDecryptListResponse,BitwardenError>


## ClientFolders

### `encrypt`
Encrypt folder

**Arguments**:
- self: 
- req: [FolderEncryptRequest](#folderencryptrequest)

**Output**: std::result::Result<FolderEncryptResponse,BitwardenError>

### `decrypt`
Decrypt folder

**Arguments**:
- self: 
- req: [FolderDecryptRequest](#folderdecryptrequest)

**Output**: std::result::Result<FolderDecryptResponse,BitwardenError>

### `decrypt_list`
Decrypt folder list

**Arguments**:
- self: 
- req: [FolderDecryptListRequest](#folderdecryptlistrequest)

**Output**: std::result::Result<FolderDecryptListResponse,BitwardenError>


## ClientPasswordHistory

### `encrypt`
Encrypt password history

**Arguments**:
- self: 
- req: [PasswordHistoryEncryptRequest](#passwordhistoryencryptrequest)

**Output**: std::result::Result<PasswordHistoryEncryptResponse,BitwardenError>

### `decrypt_list`
Decrypt password history

**Arguments**:
- self: 
- req: [PasswordHistoryDecryptListRequest](#passwordhistorydecryptlistrequest)

**Output**: std::result::Result<PasswordHistoryDecryptListResponse,BitwardenError>


# Command references

Command references are generated from the JSON schemas and should mostly match the kotlin and swift
implementations.


## `CipherDecryptListRequest`


<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>ciphers</th>
    <th>array</th>
    <th></th>
</tr>
</table>


## `CipherDecryptRequest`


<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>cipher</th>
    <th></th>
    <th></th>
</tr>
</table>


## `CipherEncryptRequest`


<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>cipher</th>
    <th></th>
    <th></th>
</tr>
</table>


## `CollectionDecryptListRequest`


<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>collections</th>
    <th>array</th>
    <th></th>
</tr>
</table>


## `CollectionDecryptRequest`


<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>collection</th>
    <th></th>
    <th></th>
</tr>
</table>


## `FolderDecryptListRequest`


<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>folders</th>
    <th>array</th>
    <th></th>
</tr>
</table>


## `FolderDecryptRequest`


<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>folder</th>
    <th></th>
    <th></th>
</tr>
</table>


## `FolderEncryptRequest`


<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>folder</th>
    <th></th>
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




## `PasswordHashRequest`


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
</table>


## `PasswordHistoryDecryptListRequest`


<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>history</th>
    <th></th>
    <th></th>
</tr>
</table>


## `PasswordHistoryEncryptRequest`


<table>
<tr>
    <th>Key</th>
    <th>Type</th>
    <th>Description</th>
</tr>
<tr>
    <th>history</th>
    <th></th>
    <th></th>
</tr>
</table>


