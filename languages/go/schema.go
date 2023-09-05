// This file was generated from JSON Schema using quicktype, do not modify it directly.
// To parse and unparse this JSON data, add this code to your project and do:
//
//    clientSettings, err := UnmarshalClientSettings(bytes)
//    bytes, err = clientSettings.Marshal()
//
//    command, err := UnmarshalCommand(bytes)
//    bytes, err = command.Marshal()
//
//    docRef, err := UnmarshalDocRef(bytes)
//    bytes, err = docRef.Marshal()
//
//    responseForAPIKeyLoginResponse, err := UnmarshalResponseForAPIKeyLoginResponse(bytes)
//    bytes, err = responseForAPIKeyLoginResponse.Marshal()
//
//    responseForFingerprintResponse, err := UnmarshalResponseForFingerprintResponse(bytes)
//    bytes, err = responseForFingerprintResponse.Marshal()
//
//    responseForPasswordLoginResponse, err := UnmarshalResponseForPasswordLoginResponse(bytes)
//    bytes, err = responseForPasswordLoginResponse.Marshal()
//
//    responseForProjectResponse, err := UnmarshalResponseForProjectResponse(bytes)
//    bytes, err = responseForProjectResponse.Marshal()
//
//    responseForProjectsDeleteResponse, err := UnmarshalResponseForProjectsDeleteResponse(bytes)
//    bytes, err = responseForProjectsDeleteResponse.Marshal()
//
//    responseForProjectsResponse, err := UnmarshalResponseForProjectsResponse(bytes)
//    bytes, err = responseForProjectsResponse.Marshal()
//
//    responseForSecretIdentifiersResponse, err := UnmarshalResponseForSecretIdentifiersResponse(bytes)
//    bytes, err = responseForSecretIdentifiersResponse.Marshal()
//
//    responseForSecretResponse, err := UnmarshalResponseForSecretResponse(bytes)
//    bytes, err = responseForSecretResponse.Marshal()
//
//    responseForSecretsDeleteResponse, err := UnmarshalResponseForSecretsDeleteResponse(bytes)
//    bytes, err = responseForSecretsDeleteResponse.Marshal()
//
//    responseForSecretsResponse, err := UnmarshalResponseForSecretsResponse(bytes)
//    bytes, err = responseForSecretsResponse.Marshal()
//
//    responseForSyncResponse, err := UnmarshalResponseForSyncResponse(bytes)
//    bytes, err = responseForSyncResponse.Marshal()
//
//    responseForUserAPIKeyResponse, err := UnmarshalResponseForUserAPIKeyResponse(bytes)
//    bytes, err = responseForUserAPIKeyResponse.Marshal()

package main

import "bytes"
import "errors"
import "encoding/json"

func UnmarshalClientSettings(data []byte) (ClientSettings, error) {
	var r ClientSettings
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *ClientSettings) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalCommand(data []byte) (Command, error) {
	var r Command
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *Command) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalDocRef(data []byte) (DocRef, error) {
	var r DocRef
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *DocRef) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalResponseForAPIKeyLoginResponse(data []byte) (ResponseForAPIKeyLoginResponse, error) {
	var r ResponseForAPIKeyLoginResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *ResponseForAPIKeyLoginResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalResponseForFingerprintResponse(data []byte) (ResponseForFingerprintResponse, error) {
	var r ResponseForFingerprintResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *ResponseForFingerprintResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalResponseForPasswordLoginResponse(data []byte) (ResponseForPasswordLoginResponse, error) {
	var r ResponseForPasswordLoginResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *ResponseForPasswordLoginResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalResponseForProjectResponse(data []byte) (ResponseForProjectResponse, error) {
	var r ResponseForProjectResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *ResponseForProjectResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalResponseForProjectsDeleteResponse(data []byte) (ResponseForProjectsDeleteResponse, error) {
	var r ResponseForProjectsDeleteResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *ResponseForProjectsDeleteResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalResponseForProjectsResponse(data []byte) (ResponseForProjectsResponse, error) {
	var r ResponseForProjectsResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *ResponseForProjectsResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalResponseForSecretIdentifiersResponse(data []byte) (ResponseForSecretIdentifiersResponse, error) {
	var r ResponseForSecretIdentifiersResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *ResponseForSecretIdentifiersResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalResponseForSecretResponse(data []byte) (ResponseForSecretResponse, error) {
	var r ResponseForSecretResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *ResponseForSecretResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalResponseForSecretsDeleteResponse(data []byte) (ResponseForSecretsDeleteResponse, error) {
	var r ResponseForSecretsDeleteResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *ResponseForSecretsDeleteResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalResponseForSecretsResponse(data []byte) (ResponseForSecretsResponse, error) {
	var r ResponseForSecretsResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *ResponseForSecretsResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalResponseForSyncResponse(data []byte) (ResponseForSyncResponse, error) {
	var r ResponseForSyncResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *ResponseForSyncResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

func UnmarshalResponseForUserAPIKeyResponse(data []byte) (ResponseForUserAPIKeyResponse, error) {
	var r ResponseForUserAPIKeyResponse
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *ResponseForUserAPIKeyResponse) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

// Basic client behavior settings. These settings specify the various targets and behavior
// of the Bitwarden Client. They are optional and uneditable once the client is
// initialized.
//
// Defaults to
//
// ``` # use bitwarden::client::client_settings::{ClientSettings, DeviceType}; # use
// assert_matches::assert_matches; let settings = ClientSettings { identity_url:
// "https://identity.bitwarden.com".to_string(), api_url:
// "https://api.bitwarden.com".to_string(), user_agent: "Bitwarden Rust-SDK".to_string(),
// device_type: DeviceType::SDK, }; let default = ClientSettings::default();
// assert_matches!(settings, default); ```
//
// Targets `localhost:8080` for debug builds.
type ClientSettings struct {
	// The api url of the targeted Bitwarden instance. Defaults to `https://api.bitwarden.com`           
	APIURL                                                                                    string     `json:"apiUrl"`
	// Device type to send to Bitwarden. Defaults to SDK                                                 
	DeviceType                                                                                DeviceType `json:"deviceType"`
	// The identity url of the targeted Bitwarden instance. Defaults to                                  
	// `https://identity.bitwarden.com`                                                                  
	IdentityURL                                                                               string     `json:"identityUrl"`
	// The user_agent to sent to Bitwarden. Defaults to `Bitwarden Rust-SDK`                             
	UserAgent                                                                                 string     `json:"userAgent"`
}

// Login with username and password
//
// This command is for initiating an authentication handshake with Bitwarden. Authorization
// may fail due to requiring 2fa or captcha challenge completion despite accurate
// credentials.
//
// This command is not capable of handling authentication requiring 2fa or captcha.
//
// Returns: [PasswordLoginResponse](bitwarden::auth::login::PasswordLoginResponse)
//
// Login with API Key
//
// This command is for initiating an authentication handshake with Bitwarden.
//
// Returns: [ApiKeyLoginResponse](bitwarden::auth::login::ApiKeyLoginResponse)
//
// Login with Secrets Manager Access Token
//
// This command is for initiating an authentication handshake with Bitwarden.
//
// Returns: [ApiKeyLoginResponse](bitwarden::auth::login::ApiKeyLoginResponse)
//
// > Requires Authentication Get the API key of the currently authenticated user
//
// Returns: [UserApiKeyResponse](bitwarden::platform::UserApiKeyResponse)
//
// Get the user's passphrase
//
// Returns: String
//
// > Requires Authentication Retrieve all user data, ciphers and organizations the user is a
// part of
//
// Returns: [SyncResponse](bitwarden::platform::SyncResponse)
type Command struct {
	PasswordLogin    *PasswordLoginRequest      `json:"passwordLogin,omitempty"`
	APIKeyLogin      *APIKeyLoginRequest        `json:"apiKeyLogin,omitempty"`
	AccessTokenLogin *AccessTokenLoginRequest   `json:"accessTokenLogin,omitempty"`
	GetUserAPIKey    *SecretVerificationRequest `json:"getUserApiKey,omitempty"`
	Fingerprint      *FingerprintRequest        `json:"fingerprint,omitempty"`
	Sync             *SyncRequest               `json:"sync,omitempty"`
	Secrets          *SecretsCommand            `json:"secrets,omitempty"`
	Projects         *ProjectsCommand           `json:"projects,omitempty"`
}

// Login to Bitwarden with Api Key
type APIKeyLoginRequest struct {
	// Bitwarden account client_id             
	ClientID                            string `json:"clientId"`
	// Bitwarden account client_secret         
	ClientSecret                        string `json:"clientSecret"`
	// Bitwarden account master password       
	Password                            string `json:"password"`
}

// Login to Bitwarden with access token
type AccessTokenLoginRequest struct {
	// Bitwarden service API access token       
	AccessToken                          string `json:"accessToken"`
}

type FingerprintRequest struct {
	// The input material, used in the fingerprint generation process.       
	FingerprintMaterial                                               string `json:"fingerprintMaterial"`
	// The user's public key encoded with base64.                            
	PublicKey                                                         string `json:"publicKey"`
}

type SecretVerificationRequest struct {
	// The user's master password to use for user verification. If supplied, this will be used        
	// for verification purposes.                                                                     
	MasterPassword                                                                            *string `json:"masterPassword"`
	// Alternate user verification method through OTP. This is provided for users who have no         
	// master password due to use of Customer Managed Encryption. Must be present and valid if        
	// master_password is absent.                                                                     
	Otp                                                                                       *string `json:"otp"`
}

// Login to Bitwarden with Username and Password
type PasswordLoginRequest struct {
	// Bitwarden account email address                    
	Email                               string            `json:"email"`
	// Bitwarden account master password                  
	Password                            string            `json:"password"`
	TwoFactor                           *TwoFactorRequest `json:"twoFactor"`
}

type TwoFactorRequest struct {
	// Two-factor provider                  
	Provider              TwoFactorProvider `json:"provider"`
	// Two-factor remember                  
	Remember              bool              `json:"remember"`
	// Two-factor Token                     
	Token                 string            `json:"token"`
}

// > Requires Authentication > Requires using an Access Token for login or calling Sync at
// least once Retrieve a project by the provided identifier
//
// Returns: [ProjectResponse](bitwarden::secrets_manager::projects::ProjectResponse)
//
// > Requires Authentication > Requires using an Access Token for login or calling Sync at
// least once Creates a new project in the provided organization using the given data
//
// Returns: [ProjectResponse](bitwarden::secrets_manager::projects::ProjectResponse)
//
// > Requires Authentication > Requires using an Access Token for login or calling Sync at
// least once Lists all projects of the given organization
//
// Returns: [ProjectsResponse](bitwarden::secrets_manager::projects::ProjectsResponse)
//
// > Requires Authentication > Requires using an Access Token for login or calling Sync at
// least once Updates an existing project with the provided ID using the given data
//
// Returns: [ProjectResponse](bitwarden::secrets_manager::projects::ProjectResponse)
//
// > Requires Authentication > Requires using an Access Token for login or calling Sync at
// least once Deletes all the projects whose IDs match the provided ones
//
// Returns:
// [ProjectsDeleteResponse](bitwarden::secrets_manager::projects::ProjectsDeleteResponse)
type ProjectsCommand struct {
	Get    *ProjectGetRequest     `json:"get,omitempty"`
	Create *ProjectCreateRequest  `json:"create,omitempty"`
	List   *ProjectsListRequest   `json:"list,omitempty"`
	Update *ProjectPutRequest     `json:"update,omitempty"`
	Delete *ProjectsDeleteRequest `json:"delete,omitempty"`
}

type ProjectCreateRequest struct {
	Name                                             string `json:"name"`
	// Organization where the project will be created       
	OrganizationID                                   string `json:"organizationId"`
}

type ProjectsDeleteRequest struct {
	// IDs of the projects to delete         
	IDS                             []string `json:"ids"`
}

type ProjectGetRequest struct {
	// ID of the project to retrieve       
	ID                              string `json:"id"`
}

type ProjectsListRequest struct {
	// Organization to retrieve all the projects from       
	OrganizationID                                   string `json:"organizationId"`
}

type ProjectPutRequest struct {
	// ID of the project to modify                    
	ID                                         string `json:"id"`
	Name                                       string `json:"name"`
	// Organization ID of the project to modify       
	OrganizationID                             string `json:"organizationId"`
}

// > Requires Authentication > Requires using an Access Token for login or calling Sync at
// least once Retrieve a secret by the provided identifier
//
// Returns: [SecretResponse](bitwarden::secrets_manager::secrets::SecretResponse)
//
// > Requires Authentication > Requires using an Access Token for login or calling Sync at
// least once Retrieve secrets by the provided identifiers
//
// Returns: [SecretsResponse](bitwarden::secrets_manager::secrets::SecretsResponse)
//
// > Requires Authentication > Requires using an Access Token for login or calling Sync at
// least once Creates a new secret in the provided organization using the given data
//
// Returns: [SecretResponse](bitwarden::secrets_manager::secrets::SecretResponse)
//
// > Requires Authentication > Requires using an Access Token for login or calling Sync at
// least once Lists all secret identifiers of the given organization, to then retrieve each
// secret, use `CreateSecret`
//
// Returns:
// [SecretIdentifiersResponse](bitwarden::secrets_manager::secrets::SecretIdentifiersResponse)
//
// > Requires Authentication > Requires using an Access Token for login or calling Sync at
// least once Updates an existing secret with the provided ID using the given data
//
// Returns: [SecretResponse](bitwarden::secrets_manager::secrets::SecretResponse)
//
// > Requires Authentication > Requires using an Access Token for login or calling Sync at
// least once Deletes all the secrets whose IDs match the provided ones
//
// Returns:
// [SecretsDeleteResponse](bitwarden::secrets_manager::secrets::SecretsDeleteResponse)
type SecretsCommand struct {
	Get      *SecretGetRequest         `json:"get,omitempty"`
	GetByIDS *SecretsGetRequest        `json:"getByIds,omitempty"`
	Create   *SecretCreateRequest      `json:"create,omitempty"`
	List     *SecretIdentifiersRequest `json:"list,omitempty"`
	Update   *SecretPutRequest         `json:"update,omitempty"`
	Delete   *SecretsDeleteRequest     `json:"delete,omitempty"`
}

type SecretCreateRequest struct {
	Key                                                   string   `json:"key"`
	Note                                                  string   `json:"note"`
	// Organization where the secret will be created               
	OrganizationID                                        string   `json:"organizationId"`
	// IDs of the projects that this secret will belong to         
	ProjectIDS                                            []string `json:"projectIds"`
	Value                                                 string   `json:"value"`
}

type SecretsDeleteRequest struct {
	// IDs of the secrets to delete         
	IDS                            []string `json:"ids"`
}

type SecretGetRequest struct {
	// ID of the secret to retrieve       
	ID                             string `json:"id"`
}

type SecretsGetRequest struct {
	// IDs of the secrets to retrieve         
	IDS                              []string `json:"ids"`
}

type SecretIdentifiersRequest struct {
	// Organization to retrieve all the secrets from       
	OrganizationID                                  string `json:"organizationId"`
}

type SecretPutRequest struct {
	// ID of the secret to modify                      
	ID                                        string   `json:"id"`
	Key                                       string   `json:"key"`
	Note                                      string   `json:"note"`
	// Organization ID of the secret to modify         
	OrganizationID                            string   `json:"organizationId"`
	ProjectIDS                                []string `json:"projectIds"`
	Value                                     string   `json:"value"`
}

type SyncRequest struct {
	// Exclude the subdomains from the response, defaults to false      
	ExcludeSubdomains                                             *bool `json:"excludeSubdomains"`
}

type DocRef struct {
	Cipher                      *Cipher                      `json:"Cipher,omitempty"`
	CipherView                  *CipherView                  `json:"CipherView,omitempty"`
	Collection                  *Collection                  `json:"Collection,omitempty"`
	Folder                      *Folder                      `json:"Folder,omitempty"`
	FolderView                  *FolderView                  `json:"FolderView,omitempty"`
	InitCryptoRequest           *InitCryptoRequest           `json:"InitCryptoRequest,omitempty"`
	PasswordGeneratorRequest    *PasswordGeneratorRequest    `json:"PasswordGeneratorRequest,omitempty"`
	PassphraseGeneratorRequest  *PassphraseGeneratorRequest  `json:"PassphraseGeneratorRequest,omitempty"`
	ExportFormat                *ExportFormat                `json:"ExportFormat"`
	MasterPasswordPolicyOptions *MasterPasswordPolicyOptions `json:"MasterPasswordPolicyOptions,omitempty"`
	Kdf                         *Kdf                         `json:"Kdf,omitempty"`
}

type Cipher struct {
	Attachments         []Attachment       `json:"attachments"`
	Card                *Card              `json:"card"`
	CollectionIDS       []string           `json:"collectionIds"`
	CreationDate        string             `json:"creationDate"`
	DeletedDate         *string            `json:"deletedDate"`
	Edit                bool               `json:"edit"`
	Favorite            bool               `json:"favorite"`
	Fields              []Field            `json:"fields"`
	FolderID            *string            `json:"folderId"`
	ID                  *string            `json:"id"`
	Identity            *Identity          `json:"identity"`
	LocalData           *LocalData         `json:"localData"`
	Login               *Login             `json:"login"`
	Name                string             `json:"name"`
	Notes               string             `json:"notes"`
	OrganizationID      *string            `json:"organizationId"`
	OrganizationUseTotp bool               `json:"organizationUseTotp"`
	PasswordHistory     []PasswordHistory  `json:"passwordHistory"`
	Reprompt            CipherRepromptType `json:"reprompt"`
	RevisionDate        string             `json:"revisionDate"`
	SecureNote          *SecureNote        `json:"secureNote"`
	Type                CipherType         `json:"type"`
	ViewPassword        bool               `json:"viewPassword"`
}

type Attachment struct {
	FileName                                   *string `json:"fileName"`
	ID                                         *string `json:"id"`
	Key                                        *string `json:"key"`
	Size                                       *string `json:"size"`
	// Readable size, ex: "4.2 KB" or "1.43 GB"        
	SizeName                                   *string `json:"sizeName"`
	URL                                        *string `json:"url"`
}

type Card struct {
	Brand          *string `json:"brand"`
	CardholderName *string `json:"cardholderName"`
	Code           *string `json:"code"`
	ExpMonth       *string `json:"expMonth"`
	ExpYear        *string `json:"expYear"`
	Number         *string `json:"number"`
}

type Field struct {
	LinkedID *LinkedIDType `json:"linkedId"`
	Name     string        `json:"name"`
	Type     FieldType     `json:"type"`
	Value    string        `json:"value"`
}

type Identity struct {
	Address1       *string `json:"address1"`
	Address2       *string `json:"address2"`
	Address3       *string `json:"address3"`
	City           *string `json:"city"`
	Company        *string `json:"company"`
	Country        *string `json:"country"`
	Email          *string `json:"email"`
	FirstName      *string `json:"firstName"`
	LastName       *string `json:"lastName"`
	LicenseNumber  *string `json:"licenseNumber"`
	MiddleName     *string `json:"middleName"`
	PassportNumber *string `json:"passportNumber"`
	Phone          *string `json:"phone"`
	PostalCode     *string `json:"postalCode"`
	Ssn            *string `json:"ssn"`
	State          *string `json:"state"`
	Title          *string `json:"title"`
	Username       *string `json:"username"`
}

type LocalData struct {
	LastLaunched *int64 `json:"lastLaunched"`
	LastUsedDate *int64 `json:"lastUsedDate"`
}

type Login struct {
	AutofillOnPageLoad   *bool      `json:"autofillOnPageLoad"`
	Password             string     `json:"password"`
	PasswordRevisionDate *string    `json:"passwordRevisionDate"`
	Totp                 *string    `json:"totp"`
	Uris                 []LoginURI `json:"uris"`
	Username             string     `json:"username"`
}

type LoginURI struct {
	Match *URIMatchType `json:"match"`
	URI   string        `json:"uri"`
}

type PasswordHistory struct {
	LastUsedDate string `json:"lastUsedDate"`
	Password     string `json:"password"`
}

type SecureNote struct {
	Type SecureNoteType `json:"type"`
}

type CipherView struct {
	Attachments         []AttachmentView      `json:"attachments"`
	Card                *CardView             `json:"card"`
	CollectionIDS       []string              `json:"collectionIds"`
	CreationDate        string                `json:"creationDate"`
	DeletedDate         *string               `json:"deletedDate"`
	Edit                bool                  `json:"edit"`
	Favorite            bool                  `json:"favorite"`
	Fields              []FieldView           `json:"fields"`
	FolderID            *string               `json:"folderId"`
	ID                  *string               `json:"id"`
	Identity            *IdentityView         `json:"identity"`
	LocalData           *LocalDataView        `json:"localData"`
	Login               *LoginView            `json:"login"`
	Name                string                `json:"name"`
	Notes               string                `json:"notes"`
	OrganizationID      *string               `json:"organizationId"`
	OrganizationUseTotp bool                  `json:"organizationUseTotp"`
	PasswordHistory     []PasswordHistoryView `json:"passwordHistory"`
	Reprompt            CipherRepromptType    `json:"reprompt"`
	RevisionDate        string                `json:"revisionDate"`
	SecureNote          *SecureNoteView       `json:"secureNote"`
	Type                CipherType            `json:"type"`
	ViewPassword        bool                  `json:"viewPassword"`
}

type AttachmentView struct {
	FileName *string `json:"fileName"`
	ID       *string `json:"id"`
	Key      *string `json:"key"`
	Size     *string `json:"size"`
	SizeName *string `json:"sizeName"`
	URL      *string `json:"url"`
}

type CardView struct {
	Brand          *string `json:"brand"`
	CardholderName *string `json:"cardholderName"`
	Code           *string `json:"code"`
	ExpMonth       *string `json:"expMonth"`
	ExpYear        *string `json:"expYear"`
	Number         *string `json:"number"`
}

type FieldView struct {
	LinkedID *LinkedIDType `json:"linkedId"`
	Name     string        `json:"name"`
	Type     FieldType     `json:"type"`
	Value    string        `json:"value"`
}

type IdentityView struct {
	Address1       *string `json:"address1"`
	Address2       *string `json:"address2"`
	Address3       *string `json:"address3"`
	City           *string `json:"city"`
	Company        *string `json:"company"`
	Country        *string `json:"country"`
	Email          *string `json:"email"`
	FirstName      *string `json:"firstName"`
	LastName       *string `json:"lastName"`
	LicenseNumber  *string `json:"licenseNumber"`
	MiddleName     *string `json:"middleName"`
	PassportNumber *string `json:"passportNumber"`
	Phone          *string `json:"phone"`
	PostalCode     *string `json:"postalCode"`
	Ssn            *string `json:"ssn"`
	State          *string `json:"state"`
	Title          *string `json:"title"`
	Username       *string `json:"username"`
}

type LocalDataView struct {
	LastLaunched *int64 `json:"lastLaunched"`
	LastUsedDate *int64 `json:"lastUsedDate"`
}

type LoginView struct {
	AutofillOnPageLoad   *bool          `json:"autofillOnPageLoad"`
	Password             string         `json:"password"`
	PasswordRevisionDate *string        `json:"passwordRevisionDate"`
	Totp                 *string        `json:"totp"`
	Uris                 []LoginURIView `json:"uris"`
	Username             string         `json:"username"`
}

type LoginURIView struct {
	Match *URIMatchType `json:"match"`
	URI   string        `json:"uri"`
}

type PasswordHistoryView struct {
	LastUsedDate string `json:"lastUsedDate"`
	Password     string `json:"password"`
}

type SecureNoteView struct {
	Type SecureNoteType `json:"type"`
}

type Collection struct {
	ExternalID     *string `json:"externalId"`
	HidePasswords  bool    `json:"hidePasswords"`
	ID             string  `json:"id"`
	Name           string  `json:"name"`
	OrganizationID string  `json:"organizationId"`
	ReadOnly       bool    `json:"readOnly"`
}

type ExportFormatClass struct {
	EncryptedJSON EncryptedJSON `json:"EncryptedJson"`
}

type EncryptedJSON struct {
	Password string `json:"password"`
}

type Folder struct {
	ID           string `json:"id"`
	Name         string `json:"name"`
	RevisionDate string `json:"revisionDate"`
}

type FolderView struct {
	ID           string `json:"id"`
	Name         string `json:"name"`
	RevisionDate string `json:"revisionDate"`
}

type InitCryptoRequest struct {
	// The user's email address                                                             
	Email                                                                 string            `json:"email"`
	// The user's KDF parameters, as received from the prelogin request                     
	KdfParams                                                             Kdf               `json:"kdfParams"`
	// The encryption keys for all the organizations the user is a part of                  
	OrganizationKeys                                                      map[string]string `json:"organizationKeys"`
	// The user's master password                                                           
	Password                                                              string            `json:"password"`
	// The user's encryptred private key                                                    
	PrivateKey                                                            string            `json:"privateKey"`
	// The user's encrypted symmetric crypto key                                            
	UserKey                                                               string            `json:"userKey"`
}

// The user's KDF parameters, as received from the prelogin request
type Kdf struct {
	PBKDF2   *PBKDF2   `json:"pBKDF2,omitempty"`
	Argon2ID *Argon2ID `json:"argon2id,omitempty"`
}

type Argon2ID struct {
	Iterations  int64 `json:"iterations"`
	Memory      int64 `json:"memory"`
	Parallelism int64 `json:"parallelism"`
}

type PBKDF2 struct {
	Iterations int64 `json:"iterations"`
}

type MasterPasswordPolicyOptions struct {
	// Flag to indicate if the policy should be enforced on login. If true, and the user's          
	// password does not meet the policy requirements, the user will be forced to update their      
	// password.                                                                                    
	EnforceOnLogin                                                                            bool  `json:"enforce_on_login"`
	MinComplexity                                                                             int64 `json:"min_complexity"`
	MinLength                                                                                 int64 `json:"min_length"`
	RequireLower                                                                              bool  `json:"require_lower"`
	RequireNumbers                                                                            bool  `json:"require_numbers"`
	RequireSpecial                                                                            bool  `json:"require_special"`
	RequireUpper                                                                              bool  `json:"require_upper"`
}

// Passphrase generator request.
//
// The default separator is `-` and default number of words is 3.
type PassphraseGeneratorRequest struct {
	Capitalize    *bool   `json:"capitalize"`
	IncludeNumber *bool   `json:"includeNumber"`
	NumWords      *int64  `json:"numWords"`
	WordSeparator *string `json:"wordSeparator"`
}

// Password generator request. If all options are false, the default is to generate a
// password with: - lowercase - uppercase - numbers
//
// The default length is 16.
type PasswordGeneratorRequest struct {
	AvoidAmbiguous *bool  `json:"avoidAmbiguous"`
	Length         *int64 `json:"length"`
	Lowercase      bool   `json:"lowercase"`
	MinLowercase   *bool  `json:"minLowercase"`
	MinNumber      *bool  `json:"minNumber"`
	MinSpecial     *bool  `json:"minSpecial"`
	MinUppercase   *bool  `json:"minUppercase"`
	Numbers        bool   `json:"numbers"`
	Special        bool   `json:"special"`
	Uppercase      bool   `json:"uppercase"`
}

type ResponseForAPIKeyLoginResponse struct {
	// The response data. Populated if `success` is true.                                           
	Data                                                                       *APIKeyLoginResponse `json:"data"`
	// A message for any error that may occur. Populated if `success` is false.                     
	ErrorMessage                                                               *string              `json:"errorMessage"`
	// Whether or not the SDK request succeeded.                                                    
	Success                                                                    bool                 `json:"success"`
}

type APIKeyLoginResponse struct {
	Authenticated                                                         bool                                   `json:"authenticated"`
	// Whether or not the user is required to update their master password                                       
	ForcePasswordReset                                                    bool                                   `json:"forcePasswordReset"`
	// TODO: What does this do?                                                                                  
	ResetMasterPassword                                                   bool                                   `json:"resetMasterPassword"`
	TwoFactor                                                             *APIKeyLoginResponseTwoFactorProviders `json:"twoFactor"`
}

type APIKeyLoginResponseTwoFactorProviders struct {
	Authenticator                                                         *PurpleAuthenticator `json:"authenticator"`
	// Duo-backed 2fa                                                                          
	Duo                                                                   *PurpleDuo           `json:"duo"`
	// Email 2fa                                                                               
	Email                                                                 *PurpleEmail         `json:"email"`
	// Duo-backed 2fa operated by an organization the user is a member of                      
	OrganizationDuo                                                       *PurpleDuo           `json:"organizationDuo"`
	// Presence indicates the user has stored this device as bypassing 2fa                     
	Remember                                                              *PurpleRemember      `json:"remember"`
	// WebAuthn-backed 2fa                                                                     
	WebAuthn                                                              *PurpleWebAuthn      `json:"webAuthn"`
	// Yubikey-backed 2fa                                                                      
	YubiKey                                                               *PurpleYubiKey       `json:"yubiKey"`
}

type PurpleAuthenticator struct {
}

type PurpleDuo struct {
	Host      string `json:"host"`
	Signature string `json:"signature"`
}

type PurpleEmail struct {
	// The email to request a 2fa TOTP for       
	Email                                 string `json:"email"`
}

type PurpleRemember struct {
}

type PurpleWebAuthn struct {
}

type PurpleYubiKey struct {
	// Whether the stored yubikey supports near field communication     
	NFC                                                            bool `json:"nfc"`
}

type ResponseForFingerprintResponse struct {
	// The response data. Populated if `success` is true.                                           
	Data                                                                       *FingerprintResponse `json:"data"`
	// A message for any error that may occur. Populated if `success` is false.                     
	ErrorMessage                                                               *string              `json:"errorMessage"`
	// Whether or not the SDK request succeeded.                                                    
	Success                                                                    bool                 `json:"success"`
}

type FingerprintResponse struct {
	Fingerprint string `json:"fingerprint"`
}

type ResponseForPasswordLoginResponse struct {
	// The response data. Populated if `success` is true.                                             
	Data                                                                       *PasswordLoginResponse `json:"data"`
	// A message for any error that may occur. Populated if `success` is false.                       
	ErrorMessage                                                               *string                `json:"errorMessage"`
	// Whether or not the SDK request succeeded.                                                      
	Success                                                                    bool                   `json:"success"`
}

type PasswordLoginResponse struct {
	Authenticated                                                                              bool                                     `json:"authenticated"`
	// The information required to present the user with a captcha challenge. Only present when                                         
	// authentication fails due to requiring validation of a captcha challenge.                                                         
	CAPTCHA                                                                                    *CAPTCHAResponse                         `json:"captcha"`
	// Whether or not the user is required to update their master password                                                              
	ForcePasswordReset                                                                         bool                                     `json:"forcePasswordReset"`
	// TODO: What does this do?                                                                                                         
	ResetMasterPassword                                                                        bool                                     `json:"resetMasterPassword"`
	// The available two factor authentication options. Present only when authentication fails                                          
	// due to requiring a second authentication factor.                                                                                 
	TwoFactor                                                                                  *PasswordLoginResponseTwoFactorProviders `json:"twoFactor"`
}

type CAPTCHAResponse struct {
	// hcaptcha site key       
	SiteKey             string `json:"siteKey"`
}

type PasswordLoginResponseTwoFactorProviders struct {
	Authenticator                                                         *FluffyAuthenticator `json:"authenticator"`
	// Duo-backed 2fa                                                                          
	Duo                                                                   *FluffyDuo           `json:"duo"`
	// Email 2fa                                                                               
	Email                                                                 *FluffyEmail         `json:"email"`
	// Duo-backed 2fa operated by an organization the user is a member of                      
	OrganizationDuo                                                       *FluffyDuo           `json:"organizationDuo"`
	// Presence indicates the user has stored this device as bypassing 2fa                     
	Remember                                                              *FluffyRemember      `json:"remember"`
	// WebAuthn-backed 2fa                                                                     
	WebAuthn                                                              *FluffyWebAuthn      `json:"webAuthn"`
	// Yubikey-backed 2fa                                                                      
	YubiKey                                                               *FluffyYubiKey       `json:"yubiKey"`
}

type FluffyAuthenticator struct {
}

type FluffyDuo struct {
	Host      string `json:"host"`
	Signature string `json:"signature"`
}

type FluffyEmail struct {
	// The email to request a 2fa TOTP for       
	Email                                 string `json:"email"`
}

type FluffyRemember struct {
}

type FluffyWebAuthn struct {
}

type FluffyYubiKey struct {
	// Whether the stored yubikey supports near field communication     
	NFC                                                            bool `json:"nfc"`
}

type ResponseForProjectResponse struct {
	// The response data. Populated if `success` is true.                                       
	Data                                                                       *ProjectResponse `json:"data"`
	// A message for any error that may occur. Populated if `success` is false.                 
	ErrorMessage                                                               *string          `json:"errorMessage"`
	// Whether or not the SDK request succeeded.                                                
	Success                                                                    bool             `json:"success"`
}

type ProjectResponse struct {
	CreationDate   string `json:"creationDate"`
	ID             string `json:"id"`
	Name           string `json:"name"`
	Object         string `json:"object"`
	OrganizationID string `json:"organizationId"`
	RevisionDate   string `json:"revisionDate"`
}

type ResponseForProjectsDeleteResponse struct {
	// The response data. Populated if `success` is true.                                              
	Data                                                                       *ProjectsDeleteResponse `json:"data"`
	// A message for any error that may occur. Populated if `success` is false.                        
	ErrorMessage                                                               *string                 `json:"errorMessage"`
	// Whether or not the SDK request succeeded.                                                       
	Success                                                                    bool                    `json:"success"`
}

type ProjectsDeleteResponse struct {
	Data []ProjectDeleteResponse `json:"data"`
}

type ProjectDeleteResponse struct {
	Error *string `json:"error"`
	ID    string  `json:"id"`
}

type ResponseForProjectsResponse struct {
	// The response data. Populated if `success` is true.                                        
	Data                                                                       *ProjectsResponse `json:"data"`
	// A message for any error that may occur. Populated if `success` is false.                  
	ErrorMessage                                                               *string           `json:"errorMessage"`
	// Whether or not the SDK request succeeded.                                                 
	Success                                                                    bool              `json:"success"`
}

type ProjectsResponse struct {
	Data []DatumElement `json:"data"`
}

type DatumElement struct {
	CreationDate   string `json:"creationDate"`
	ID             string `json:"id"`
	Name           string `json:"name"`
	Object         string `json:"object"`
	OrganizationID string `json:"organizationId"`
	RevisionDate   string `json:"revisionDate"`
}

type ResponseForSecretIdentifiersResponse struct {
	// The response data. Populated if `success` is true.                                                 
	Data                                                                       *SecretIdentifiersResponse `json:"data"`
	// A message for any error that may occur. Populated if `success` is false.                           
	ErrorMessage                                                               *string                    `json:"errorMessage"`
	// Whether or not the SDK request succeeded.                                                          
	Success                                                                    bool                       `json:"success"`
}

type SecretIdentifiersResponse struct {
	Data []SecretIdentifierResponse `json:"data"`
}

type SecretIdentifierResponse struct {
	ID             string `json:"id"`
	Key            string `json:"key"`
	OrganizationID string `json:"organizationId"`
}

type ResponseForSecretResponse struct {
	// The response data. Populated if `success` is true.                                      
	Data                                                                       *SecretResponse `json:"data"`
	// A message for any error that may occur. Populated if `success` is false.                
	ErrorMessage                                                               *string         `json:"errorMessage"`
	// Whether or not the SDK request succeeded.                                               
	Success                                                                    bool            `json:"success"`
}

type SecretResponse struct {
	CreationDate   string  `json:"creationDate"`
	ID             string  `json:"id"`
	Key            string  `json:"key"`
	Note           string  `json:"note"`
	Object         string  `json:"object"`
	OrganizationID string  `json:"organizationId"`
	ProjectID      *string `json:"projectId"`
	RevisionDate   string  `json:"revisionDate"`
	Value          string  `json:"value"`
}

type ResponseForSecretsDeleteResponse struct {
	// The response data. Populated if `success` is true.                                             
	Data                                                                       *SecretsDeleteResponse `json:"data"`
	// A message for any error that may occur. Populated if `success` is false.                       
	ErrorMessage                                                               *string                `json:"errorMessage"`
	// Whether or not the SDK request succeeded.                                                      
	Success                                                                    bool                   `json:"success"`
}

type SecretsDeleteResponse struct {
	Data []SecretDeleteResponse `json:"data"`
}

type SecretDeleteResponse struct {
	Error *string `json:"error"`
	ID    string  `json:"id"`
}

type ResponseForSecretsResponse struct {
	// The response data. Populated if `success` is true.                                       
	Data                                                                       *SecretsResponse `json:"data"`
	// A message for any error that may occur. Populated if `success` is false.                 
	ErrorMessage                                                               *string          `json:"errorMessage"`
	// Whether or not the SDK request succeeded.                                                
	Success                                                                    bool             `json:"success"`
}

type SecretsResponse struct {
	Data []DatumClass `json:"data"`
}

type DatumClass struct {
	CreationDate   string  `json:"creationDate"`
	ID             string  `json:"id"`
	Key            string  `json:"key"`
	Note           string  `json:"note"`
	Object         string  `json:"object"`
	OrganizationID string  `json:"organizationId"`
	ProjectID      *string `json:"projectId"`
	RevisionDate   string  `json:"revisionDate"`
	Value          string  `json:"value"`
}

type ResponseForSyncResponse struct {
	// The response data. Populated if `success` is true.                                    
	Data                                                                       *SyncResponse `json:"data"`
	// A message for any error that may occur. Populated if `success` is false.              
	ErrorMessage                                                               *string       `json:"errorMessage"`
	// Whether or not the SDK request succeeded.                                             
	Success                                                                    bool          `json:"success"`
}

type SyncResponse struct {
	// List of ciphers accesible by the user                                                                        
	Ciphers                                                                                 []CipherDetailsResponse `json:"ciphers"`
	// Data about the user, including their encryption keys and the organizations they are a                        
	// part of                                                                                                      
	Profile                                                                                 ProfileResponse         `json:"profile"`
}

type CipherDetailsResponse struct {
}

// Data about the user, including their encryption keys and the organizations they are a
// part of
type ProfileResponse struct {
	Email         string                        `json:"email"`
	ID            string                        `json:"id"`
	Name          string                        `json:"name"`
	Organizations []ProfileOrganizationResponse `json:"organizations"`
}

type ProfileOrganizationResponse struct {
	ID string `json:"id"`
}

type ResponseForUserAPIKeyResponse struct {
	// The response data. Populated if `success` is true.                                          
	Data                                                                       *UserAPIKeyResponse `json:"data"`
	// A message for any error that may occur. Populated if `success` is false.                    
	ErrorMessage                                                               *string             `json:"errorMessage"`
	// Whether or not the SDK request succeeded.                                                   
	Success                                                                    bool                `json:"success"`
}

type UserAPIKeyResponse struct {
	// The user's API key, which represents the client_secret portion of an oauth request.       
	APIKey                                                                                string `json:"apiKey"`
}

// Device type to send to Bitwarden. Defaults to SDK
type DeviceType string

const (
	Android          DeviceType = "Android"
	AndroidAmazon    DeviceType = "AndroidAmazon"
	ChromeBrowser    DeviceType = "ChromeBrowser"
	ChromeExtension  DeviceType = "ChromeExtension"
	EdgeBrowser      DeviceType = "EdgeBrowser"
	EdgeExtension    DeviceType = "EdgeExtension"
	FirefoxBrowser   DeviceType = "FirefoxBrowser"
	FirefoxExtension DeviceType = "FirefoxExtension"
	IEBrowser        DeviceType = "IEBrowser"
	IOS              DeviceType = "iOS"
	LinuxDesktop     DeviceType = "LinuxDesktop"
	MACOSDesktop     DeviceType = "MacOsDesktop"
	OperaBrowser     DeviceType = "OperaBrowser"
	OperaExtension   DeviceType = "OperaExtension"
	SDK              DeviceType = "SDK"
	SafariBrowser    DeviceType = "SafariBrowser"
	SafariExtension  DeviceType = "SafariExtension"
	UWP              DeviceType = "UWP"
	UnknownBrowser   DeviceType = "UnknownBrowser"
	VivaldiBrowser   DeviceType = "VivaldiBrowser"
	VivaldiExtension DeviceType = "VivaldiExtension"
	WindowsDesktop   DeviceType = "WindowsDesktop"
)

// Two-factor provider
type TwoFactorProvider string

const (
	Authenticator          TwoFactorProvider = "Authenticator"
	Duo                    TwoFactorProvider = "Duo"
	OrganizationDuo        TwoFactorProvider = "OrganizationDuo"
	Remember               TwoFactorProvider = "Remember"
	TwoFactorProviderEmail TwoFactorProvider = "Email"
	U2F                    TwoFactorProvider = "U2f"
	WebAuthn               TwoFactorProvider = "WebAuthn"
	Yubikey                TwoFactorProvider = "Yubikey"
)

type LinkedIDType string

const (
	Address1             LinkedIDType = "Address1"
	Address2             LinkedIDType = "Address2"
	Address3             LinkedIDType = "Address3"
	Brand                LinkedIDType = "Brand"
	CardholderName       LinkedIDType = "CardholderName"
	City                 LinkedIDType = "City"
	Code                 LinkedIDType = "Code"
	Company              LinkedIDType = "Company"
	Country              LinkedIDType = "Country"
	ExpMonth             LinkedIDType = "ExpMonth"
	ExpYear              LinkedIDType = "ExpYear"
	FirstName            LinkedIDType = "FirstName"
	FullName             LinkedIDType = "FullName"
	LastName             LinkedIDType = "LastName"
	LicenseNumber        LinkedIDType = "LicenseNumber"
	LinkedIDTypeEmail    LinkedIDType = "Email"
	LinkedIDTypePassword LinkedIDType = "Password"
	MiddleName           LinkedIDType = "MiddleName"
	Number               LinkedIDType = "Number"
	PassportNumber       LinkedIDType = "PassportNumber"
	Phone                LinkedIDType = "Phone"
	PostalCode           LinkedIDType = "PostalCode"
	Ssn                  LinkedIDType = "Ssn"
	State                LinkedIDType = "State"
	Title                LinkedIDType = "Title"
	Username             LinkedIDType = "Username"
)

type FieldType string

const (
	Boolean FieldType = "Boolean"
	Hidden  FieldType = "Hidden"
	Linked  FieldType = "Linked"
	Text    FieldType = "Text"
)

type URIMatchType string

const (
	Domain            URIMatchType = "domain"
	Exact             URIMatchType = "exact"
	Host              URIMatchType = "host"
	Never             URIMatchType = "never"
	RegularExpression URIMatchType = "regularExpression"
	StartsWith        URIMatchType = "startsWith"
)

type CipherRepromptType string

const (
	CipherRepromptTypePassword CipherRepromptType = "Password"
	None                       CipherRepromptType = "None"
)

type SecureNoteType string

const (
	Generic SecureNoteType = "Generic"
)

type CipherType string

const (
	CipherTypeCard       CipherType = "Card"
	CipherTypeIdentity   CipherType = "Identity"
	CipherTypeLogin      CipherType = "Login"
	CipherTypeSecureNote CipherType = "SecureNote"
)

type ExportFormatEnum string

const (
	AccountEncryptedJSON ExportFormatEnum = "AccountEncryptedJson"
	CSV                  ExportFormatEnum = "Csv"
	JSON                 ExportFormatEnum = "Json"
)

type ExportFormat struct {
	Enum              *ExportFormatEnum
	ExportFormatClass *ExportFormatClass
}

func (x *ExportFormat) UnmarshalJSON(data []byte) error {
	x.ExportFormatClass = nil
	x.Enum = nil
	var c ExportFormatClass
	object, err := unmarshalUnion(data, nil, nil, nil, nil, false, nil, true, &c, false, nil, true, &x.Enum, false)
	if err != nil {
		return err
	}
	if object {
		x.ExportFormatClass = &c
	}
	return nil
}

func (x *ExportFormat) MarshalJSON() ([]byte, error) {
	return marshalUnion(nil, nil, nil, nil, false, nil, x.ExportFormatClass != nil, x.ExportFormatClass, false, nil, x.Enum != nil, x.Enum, false)
}

func unmarshalUnion(data []byte, pi **int64, pf **float64, pb **bool, ps **string, haveArray bool, pa interface{}, haveObject bool, pc interface{}, haveMap bool, pm interface{}, haveEnum bool, pe interface{}, nullable bool) (bool, error) {
	if pi != nil {
		*pi = nil
	}
	if pf != nil {
		*pf = nil
	}
	if pb != nil {
		*pb = nil
	}
	if ps != nil {
		*ps = nil
	}

	dec := json.NewDecoder(bytes.NewReader(data))
	dec.UseNumber()
	tok, err := dec.Token()
	if err != nil {
		return false, err
	}

	switch v := tok.(type) {
	case json.Number:
		if pi != nil {
			i, err := v.Int64()
			if err == nil {
				*pi = &i
				return false, nil
			}
		}
		if pf != nil {
			f, err := v.Float64()
			if err == nil {
				*pf = &f
				return false, nil
			}
			return false, errors.New("Unparsable number")
		}
		return false, errors.New("Union does not contain number")
	case float64:
		return false, errors.New("Decoder should not return float64")
	case bool:
		if pb != nil {
			*pb = &v
			return false, nil
		}
		return false, errors.New("Union does not contain bool")
	case string:
		if haveEnum {
			return false, json.Unmarshal(data, pe)
		}
		if ps != nil {
			*ps = &v
			return false, nil
		}
		return false, errors.New("Union does not contain string")
	case nil:
		if nullable {
			return false, nil
		}
		return false, errors.New("Union does not contain null")
	case json.Delim:
		if v == '{' {
			if haveObject {
				return true, json.Unmarshal(data, pc)
			}
			if haveMap {
				return false, json.Unmarshal(data, pm)
			}
			return false, errors.New("Union does not contain object")
		}
		if v == '[' {
			if haveArray {
				return false, json.Unmarshal(data, pa)
			}
			return false, errors.New("Union does not contain array")
		}
		return false, errors.New("Cannot handle delimiter")
	}
	return false, errors.New("Cannot unmarshal union")

}

func marshalUnion(pi *int64, pf *float64, pb *bool, ps *string, haveArray bool, pa interface{}, haveObject bool, pc interface{}, haveMap bool, pm interface{}, haveEnum bool, pe interface{}, nullable bool) ([]byte, error) {
	if pi != nil {
		return json.Marshal(*pi)
	}
	if pf != nil {
		return json.Marshal(*pf)
	}
	if pb != nil {
		return json.Marshal(*pb)
	}
	if ps != nil {
		return json.Marshal(*ps)
	}
	if haveArray {
		return json.Marshal(pa)
	}
	if haveObject {
		return json.Marshal(pc)
	}
	if haveMap {
		return json.Marshal(pm)
	}
	if haveEnum {
		return json.Marshal(pe)
	}
	if nullable {
		return json.Marshal(nil)
	}
	return nil, errors.New("Union must not be null")
}

