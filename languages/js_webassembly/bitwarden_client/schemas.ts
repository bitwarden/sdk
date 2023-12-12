// To parse this data:
//
//   import { Convert, ClientSettings, DeviceType, Command, PasswordLoginRequest, TwoFactorRequest, TwoFactorProvider, Kdf, APIKeyLoginRequest, AccessTokenLoginRequest, SecretVerificationRequest, FingerprintRequest, SyncRequest, SecretsCommand, SecretGetRequest, SecretsGetRequest, SecretCreateRequest, SecretIdentifiersRequest, SecretPutRequest, SecretsDeleteRequest, ProjectsCommand, ProjectGetRequest, ProjectCreateRequest, ProjectsListRequest, ProjectPutRequest, ProjectsDeleteRequest, ResponseForAPIKeyLoginResponse, APIKeyLoginResponse, TwoFactorProviders, Authenticator, Email, Duo, YubiKey, Remember, WebAuthn, ResponseForPasswordLoginResponse, PasswordLoginResponse, CAPTCHAResponse, ResponseForAccessTokenLoginResponse, AccessTokenLoginResponse, ResponseForSecretIdentifiersResponse, SecretIdentifiersResponse, SecretIdentifierResponse, ResponseForSecretResponse, SecretResponse, ResponseForSecretsResponse, SecretsResponse, ResponseForSecretsDeleteResponse, SecretsDeleteResponse, SecretDeleteResponse, ResponseForProjectResponse, ProjectResponse, ResponseForProjectsResponse, ProjectsResponse, ResponseForProjectsDeleteResponse, ProjectsDeleteResponse, ProjectDeleteResponse, ResponseForFingerprintResponse, FingerprintResponse, ResponseForSyncResponse, SyncResponse, ProfileResponse, ProfileOrganizationResponse, CipherDetailsResponse, ResponseForUserAPIKeyResponse, UserAPIKeyResponse } from "./file";
//
//   const clientSettings = Convert.toClientSettings(json);
//   const deviceType = Convert.toDeviceType(json);
//   const command = Convert.toCommand(json);
//   const passwordLoginRequest = Convert.toPasswordLoginRequest(json);
//   const twoFactorRequest = Convert.toTwoFactorRequest(json);
//   const twoFactorProvider = Convert.toTwoFactorProvider(json);
//   const kdf = Convert.toKdf(json);
//   const aPIKeyLoginRequest = Convert.toAPIKeyLoginRequest(json);
//   const accessTokenLoginRequest = Convert.toAccessTokenLoginRequest(json);
//   const secretVerificationRequest = Convert.toSecretVerificationRequest(json);
//   const fingerprintRequest = Convert.toFingerprintRequest(json);
//   const syncRequest = Convert.toSyncRequest(json);
//   const secretsCommand = Convert.toSecretsCommand(json);
//   const secretGetRequest = Convert.toSecretGetRequest(json);
//   const secretsGetRequest = Convert.toSecretsGetRequest(json);
//   const secretCreateRequest = Convert.toSecretCreateRequest(json);
//   const secretIdentifiersRequest = Convert.toSecretIdentifiersRequest(json);
//   const secretPutRequest = Convert.toSecretPutRequest(json);
//   const secretsDeleteRequest = Convert.toSecretsDeleteRequest(json);
//   const projectsCommand = Convert.toProjectsCommand(json);
//   const projectGetRequest = Convert.toProjectGetRequest(json);
//   const projectCreateRequest = Convert.toProjectCreateRequest(json);
//   const projectsListRequest = Convert.toProjectsListRequest(json);
//   const projectPutRequest = Convert.toProjectPutRequest(json);
//   const projectsDeleteRequest = Convert.toProjectsDeleteRequest(json);
//   const responseForAPIKeyLoginResponse = Convert.toResponseForAPIKeyLoginResponse(json);
//   const aPIKeyLoginResponse = Convert.toAPIKeyLoginResponse(json);
//   const twoFactorProviders = Convert.toTwoFactorProviders(json);
//   const authenticator = Convert.toAuthenticator(json);
//   const email = Convert.toEmail(json);
//   const duo = Convert.toDuo(json);
//   const yubiKey = Convert.toYubiKey(json);
//   const remember = Convert.toRemember(json);
//   const webAuthn = Convert.toWebAuthn(json);
//   const responseForPasswordLoginResponse = Convert.toResponseForPasswordLoginResponse(json);
//   const passwordLoginResponse = Convert.toPasswordLoginResponse(json);
//   const cAPTCHAResponse = Convert.toCAPTCHAResponse(json);
//   const responseForAccessTokenLoginResponse = Convert.toResponseForAccessTokenLoginResponse(json);
//   const accessTokenLoginResponse = Convert.toAccessTokenLoginResponse(json);
//   const responseForSecretIdentifiersResponse = Convert.toResponseForSecretIdentifiersResponse(json);
//   const secretIdentifiersResponse = Convert.toSecretIdentifiersResponse(json);
//   const secretIdentifierResponse = Convert.toSecretIdentifierResponse(json);
//   const responseForSecretResponse = Convert.toResponseForSecretResponse(json);
//   const secretResponse = Convert.toSecretResponse(json);
//   const responseForSecretsResponse = Convert.toResponseForSecretsResponse(json);
//   const secretsResponse = Convert.toSecretsResponse(json);
//   const responseForSecretsDeleteResponse = Convert.toResponseForSecretsDeleteResponse(json);
//   const secretsDeleteResponse = Convert.toSecretsDeleteResponse(json);
//   const secretDeleteResponse = Convert.toSecretDeleteResponse(json);
//   const responseForProjectResponse = Convert.toResponseForProjectResponse(json);
//   const projectResponse = Convert.toProjectResponse(json);
//   const responseForProjectsResponse = Convert.toResponseForProjectsResponse(json);
//   const projectsResponse = Convert.toProjectsResponse(json);
//   const responseForProjectsDeleteResponse = Convert.toResponseForProjectsDeleteResponse(json);
//   const projectsDeleteResponse = Convert.toProjectsDeleteResponse(json);
//   const projectDeleteResponse = Convert.toProjectDeleteResponse(json);
//   const responseForFingerprintResponse = Convert.toResponseForFingerprintResponse(json);
//   const fingerprintResponse = Convert.toFingerprintResponse(json);
//   const responseForSyncResponse = Convert.toResponseForSyncResponse(json);
//   const syncResponse = Convert.toSyncResponse(json);
//   const profileResponse = Convert.toProfileResponse(json);
//   const profileOrganizationResponse = Convert.toProfileOrganizationResponse(json);
//   const cipherDetailsResponse = Convert.toCipherDetailsResponse(json);
//   const responseForUserAPIKeyResponse = Convert.toResponseForUserAPIKeyResponse(json);
//   const userAPIKeyResponse = Convert.toUserAPIKeyResponse(json);
//
// These functions will throw an error if the JSON doesn't
// match the expected interface, even if the JSON is valid.

/**
 * Basic client behavior settings. These settings specify the various targets and behavior
 * of the Bitwarden Client. They are optional and uneditable once the client is
 * initialized.
 *
 * Defaults to
 *
 * ``` # use bitwarden::client::client_settings::{ClientSettings, DeviceType}; # use
 * assert_matches::assert_matches; let settings = ClientSettings { identity_url:
 * "https://identity.bitwarden.com".to_string(), api_url:
 * "https://api.bitwarden.com".to_string(), user_agent: "Bitwarden Rust-SDK".to_string(),
 * device_type: DeviceType::SDK, }; let default = ClientSettings::default();
 * assert_matches!(settings, default); ```
 *
 * Targets `localhost:8080` for debug builds.
 */
export interface ClientSettings {
    /**
     * The api url of the targeted Bitwarden instance. Defaults to `https://api.bitwarden.com`
     */
    apiUrl?: string;
    /**
     * Device type to send to Bitwarden. Defaults to SDK
     */
    deviceType?: DeviceType;
    /**
     * The identity url of the targeted Bitwarden instance. Defaults to
     * `https://identity.bitwarden.com`
     */
    identityUrl?: string;
    /**
     * The user_agent to sent to Bitwarden. Defaults to `Bitwarden Rust-SDK`
     */
    userAgent?: string;
}

/**
 * Device type to send to Bitwarden. Defaults to SDK
 */
export enum DeviceType {
    Android = "Android",
    AndroidAmazon = "AndroidAmazon",
    ChromeBrowser = "ChromeBrowser",
    ChromeExtension = "ChromeExtension",
    EdgeBrowser = "EdgeBrowser",
    EdgeExtension = "EdgeExtension",
    FirefoxBrowser = "FirefoxBrowser",
    FirefoxExtension = "FirefoxExtension",
    IEBrowser = "IEBrowser",
    IOS = "iOS",
    LinuxDesktop = "LinuxDesktop",
    MACOSDesktop = "MacOsDesktop",
    OperaBrowser = "OperaBrowser",
    OperaExtension = "OperaExtension",
    SDK = "SDK",
    SafariBrowser = "SafariBrowser",
    SafariExtension = "SafariExtension",
    UWP = "UWP",
    UnknownBrowser = "UnknownBrowser",
    VivaldiBrowser = "VivaldiBrowser",
    VivaldiExtension = "VivaldiExtension",
    WindowsDesktop = "WindowsDesktop",
}

/**
 * Login with username and password
 *
 * This command is for initiating an authentication handshake with Bitwarden. Authorization
 * may fail due to requiring 2fa or captcha challenge completion despite accurate
 * credentials.
 *
 * This command is not capable of handling authentication requiring 2fa or captcha.
 *
 * Returns: [PasswordLoginResponse](bitwarden::auth::login::PasswordLoginResponse)
 *
 * Login with API Key
 *
 * This command is for initiating an authentication handshake with Bitwarden.
 *
 * Returns: [ApiKeyLoginResponse](bitwarden::auth::login::ApiKeyLoginResponse)
 *
 * Login with Secrets Manager Access Token
 *
 * This command is for initiating an authentication handshake with Bitwarden.
 *
 * Returns: [ApiKeyLoginResponse](bitwarden::auth::login::ApiKeyLoginResponse)
 *
 * > Requires Authentication Get the API key of the currently authenticated user
 *
 * Returns: [UserApiKeyResponse](bitwarden::platform::UserApiKeyResponse)
 *
 * Get the user's passphrase
 *
 * Returns: String
 *
 * > Requires Authentication Retrieve all user data, ciphers and organizations the user is a
 * part of
 *
 * Returns: [SyncResponse](bitwarden::platform::SyncResponse)
 */
export interface Command {
    passwordLogin?:    PasswordLoginRequest;
    apiKeyLogin?:      APIKeyLoginRequest;
    accessTokenLogin?: AccessTokenLoginRequest;
    getUserApiKey?:    SecretVerificationRequest;
    fingerprint?:      FingerprintRequest;
    sync?:             SyncRequest;
    secrets?:          SecretsCommand;
    projects?:         ProjectsCommand;
}

/**
 * Login to Bitwarden with access token
 */
export interface AccessTokenLoginRequest {
    /**
     * Bitwarden service API access token
     */
    accessToken: string;
}

/**
 * Login to Bitwarden with Api Key
 */
export interface APIKeyLoginRequest {
    /**
     * Bitwarden account client_id
     */
    clientId: string;
    /**
     * Bitwarden account client_secret
     */
    clientSecret: string;
    /**
     * Bitwarden account master password
     */
    password: string;
}

export interface FingerprintRequest {
    /**
     * The input material, used in the fingerprint generation process.
     */
    fingerprintMaterial: string;
    /**
     * The user's public key encoded with base64.
     */
    publicKey: string;
}

export interface SecretVerificationRequest {
    /**
     * The user's master password to use for user verification. If supplied, this will be used
     * for verification purposes.
     */
    masterPassword?: null | string;
    /**
     * Alternate user verification method through OTP. This is provided for users who have no
     * master password due to use of Customer Managed Encryption. Must be present and valid if
     * master_password is absent.
     */
    otp?: null | string;
}

/**
 * Login to Bitwarden with Username and Password
 */
export interface PasswordLoginRequest {
    /**
     * Bitwarden account email address
     */
    email: string;
    /**
     * Kdf from prelogin
     */
    kdf: Kdf;
    /**
     * Bitwarden account master password
     */
    password:   string;
    twoFactor?: TwoFactorRequest | null;
}

/**
 * Kdf from prelogin
 */
export interface Kdf {
    pBKDF2?:   PBKDF2;
    argon2id?: Argon2ID;
}

export interface Argon2ID {
    iterations:  number;
    memory:      number;
    parallelism: number;
}

export interface PBKDF2 {
    iterations: number;
}

export interface TwoFactorRequest {
    /**
     * Two-factor provider
     */
    provider: TwoFactorProvider;
    /**
     * Two-factor remember
     */
    remember: boolean;
    /**
     * Two-factor Token
     */
    token: string;
}

/**
 * Two-factor provider
 */
export enum TwoFactorProvider {
    Authenticator = "Authenticator",
    Duo = "Duo",
    Email = "Email",
    OrganizationDuo = "OrganizationDuo",
    Remember = "Remember",
    U2F = "U2f",
    WebAuthn = "WebAuthn",
    Yubikey = "Yubikey",
}

/**
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at
 * least once Retrieve a project by the provided identifier
 *
 * Returns: [ProjectResponse](bitwarden::secrets_manager::projects::ProjectResponse)
 *
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at
 * least once Creates a new project in the provided organization using the given data
 *
 * Returns: [ProjectResponse](bitwarden::secrets_manager::projects::ProjectResponse)
 *
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at
 * least once Lists all projects of the given organization
 *
 * Returns: [ProjectsResponse](bitwarden::secrets_manager::projects::ProjectsResponse)
 *
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at
 * least once Updates an existing project with the provided ID using the given data
 *
 * Returns: [ProjectResponse](bitwarden::secrets_manager::projects::ProjectResponse)
 *
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at
 * least once Deletes all the projects whose IDs match the provided ones
 *
 * Returns:
 * [ProjectsDeleteResponse](bitwarden::secrets_manager::projects::ProjectsDeleteResponse)
 */
export interface ProjectsCommand {
    get?:    ProjectGetRequest;
    create?: ProjectCreateRequest;
    list?:   ProjectsListRequest;
    update?: ProjectPutRequest;
    delete?: ProjectsDeleteRequest;
}

export interface ProjectCreateRequest {
    name: string;
    /**
     * Organization where the project will be created
     */
    organizationId: string;
}

export interface ProjectsDeleteRequest {
    /**
     * IDs of the projects to delete
     */
    ids: string[];
}

export interface ProjectGetRequest {
    /**
     * ID of the project to retrieve
     */
    id: string;
}

export interface ProjectsListRequest {
    /**
     * Organization to retrieve all the projects from
     */
    organizationId: string;
}

export interface ProjectPutRequest {
    /**
     * ID of the project to modify
     */
    id:   string;
    name: string;
    /**
     * Organization ID of the project to modify
     */
    organizationId: string;
}

/**
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at
 * least once Retrieve a secret by the provided identifier
 *
 * Returns: [SecretResponse](bitwarden::secrets_manager::secrets::SecretResponse)
 *
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at
 * least once Retrieve secrets by the provided identifiers
 *
 * Returns: [SecretsResponse](bitwarden::secrets_manager::secrets::SecretsResponse)
 *
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at
 * least once Creates a new secret in the provided organization using the given data
 *
 * Returns: [SecretResponse](bitwarden::secrets_manager::secrets::SecretResponse)
 *
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at
 * least once Lists all secret identifiers of the given organization, to then retrieve each
 * secret, use `CreateSecret`
 *
 * Returns:
 * [SecretIdentifiersResponse](bitwarden::secrets_manager::secrets::SecretIdentifiersResponse)
 *
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at
 * least once Updates an existing secret with the provided ID using the given data
 *
 * Returns: [SecretResponse](bitwarden::secrets_manager::secrets::SecretResponse)
 *
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at
 * least once Deletes all the secrets whose IDs match the provided ones
 *
 * Returns:
 * [SecretsDeleteResponse](bitwarden::secrets_manager::secrets::SecretsDeleteResponse)
 */
export interface SecretsCommand {
    get?:      SecretGetRequest;
    getByIds?: SecretsGetRequest;
    create?:   SecretCreateRequest;
    list?:     SecretIdentifiersRequest;
    update?:   SecretPutRequest;
    delete?:   SecretsDeleteRequest;
}

export interface SecretCreateRequest {
    key:  string;
    note: string;
    /**
     * Organization where the secret will be created
     */
    organizationId: string;
    /**
     * IDs of the projects that this secret will belong to
     */
    projectIds?: string[] | null;
    value:       string;
}

export interface SecretsDeleteRequest {
    /**
     * IDs of the secrets to delete
     */
    ids: string[];
}

export interface SecretGetRequest {
    /**
     * ID of the secret to retrieve
     */
    id: string;
}

export interface SecretsGetRequest {
    /**
     * IDs of the secrets to retrieve
     */
    ids: string[];
}

export interface SecretIdentifiersRequest {
    /**
     * Organization to retrieve all the secrets from
     */
    organizationId: string;
}

export interface SecretPutRequest {
    /**
     * ID of the secret to modify
     */
    id:   string;
    key:  string;
    note: string;
    /**
     * Organization ID of the secret to modify
     */
    organizationId: string;
    projectIds?:    string[] | null;
    value:          string;
}

export interface SyncRequest {
    /**
     * Exclude the subdomains from the response, defaults to false
     */
    excludeSubdomains?: boolean | null;
}

export interface ResponseForAPIKeyLoginResponse {
    /**
     * The response data. Populated if `success` is true.
     */
    data?: APIKeyLoginResponse | null;
    /**
     * A message for any error that may occur. Populated if `success` is false.
     */
    errorMessage?: null | string;
    /**
     * Whether or not the SDK request succeeded.
     */
    success: boolean;
}

export interface APIKeyLoginResponse {
    authenticated: boolean;
    /**
     * Whether or not the user is required to update their master password
     */
    forcePasswordReset: boolean;
    /**
     * TODO: What does this do?
     */
    resetMasterPassword: boolean;
    twoFactor?:          TwoFactorProviders | null;
}

export interface TwoFactorProviders {
    authenticator?: Authenticator | null;
    /**
     * Duo-backed 2fa
     */
    duo?: Duo | null;
    /**
     * Email 2fa
     */
    email?: Email | null;
    /**
     * Duo-backed 2fa operated by an organization the user is a member of
     */
    organizationDuo?: Duo | null;
    /**
     * Presence indicates the user has stored this device as bypassing 2fa
     */
    remember?: Remember | null;
    /**
     * WebAuthn-backed 2fa
     */
    webAuthn?: WebAuthn | null;
    /**
     * Yubikey-backed 2fa
     */
    yubiKey?: YubiKey | null;
}

export interface Authenticator {
}

export interface Duo {
    host:      string;
    signature: string;
}

export interface Email {
    /**
     * The email to request a 2fa TOTP for
     */
    email: string;
}

export interface Remember {
}

export interface WebAuthn {
}

export interface YubiKey {
    /**
     * Whether the stored yubikey supports near field communication
     */
    nfc: boolean;
}

export interface ResponseForPasswordLoginResponse {
    /**
     * The response data. Populated if `success` is true.
     */
    data?: PasswordLoginResponse | null;
    /**
     * A message for any error that may occur. Populated if `success` is false.
     */
    errorMessage?: null | string;
    /**
     * Whether or not the SDK request succeeded.
     */
    success: boolean;
}

export interface PasswordLoginResponse {
    authenticated: boolean;
    /**
     * The information required to present the user with a captcha challenge. Only present when
     * authentication fails due to requiring validation of a captcha challenge.
     */
    captcha?: CAPTCHAResponse | null;
    /**
     * Whether or not the user is required to update their master password
     */
    forcePasswordReset: boolean;
    /**
     * TODO: What does this do?
     */
    resetMasterPassword: boolean;
    /**
     * The available two factor authentication options. Present only when authentication fails
     * due to requiring a second authentication factor.
     */
    twoFactor?: TwoFactorProviders | null;
}

export interface CAPTCHAResponse {
    /**
     * hcaptcha site key
     */
    siteKey: string;
}

export interface ResponseForAccessTokenLoginResponse {
    /**
     * The response data. Populated if `success` is true.
     */
    data?: AccessTokenLoginResponse | null;
    /**
     * A message for any error that may occur. Populated if `success` is false.
     */
    errorMessage?: null | string;
    /**
     * Whether or not the SDK request succeeded.
     */
    success: boolean;
}

export interface AccessTokenLoginResponse {
    authenticated: boolean;
    /**
     * Whether or not the user is required to update their master password
     */
    forcePasswordReset: boolean;
    /**
     * TODO: What does this do?
     */
    resetMasterPassword: boolean;
    twoFactor?:          TwoFactorProviders | null;
}

export interface ResponseForSecretIdentifiersResponse {
    /**
     * The response data. Populated if `success` is true.
     */
    data?: SecretIdentifiersResponse | null;
    /**
     * A message for any error that may occur. Populated if `success` is false.
     */
    errorMessage?: null | string;
    /**
     * Whether or not the SDK request succeeded.
     */
    success: boolean;
}

export interface SecretIdentifiersResponse {
    data: SecretIdentifierResponse[];
}

export interface SecretIdentifierResponse {
    id:             string;
    key:            string;
    organizationId: string;
}

export interface ResponseForSecretResponse {
    /**
     * The response data. Populated if `success` is true.
     */
    data?: SecretResponse | null;
    /**
     * A message for any error that may occur. Populated if `success` is false.
     */
    errorMessage?: null | string;
    /**
     * Whether or not the SDK request succeeded.
     */
    success: boolean;
}

export interface SecretResponse {
    creationDate:   Date;
    id:             string;
    key:            string;
    note:           string;
    organizationId: string;
    projectId?:     null | string;
    revisionDate:   Date;
    value:          string;
}

export interface ResponseForSecretsResponse {
    /**
     * The response data. Populated if `success` is true.
     */
    data?: SecretsResponse | null;
    /**
     * A message for any error that may occur. Populated if `success` is false.
     */
    errorMessage?: null | string;
    /**
     * Whether or not the SDK request succeeded.
     */
    success: boolean;
}

export interface SecretsResponse {
    data: SecretResponse[];
}

export interface ResponseForSecretsDeleteResponse {
    /**
     * The response data. Populated if `success` is true.
     */
    data?: SecretsDeleteResponse | null;
    /**
     * A message for any error that may occur. Populated if `success` is false.
     */
    errorMessage?: null | string;
    /**
     * Whether or not the SDK request succeeded.
     */
    success: boolean;
}

export interface SecretsDeleteResponse {
    data: SecretDeleteResponse[];
}

export interface SecretDeleteResponse {
    error?: null | string;
    id:     string;
}

export interface ResponseForProjectResponse {
    /**
     * The response data. Populated if `success` is true.
     */
    data?: ProjectResponse | null;
    /**
     * A message for any error that may occur. Populated if `success` is false.
     */
    errorMessage?: null | string;
    /**
     * Whether or not the SDK request succeeded.
     */
    success: boolean;
}

export interface ProjectResponse {
    creationDate:   Date;
    id:             string;
    name:           string;
    organizationId: string;
    revisionDate:   Date;
}

export interface ResponseForProjectsResponse {
    /**
     * The response data. Populated if `success` is true.
     */
    data?: ProjectsResponse | null;
    /**
     * A message for any error that may occur. Populated if `success` is false.
     */
    errorMessage?: null | string;
    /**
     * Whether or not the SDK request succeeded.
     */
    success: boolean;
}

export interface ProjectsResponse {
    data: ProjectResponse[];
}

export interface ResponseForProjectsDeleteResponse {
    /**
     * The response data. Populated if `success` is true.
     */
    data?: ProjectsDeleteResponse | null;
    /**
     * A message for any error that may occur. Populated if `success` is false.
     */
    errorMessage?: null | string;
    /**
     * Whether or not the SDK request succeeded.
     */
    success: boolean;
}

export interface ProjectsDeleteResponse {
    data: ProjectDeleteResponse[];
}

export interface ProjectDeleteResponse {
    error?: null | string;
    id:     string;
}

export interface ResponseForFingerprintResponse {
    /**
     * The response data. Populated if `success` is true.
     */
    data?: FingerprintResponse | null;
    /**
     * A message for any error that may occur. Populated if `success` is false.
     */
    errorMessage?: null | string;
    /**
     * Whether or not the SDK request succeeded.
     */
    success: boolean;
}

export interface FingerprintResponse {
    fingerprint: string;
}

export interface ResponseForSyncResponse {
    /**
     * The response data. Populated if `success` is true.
     */
    data?: SyncResponse | null;
    /**
     * A message for any error that may occur. Populated if `success` is false.
     */
    errorMessage?: null | string;
    /**
     * Whether or not the SDK request succeeded.
     */
    success: boolean;
}

export interface SyncResponse {
    /**
     * List of ciphers accesible by the user
     */
    ciphers: CipherDetailsResponse[];
    /**
     * Data about the user, including their encryption keys and the organizations they are a
     * part of
     */
    profile: ProfileResponse;
}

export interface CipherDetailsResponse {
}

/**
 * Data about the user, including their encryption keys and the organizations they are a
 * part of
 */
export interface ProfileResponse {
    email:         string;
    id:            string;
    name:          string;
    organizations: ProfileOrganizationResponse[];
}

export interface ProfileOrganizationResponse {
    id: string;
}

export interface ResponseForUserAPIKeyResponse {
    /**
     * The response data. Populated if `success` is true.
     */
    data?: UserAPIKeyResponse | null;
    /**
     * A message for any error that may occur. Populated if `success` is false.
     */
    errorMessage?: null | string;
    /**
     * Whether or not the SDK request succeeded.
     */
    success: boolean;
}

export interface UserAPIKeyResponse {
    /**
     * The user's API key, which represents the client_secret portion of an oauth request.
     */
    apiKey: string;
}

// Converts JSON strings to/from your types
// and asserts the results of JSON.parse at runtime
export class Convert {
    public static toClientSettings(json: string): ClientSettings {
        return cast(JSON.parse(json), r("ClientSettings"));
    }

    public static clientSettingsToJson(value: ClientSettings): string {
        return JSON.stringify(uncast(value, r("ClientSettings")), null, 2);
    }

    public static toDeviceType(json: string): DeviceType {
        return cast(JSON.parse(json), r("DeviceType"));
    }

    public static deviceTypeToJson(value: DeviceType): string {
        return JSON.stringify(uncast(value, r("DeviceType")), null, 2);
    }

    public static toCommand(json: string): Command {
        return cast(JSON.parse(json), r("Command"));
    }

    public static commandToJson(value: Command): string {
        return JSON.stringify(uncast(value, r("Command")), null, 2);
    }

    public static toPasswordLoginRequest(json: string): PasswordLoginRequest {
        return cast(JSON.parse(json), r("PasswordLoginRequest"));
    }

    public static passwordLoginRequestToJson(value: PasswordLoginRequest): string {
        return JSON.stringify(uncast(value, r("PasswordLoginRequest")), null, 2);
    }

    public static toTwoFactorRequest(json: string): TwoFactorRequest {
        return cast(JSON.parse(json), r("TwoFactorRequest"));
    }

    public static twoFactorRequestToJson(value: TwoFactorRequest): string {
        return JSON.stringify(uncast(value, r("TwoFactorRequest")), null, 2);
    }

    public static toTwoFactorProvider(json: string): TwoFactorProvider {
        return cast(JSON.parse(json), r("TwoFactorProvider"));
    }

    public static twoFactorProviderToJson(value: TwoFactorProvider): string {
        return JSON.stringify(uncast(value, r("TwoFactorProvider")), null, 2);
    }

    public static toKdf(json: string): Kdf {
        return cast(JSON.parse(json), r("Kdf"));
    }

    public static kdfToJson(value: Kdf): string {
        return JSON.stringify(uncast(value, r("Kdf")), null, 2);
    }

    public static toAPIKeyLoginRequest(json: string): APIKeyLoginRequest {
        return cast(JSON.parse(json), r("APIKeyLoginRequest"));
    }

    public static aPIKeyLoginRequestToJson(value: APIKeyLoginRequest): string {
        return JSON.stringify(uncast(value, r("APIKeyLoginRequest")), null, 2);
    }

    public static toAccessTokenLoginRequest(json: string): AccessTokenLoginRequest {
        return cast(JSON.parse(json), r("AccessTokenLoginRequest"));
    }

    public static accessTokenLoginRequestToJson(value: AccessTokenLoginRequest): string {
        return JSON.stringify(uncast(value, r("AccessTokenLoginRequest")), null, 2);
    }

    public static toSecretVerificationRequest(json: string): SecretVerificationRequest {
        return cast(JSON.parse(json), r("SecretVerificationRequest"));
    }

    public static secretVerificationRequestToJson(value: SecretVerificationRequest): string {
        return JSON.stringify(uncast(value, r("SecretVerificationRequest")), null, 2);
    }

    public static toFingerprintRequest(json: string): FingerprintRequest {
        return cast(JSON.parse(json), r("FingerprintRequest"));
    }

    public static fingerprintRequestToJson(value: FingerprintRequest): string {
        return JSON.stringify(uncast(value, r("FingerprintRequest")), null, 2);
    }

    public static toSyncRequest(json: string): SyncRequest {
        return cast(JSON.parse(json), r("SyncRequest"));
    }

    public static syncRequestToJson(value: SyncRequest): string {
        return JSON.stringify(uncast(value, r("SyncRequest")), null, 2);
    }

    public static toSecretsCommand(json: string): SecretsCommand {
        return cast(JSON.parse(json), r("SecretsCommand"));
    }

    public static secretsCommandToJson(value: SecretsCommand): string {
        return JSON.stringify(uncast(value, r("SecretsCommand")), null, 2);
    }

    public static toSecretGetRequest(json: string): SecretGetRequest {
        return cast(JSON.parse(json), r("SecretGetRequest"));
    }

    public static secretGetRequestToJson(value: SecretGetRequest): string {
        return JSON.stringify(uncast(value, r("SecretGetRequest")), null, 2);
    }

    public static toSecretsGetRequest(json: string): SecretsGetRequest {
        return cast(JSON.parse(json), r("SecretsGetRequest"));
    }

    public static secretsGetRequestToJson(value: SecretsGetRequest): string {
        return JSON.stringify(uncast(value, r("SecretsGetRequest")), null, 2);
    }

    public static toSecretCreateRequest(json: string): SecretCreateRequest {
        return cast(JSON.parse(json), r("SecretCreateRequest"));
    }

    public static secretCreateRequestToJson(value: SecretCreateRequest): string {
        return JSON.stringify(uncast(value, r("SecretCreateRequest")), null, 2);
    }

    public static toSecretIdentifiersRequest(json: string): SecretIdentifiersRequest {
        return cast(JSON.parse(json), r("SecretIdentifiersRequest"));
    }

    public static secretIdentifiersRequestToJson(value: SecretIdentifiersRequest): string {
        return JSON.stringify(uncast(value, r("SecretIdentifiersRequest")), null, 2);
    }

    public static toSecretPutRequest(json: string): SecretPutRequest {
        return cast(JSON.parse(json), r("SecretPutRequest"));
    }

    public static secretPutRequestToJson(value: SecretPutRequest): string {
        return JSON.stringify(uncast(value, r("SecretPutRequest")), null, 2);
    }

    public static toSecretsDeleteRequest(json: string): SecretsDeleteRequest {
        return cast(JSON.parse(json), r("SecretsDeleteRequest"));
    }

    public static secretsDeleteRequestToJson(value: SecretsDeleteRequest): string {
        return JSON.stringify(uncast(value, r("SecretsDeleteRequest")), null, 2);
    }

    public static toProjectsCommand(json: string): ProjectsCommand {
        return cast(JSON.parse(json), r("ProjectsCommand"));
    }

    public static projectsCommandToJson(value: ProjectsCommand): string {
        return JSON.stringify(uncast(value, r("ProjectsCommand")), null, 2);
    }

    public static toProjectGetRequest(json: string): ProjectGetRequest {
        return cast(JSON.parse(json), r("ProjectGetRequest"));
    }

    public static projectGetRequestToJson(value: ProjectGetRequest): string {
        return JSON.stringify(uncast(value, r("ProjectGetRequest")), null, 2);
    }

    public static toProjectCreateRequest(json: string): ProjectCreateRequest {
        return cast(JSON.parse(json), r("ProjectCreateRequest"));
    }

    public static projectCreateRequestToJson(value: ProjectCreateRequest): string {
        return JSON.stringify(uncast(value, r("ProjectCreateRequest")), null, 2);
    }

    public static toProjectsListRequest(json: string): ProjectsListRequest {
        return cast(JSON.parse(json), r("ProjectsListRequest"));
    }

    public static projectsListRequestToJson(value: ProjectsListRequest): string {
        return JSON.stringify(uncast(value, r("ProjectsListRequest")), null, 2);
    }

    public static toProjectPutRequest(json: string): ProjectPutRequest {
        return cast(JSON.parse(json), r("ProjectPutRequest"));
    }

    public static projectPutRequestToJson(value: ProjectPutRequest): string {
        return JSON.stringify(uncast(value, r("ProjectPutRequest")), null, 2);
    }

    public static toProjectsDeleteRequest(json: string): ProjectsDeleteRequest {
        return cast(JSON.parse(json), r("ProjectsDeleteRequest"));
    }

    public static projectsDeleteRequestToJson(value: ProjectsDeleteRequest): string {
        return JSON.stringify(uncast(value, r("ProjectsDeleteRequest")), null, 2);
    }

    public static toResponseForAPIKeyLoginResponse(json: string): ResponseForAPIKeyLoginResponse {
        return cast(JSON.parse(json), r("ResponseForAPIKeyLoginResponse"));
    }

    public static responseForAPIKeyLoginResponseToJson(value: ResponseForAPIKeyLoginResponse): string {
        return JSON.stringify(uncast(value, r("ResponseForAPIKeyLoginResponse")), null, 2);
    }

    public static toAPIKeyLoginResponse(json: string): APIKeyLoginResponse {
        return cast(JSON.parse(json), r("APIKeyLoginResponse"));
    }

    public static aPIKeyLoginResponseToJson(value: APIKeyLoginResponse): string {
        return JSON.stringify(uncast(value, r("APIKeyLoginResponse")), null, 2);
    }

    public static toTwoFactorProviders(json: string): TwoFactorProviders {
        return cast(JSON.parse(json), r("TwoFactorProviders"));
    }

    public static twoFactorProvidersToJson(value: TwoFactorProviders): string {
        return JSON.stringify(uncast(value, r("TwoFactorProviders")), null, 2);
    }

    public static toAuthenticator(json: string): Authenticator {
        return cast(JSON.parse(json), r("Authenticator"));
    }

    public static authenticatorToJson(value: Authenticator): string {
        return JSON.stringify(uncast(value, r("Authenticator")), null, 2);
    }

    public static toEmail(json: string): Email {
        return cast(JSON.parse(json), r("Email"));
    }

    public static emailToJson(value: Email): string {
        return JSON.stringify(uncast(value, r("Email")), null, 2);
    }

    public static toDuo(json: string): Duo {
        return cast(JSON.parse(json), r("Duo"));
    }

    public static duoToJson(value: Duo): string {
        return JSON.stringify(uncast(value, r("Duo")), null, 2);
    }

    public static toYubiKey(json: string): YubiKey {
        return cast(JSON.parse(json), r("YubiKey"));
    }

    public static yubiKeyToJson(value: YubiKey): string {
        return JSON.stringify(uncast(value, r("YubiKey")), null, 2);
    }

    public static toRemember(json: string): Remember {
        return cast(JSON.parse(json), r("Remember"));
    }

    public static rememberToJson(value: Remember): string {
        return JSON.stringify(uncast(value, r("Remember")), null, 2);
    }

    public static toWebAuthn(json: string): WebAuthn {
        return cast(JSON.parse(json), r("WebAuthn"));
    }

    public static webAuthnToJson(value: WebAuthn): string {
        return JSON.stringify(uncast(value, r("WebAuthn")), null, 2);
    }

    public static toResponseForPasswordLoginResponse(json: string): ResponseForPasswordLoginResponse {
        return cast(JSON.parse(json), r("ResponseForPasswordLoginResponse"));
    }

    public static responseForPasswordLoginResponseToJson(value: ResponseForPasswordLoginResponse): string {
        return JSON.stringify(uncast(value, r("ResponseForPasswordLoginResponse")), null, 2);
    }

    public static toPasswordLoginResponse(json: string): PasswordLoginResponse {
        return cast(JSON.parse(json), r("PasswordLoginResponse"));
    }

    public static passwordLoginResponseToJson(value: PasswordLoginResponse): string {
        return JSON.stringify(uncast(value, r("PasswordLoginResponse")), null, 2);
    }

    public static toCAPTCHAResponse(json: string): CAPTCHAResponse {
        return cast(JSON.parse(json), r("CAPTCHAResponse"));
    }

    public static cAPTCHAResponseToJson(value: CAPTCHAResponse): string {
        return JSON.stringify(uncast(value, r("CAPTCHAResponse")), null, 2);
    }

    public static toResponseForAccessTokenLoginResponse(json: string): ResponseForAccessTokenLoginResponse {
        return cast(JSON.parse(json), r("ResponseForAccessTokenLoginResponse"));
    }

    public static responseForAccessTokenLoginResponseToJson(value: ResponseForAccessTokenLoginResponse): string {
        return JSON.stringify(uncast(value, r("ResponseForAccessTokenLoginResponse")), null, 2);
    }

    public static toAccessTokenLoginResponse(json: string): AccessTokenLoginResponse {
        return cast(JSON.parse(json), r("AccessTokenLoginResponse"));
    }

    public static accessTokenLoginResponseToJson(value: AccessTokenLoginResponse): string {
        return JSON.stringify(uncast(value, r("AccessTokenLoginResponse")), null, 2);
    }

    public static toResponseForSecretIdentifiersResponse(json: string): ResponseForSecretIdentifiersResponse {
        return cast(JSON.parse(json), r("ResponseForSecretIdentifiersResponse"));
    }

    public static responseForSecretIdentifiersResponseToJson(value: ResponseForSecretIdentifiersResponse): string {
        return JSON.stringify(uncast(value, r("ResponseForSecretIdentifiersResponse")), null, 2);
    }

    public static toSecretIdentifiersResponse(json: string): SecretIdentifiersResponse {
        return cast(JSON.parse(json), r("SecretIdentifiersResponse"));
    }

    public static secretIdentifiersResponseToJson(value: SecretIdentifiersResponse): string {
        return JSON.stringify(uncast(value, r("SecretIdentifiersResponse")), null, 2);
    }

    public static toSecretIdentifierResponse(json: string): SecretIdentifierResponse {
        return cast(JSON.parse(json), r("SecretIdentifierResponse"));
    }

    public static secretIdentifierResponseToJson(value: SecretIdentifierResponse): string {
        return JSON.stringify(uncast(value, r("SecretIdentifierResponse")), null, 2);
    }

    public static toResponseForSecretResponse(json: string): ResponseForSecretResponse {
        return cast(JSON.parse(json), r("ResponseForSecretResponse"));
    }

    public static responseForSecretResponseToJson(value: ResponseForSecretResponse): string {
        return JSON.stringify(uncast(value, r("ResponseForSecretResponse")), null, 2);
    }

    public static toSecretResponse(json: string): SecretResponse {
        return cast(JSON.parse(json), r("SecretResponse"));
    }

    public static secretResponseToJson(value: SecretResponse): string {
        return JSON.stringify(uncast(value, r("SecretResponse")), null, 2);
    }

    public static toResponseForSecretsResponse(json: string): ResponseForSecretsResponse {
        return cast(JSON.parse(json), r("ResponseForSecretsResponse"));
    }

    public static responseForSecretsResponseToJson(value: ResponseForSecretsResponse): string {
        return JSON.stringify(uncast(value, r("ResponseForSecretsResponse")), null, 2);
    }

    public static toSecretsResponse(json: string): SecretsResponse {
        return cast(JSON.parse(json), r("SecretsResponse"));
    }

    public static secretsResponseToJson(value: SecretsResponse): string {
        return JSON.stringify(uncast(value, r("SecretsResponse")), null, 2);
    }

    public static toResponseForSecretsDeleteResponse(json: string): ResponseForSecretsDeleteResponse {
        return cast(JSON.parse(json), r("ResponseForSecretsDeleteResponse"));
    }

    public static responseForSecretsDeleteResponseToJson(value: ResponseForSecretsDeleteResponse): string {
        return JSON.stringify(uncast(value, r("ResponseForSecretsDeleteResponse")), null, 2);
    }

    public static toSecretsDeleteResponse(json: string): SecretsDeleteResponse {
        return cast(JSON.parse(json), r("SecretsDeleteResponse"));
    }

    public static secretsDeleteResponseToJson(value: SecretsDeleteResponse): string {
        return JSON.stringify(uncast(value, r("SecretsDeleteResponse")), null, 2);
    }

    public static toSecretDeleteResponse(json: string): SecretDeleteResponse {
        return cast(JSON.parse(json), r("SecretDeleteResponse"));
    }

    public static secretDeleteResponseToJson(value: SecretDeleteResponse): string {
        return JSON.stringify(uncast(value, r("SecretDeleteResponse")), null, 2);
    }

    public static toResponseForProjectResponse(json: string): ResponseForProjectResponse {
        return cast(JSON.parse(json), r("ResponseForProjectResponse"));
    }

    public static responseForProjectResponseToJson(value: ResponseForProjectResponse): string {
        return JSON.stringify(uncast(value, r("ResponseForProjectResponse")), null, 2);
    }

    public static toProjectResponse(json: string): ProjectResponse {
        return cast(JSON.parse(json), r("ProjectResponse"));
    }

    public static projectResponseToJson(value: ProjectResponse): string {
        return JSON.stringify(uncast(value, r("ProjectResponse")), null, 2);
    }

    public static toResponseForProjectsResponse(json: string): ResponseForProjectsResponse {
        return cast(JSON.parse(json), r("ResponseForProjectsResponse"));
    }

    public static responseForProjectsResponseToJson(value: ResponseForProjectsResponse): string {
        return JSON.stringify(uncast(value, r("ResponseForProjectsResponse")), null, 2);
    }

    public static toProjectsResponse(json: string): ProjectsResponse {
        return cast(JSON.parse(json), r("ProjectsResponse"));
    }

    public static projectsResponseToJson(value: ProjectsResponse): string {
        return JSON.stringify(uncast(value, r("ProjectsResponse")), null, 2);
    }

    public static toResponseForProjectsDeleteResponse(json: string): ResponseForProjectsDeleteResponse {
        return cast(JSON.parse(json), r("ResponseForProjectsDeleteResponse"));
    }

    public static responseForProjectsDeleteResponseToJson(value: ResponseForProjectsDeleteResponse): string {
        return JSON.stringify(uncast(value, r("ResponseForProjectsDeleteResponse")), null, 2);
    }

    public static toProjectsDeleteResponse(json: string): ProjectsDeleteResponse {
        return cast(JSON.parse(json), r("ProjectsDeleteResponse"));
    }

    public static projectsDeleteResponseToJson(value: ProjectsDeleteResponse): string {
        return JSON.stringify(uncast(value, r("ProjectsDeleteResponse")), null, 2);
    }

    public static toProjectDeleteResponse(json: string): ProjectDeleteResponse {
        return cast(JSON.parse(json), r("ProjectDeleteResponse"));
    }

    public static projectDeleteResponseToJson(value: ProjectDeleteResponse): string {
        return JSON.stringify(uncast(value, r("ProjectDeleteResponse")), null, 2);
    }

    public static toResponseForFingerprintResponse(json: string): ResponseForFingerprintResponse {
        return cast(JSON.parse(json), r("ResponseForFingerprintResponse"));
    }

    public static responseForFingerprintResponseToJson(value: ResponseForFingerprintResponse): string {
        return JSON.stringify(uncast(value, r("ResponseForFingerprintResponse")), null, 2);
    }

    public static toFingerprintResponse(json: string): FingerprintResponse {
        return cast(JSON.parse(json), r("FingerprintResponse"));
    }

    public static fingerprintResponseToJson(value: FingerprintResponse): string {
        return JSON.stringify(uncast(value, r("FingerprintResponse")), null, 2);
    }

    public static toResponseForSyncResponse(json: string): ResponseForSyncResponse {
        return cast(JSON.parse(json), r("ResponseForSyncResponse"));
    }

    public static responseForSyncResponseToJson(value: ResponseForSyncResponse): string {
        return JSON.stringify(uncast(value, r("ResponseForSyncResponse")), null, 2);
    }

    public static toSyncResponse(json: string): SyncResponse {
        return cast(JSON.parse(json), r("SyncResponse"));
    }

    public static syncResponseToJson(value: SyncResponse): string {
        return JSON.stringify(uncast(value, r("SyncResponse")), null, 2);
    }

    public static toProfileResponse(json: string): ProfileResponse {
        return cast(JSON.parse(json), r("ProfileResponse"));
    }

    public static profileResponseToJson(value: ProfileResponse): string {
        return JSON.stringify(uncast(value, r("ProfileResponse")), null, 2);
    }

    public static toProfileOrganizationResponse(json: string): ProfileOrganizationResponse {
        return cast(JSON.parse(json), r("ProfileOrganizationResponse"));
    }

    public static profileOrganizationResponseToJson(value: ProfileOrganizationResponse): string {
        return JSON.stringify(uncast(value, r("ProfileOrganizationResponse")), null, 2);
    }

    public static toCipherDetailsResponse(json: string): CipherDetailsResponse {
        return cast(JSON.parse(json), r("CipherDetailsResponse"));
    }

    public static cipherDetailsResponseToJson(value: CipherDetailsResponse): string {
        return JSON.stringify(uncast(value, r("CipherDetailsResponse")), null, 2);
    }

    public static toResponseForUserAPIKeyResponse(json: string): ResponseForUserAPIKeyResponse {
        return cast(JSON.parse(json), r("ResponseForUserAPIKeyResponse"));
    }

    public static responseForUserAPIKeyResponseToJson(value: ResponseForUserAPIKeyResponse): string {
        return JSON.stringify(uncast(value, r("ResponseForUserAPIKeyResponse")), null, 2);
    }

    public static toUserAPIKeyResponse(json: string): UserAPIKeyResponse {
        return cast(JSON.parse(json), r("UserAPIKeyResponse"));
    }

    public static userAPIKeyResponseToJson(value: UserAPIKeyResponse): string {
        return JSON.stringify(uncast(value, r("UserAPIKeyResponse")), null, 2);
    }
}

function invalidValue(typ: any, val: any, key: any, parent: any = ''): never {
    const prettyTyp = prettyTypeName(typ);
    const parentText = parent ? ` on ${parent}` : '';
    const keyText = key ? ` for key "${key}"` : '';
    throw Error(`Invalid value${keyText}${parentText}. Expected ${prettyTyp} but got ${JSON.stringify(val)}`);
}

function prettyTypeName(typ: any): string {
    if (Array.isArray(typ)) {
        if (typ.length === 2 && typ[0] === undefined) {
            return `an optional ${prettyTypeName(typ[1])}`;
        } else {
            return `one of [${typ.map(a => { return prettyTypeName(a); }).join(", ")}]`;
        }
    } else if (typeof typ === "object" && typ.literal !== undefined) {
        return typ.literal;
    } else {
        return typeof typ;
    }
}

function jsonToJSProps(typ: any): any {
    if (typ.jsonToJS === undefined) {
        const map: any = {};
        typ.props.forEach((p: any) => map[p.json] = { key: p.js, typ: p.typ });
        typ.jsonToJS = map;
    }
    return typ.jsonToJS;
}

function jsToJSONProps(typ: any): any {
    if (typ.jsToJSON === undefined) {
        const map: any = {};
        typ.props.forEach((p: any) => map[p.js] = { key: p.json, typ: p.typ });
        typ.jsToJSON = map;
    }
    return typ.jsToJSON;
}

function transform(val: any, typ: any, getProps: any, key: any = '', parent: any = ''): any {
    function transformPrimitive(typ: string, val: any): any {
        if (typeof typ === typeof val) return val;
        return invalidValue(typ, val, key, parent);
    }

    function transformUnion(typs: any[], val: any): any {
        // val must validate against one typ in typs
        const l = typs.length;
        for (let i = 0; i < l; i++) {
            const typ = typs[i];
            try {
                return transform(val, typ, getProps);
            } catch (_) {}
        }
        return invalidValue(typs, val, key, parent);
    }

    function transformEnum(cases: string[], val: any): any {
        if (cases.indexOf(val) !== -1) return val;
        return invalidValue(cases.map(a => { return l(a); }), val, key, parent);
    }

    function transformArray(typ: any, val: any): any {
        // val must be an array with no invalid elements
        if (!Array.isArray(val)) return invalidValue(l("array"), val, key, parent);
        return val.map(el => transform(el, typ, getProps));
    }

    function transformDate(val: any): any {
        if (val === null) {
            return null;
        }
        const d = new Date(val);
        if (isNaN(d.valueOf())) {
            return invalidValue(l("Date"), val, key, parent);
        }
        return d;
    }

    function transformObject(props: { [k: string]: any }, additional: any, val: any): any {
        if (val === null || typeof val !== "object" || Array.isArray(val)) {
            return invalidValue(l(ref || "object"), val, key, parent);
        }
        const result: any = {};
        Object.getOwnPropertyNames(props).forEach(key => {
            const prop = props[key];
            const v = Object.prototype.hasOwnProperty.call(val, key) ? val[key] : undefined;
            result[prop.key] = transform(v, prop.typ, getProps, key, ref);
        });
        Object.getOwnPropertyNames(val).forEach(key => {
            if (!Object.prototype.hasOwnProperty.call(props, key)) {
                result[key] = transform(val[key], additional, getProps, key, ref);
            }
        });
        return result;
    }

    if (typ === "any") return val;
    if (typ === null) {
        if (val === null) return val;
        return invalidValue(typ, val, key, parent);
    }
    if (typ === false) return invalidValue(typ, val, key, parent);
    let ref: any = undefined;
    while (typeof typ === "object" && typ.ref !== undefined) {
        ref = typ.ref;
        typ = typeMap[typ.ref];
    }
    if (Array.isArray(typ)) return transformEnum(typ, val);
    if (typeof typ === "object") {
        return typ.hasOwnProperty("unionMembers") ? transformUnion(typ.unionMembers, val)
            : typ.hasOwnProperty("arrayItems")    ? transformArray(typ.arrayItems, val)
            : typ.hasOwnProperty("props")         ? transformObject(getProps(typ), typ.additional, val)
            : invalidValue(typ, val, key, parent);
    }
    // Numbers can be parsed by Date but shouldn't be.
    if (typ === Date && typeof val !== "number") return transformDate(val);
    return transformPrimitive(typ, val);
}

function cast<T>(val: any, typ: any): T {
    return transform(val, typ, jsonToJSProps);
}

function uncast<T>(val: T, typ: any): any {
    return transform(val, typ, jsToJSONProps);
}

function l(typ: any) {
    return { literal: typ };
}

function a(typ: any) {
    return { arrayItems: typ };
}

function u(...typs: any[]) {
    return { unionMembers: typs };
}

function o(props: any[], additional: any) {
    return { props, additional };
}

function m(additional: any) {
    return { props: [], additional };
}

function r(name: string) {
    return { ref: name };
}

const typeMap: any = {
    "ClientSettings": o([
        { json: "apiUrl", js: "apiUrl", typ: u(undefined, "") },
        { json: "deviceType", js: "deviceType", typ: u(undefined, r("DeviceType")) },
        { json: "identityUrl", js: "identityUrl", typ: u(undefined, "") },
        { json: "userAgent", js: "userAgent", typ: u(undefined, "") },
    ], false),
    "Command": o([
        { json: "passwordLogin", js: "passwordLogin", typ: u(undefined, r("PasswordLoginRequest")) },
        { json: "apiKeyLogin", js: "apiKeyLogin", typ: u(undefined, r("APIKeyLoginRequest")) },
        { json: "accessTokenLogin", js: "accessTokenLogin", typ: u(undefined, r("AccessTokenLoginRequest")) },
        { json: "getUserApiKey", js: "getUserApiKey", typ: u(undefined, r("SecretVerificationRequest")) },
        { json: "fingerprint", js: "fingerprint", typ: u(undefined, r("FingerprintRequest")) },
        { json: "sync", js: "sync", typ: u(undefined, r("SyncRequest")) },
        { json: "secrets", js: "secrets", typ: u(undefined, r("SecretsCommand")) },
        { json: "projects", js: "projects", typ: u(undefined, r("ProjectsCommand")) },
    ], false),
    "AccessTokenLoginRequest": o([
        { json: "accessToken", js: "accessToken", typ: "" },
    ], false),
    "APIKeyLoginRequest": o([
        { json: "clientId", js: "clientId", typ: "" },
        { json: "clientSecret", js: "clientSecret", typ: "" },
        { json: "password", js: "password", typ: "" },
    ], false),
    "FingerprintRequest": o([
        { json: "fingerprintMaterial", js: "fingerprintMaterial", typ: "" },
        { json: "publicKey", js: "publicKey", typ: "" },
    ], false),
    "SecretVerificationRequest": o([
        { json: "masterPassword", js: "masterPassword", typ: u(undefined, u(null, "")) },
        { json: "otp", js: "otp", typ: u(undefined, u(null, "")) },
    ], false),
    "PasswordLoginRequest": o([
        { json: "email", js: "email", typ: "" },
        { json: "kdf", js: "kdf", typ: r("Kdf") },
        { json: "password", js: "password", typ: "" },
        { json: "twoFactor", js: "twoFactor", typ: u(undefined, u(r("TwoFactorRequest"), null)) },
    ], false),
    "Kdf": o([
        { json: "pBKDF2", js: "pBKDF2", typ: u(undefined, r("PBKDF2")) },
        { json: "argon2id", js: "argon2id", typ: u(undefined, r("Argon2ID")) },
    ], false),
    "Argon2ID": o([
        { json: "iterations", js: "iterations", typ: 0 },
        { json: "memory", js: "memory", typ: 0 },
        { json: "parallelism", js: "parallelism", typ: 0 },
    ], false),
    "PBKDF2": o([
        { json: "iterations", js: "iterations", typ: 0 },
    ], false),
    "TwoFactorRequest": o([
        { json: "provider", js: "provider", typ: r("TwoFactorProvider") },
        { json: "remember", js: "remember", typ: true },
        { json: "token", js: "token", typ: "" },
    ], false),
    "ProjectsCommand": o([
        { json: "get", js: "get", typ: u(undefined, r("ProjectGetRequest")) },
        { json: "create", js: "create", typ: u(undefined, r("ProjectCreateRequest")) },
        { json: "list", js: "list", typ: u(undefined, r("ProjectsListRequest")) },
        { json: "update", js: "update", typ: u(undefined, r("ProjectPutRequest")) },
        { json: "delete", js: "delete", typ: u(undefined, r("ProjectsDeleteRequest")) },
    ], false),
    "ProjectCreateRequest": o([
        { json: "name", js: "name", typ: "" },
        { json: "organizationId", js: "organizationId", typ: "" },
    ], false),
    "ProjectsDeleteRequest": o([
        { json: "ids", js: "ids", typ: a("") },
    ], false),
    "ProjectGetRequest": o([
        { json: "id", js: "id", typ: "" },
    ], false),
    "ProjectsListRequest": o([
        { json: "organizationId", js: "organizationId", typ: "" },
    ], false),
    "ProjectPutRequest": o([
        { json: "id", js: "id", typ: "" },
        { json: "name", js: "name", typ: "" },
        { json: "organizationId", js: "organizationId", typ: "" },
    ], false),
    "SecretsCommand": o([
        { json: "get", js: "get", typ: u(undefined, r("SecretGetRequest")) },
        { json: "getByIds", js: "getByIds", typ: u(undefined, r("SecretsGetRequest")) },
        { json: "create", js: "create", typ: u(undefined, r("SecretCreateRequest")) },
        { json: "list", js: "list", typ: u(undefined, r("SecretIdentifiersRequest")) },
        { json: "update", js: "update", typ: u(undefined, r("SecretPutRequest")) },
        { json: "delete", js: "delete", typ: u(undefined, r("SecretsDeleteRequest")) },
    ], false),
    "SecretCreateRequest": o([
        { json: "key", js: "key", typ: "" },
        { json: "note", js: "note", typ: "" },
        { json: "organizationId", js: "organizationId", typ: "" },
        { json: "projectIds", js: "projectIds", typ: u(undefined, u(a(""), null)) },
        { json: "value", js: "value", typ: "" },
    ], false),
    "SecretsDeleteRequest": o([
        { json: "ids", js: "ids", typ: a("") },
    ], false),
    "SecretGetRequest": o([
        { json: "id", js: "id", typ: "" },
    ], false),
    "SecretsGetRequest": o([
        { json: "ids", js: "ids", typ: a("") },
    ], false),
    "SecretIdentifiersRequest": o([
        { json: "organizationId", js: "organizationId", typ: "" },
    ], false),
    "SecretPutRequest": o([
        { json: "id", js: "id", typ: "" },
        { json: "key", js: "key", typ: "" },
        { json: "note", js: "note", typ: "" },
        { json: "organizationId", js: "organizationId", typ: "" },
        { json: "projectIds", js: "projectIds", typ: u(undefined, u(a(""), null)) },
        { json: "value", js: "value", typ: "" },
    ], false),
    "SyncRequest": o([
        { json: "excludeSubdomains", js: "excludeSubdomains", typ: u(undefined, u(true, null)) },
    ], false),
    "ResponseForAPIKeyLoginResponse": o([
        { json: "data", js: "data", typ: u(undefined, u(r("APIKeyLoginResponse"), null)) },
        { json: "errorMessage", js: "errorMessage", typ: u(undefined, u(null, "")) },
        { json: "success", js: "success", typ: true },
    ], false),
    "APIKeyLoginResponse": o([
        { json: "authenticated", js: "authenticated", typ: true },
        { json: "forcePasswordReset", js: "forcePasswordReset", typ: true },
        { json: "resetMasterPassword", js: "resetMasterPassword", typ: true },
        { json: "twoFactor", js: "twoFactor", typ: u(undefined, u(r("TwoFactorProviders"), null)) },
    ], false),
    "TwoFactorProviders": o([
        { json: "authenticator", js: "authenticator", typ: u(undefined, u(r("Authenticator"), null)) },
        { json: "duo", js: "duo", typ: u(undefined, u(r("Duo"), null)) },
        { json: "email", js: "email", typ: u(undefined, u(r("Email"), null)) },
        { json: "organizationDuo", js: "organizationDuo", typ: u(undefined, u(r("Duo"), null)) },
        { json: "remember", js: "remember", typ: u(undefined, u(r("Remember"), null)) },
        { json: "webAuthn", js: "webAuthn", typ: u(undefined, u(r("WebAuthn"), null)) },
        { json: "yubiKey", js: "yubiKey", typ: u(undefined, u(r("YubiKey"), null)) },
    ], false),
    "Authenticator": o([
    ], false),
    "Duo": o([
        { json: "host", js: "host", typ: "" },
        { json: "signature", js: "signature", typ: "" },
    ], false),
    "Email": o([
        { json: "email", js: "email", typ: "" },
    ], false),
    "Remember": o([
    ], false),
    "WebAuthn": o([
    ], false),
    "YubiKey": o([
        { json: "nfc", js: "nfc", typ: true },
    ], false),
    "ResponseForPasswordLoginResponse": o([
        { json: "data", js: "data", typ: u(undefined, u(r("PasswordLoginResponse"), null)) },
        { json: "errorMessage", js: "errorMessage", typ: u(undefined, u(null, "")) },
        { json: "success", js: "success", typ: true },
    ], false),
    "PasswordLoginResponse": o([
        { json: "authenticated", js: "authenticated", typ: true },
        { json: "captcha", js: "captcha", typ: u(undefined, u(r("CAPTCHAResponse"), null)) },
        { json: "forcePasswordReset", js: "forcePasswordReset", typ: true },
        { json: "resetMasterPassword", js: "resetMasterPassword", typ: true },
        { json: "twoFactor", js: "twoFactor", typ: u(undefined, u(r("TwoFactorProviders"), null)) },
    ], false),
    "CAPTCHAResponse": o([
        { json: "siteKey", js: "siteKey", typ: "" },
    ], false),
    "ResponseForAccessTokenLoginResponse": o([
        { json: "data", js: "data", typ: u(undefined, u(r("AccessTokenLoginResponse"), null)) },
        { json: "errorMessage", js: "errorMessage", typ: u(undefined, u(null, "")) },
        { json: "success", js: "success", typ: true },
    ], false),
    "AccessTokenLoginResponse": o([
        { json: "authenticated", js: "authenticated", typ: true },
        { json: "forcePasswordReset", js: "forcePasswordReset", typ: true },
        { json: "resetMasterPassword", js: "resetMasterPassword", typ: true },
        { json: "twoFactor", js: "twoFactor", typ: u(undefined, u(r("TwoFactorProviders"), null)) },
    ], false),
    "ResponseForSecretIdentifiersResponse": o([
        { json: "data", js: "data", typ: u(undefined, u(r("SecretIdentifiersResponse"), null)) },
        { json: "errorMessage", js: "errorMessage", typ: u(undefined, u(null, "")) },
        { json: "success", js: "success", typ: true },
    ], false),
    "SecretIdentifiersResponse": o([
        { json: "data", js: "data", typ: a(r("SecretIdentifierResponse")) },
    ], false),
    "SecretIdentifierResponse": o([
        { json: "id", js: "id", typ: "" },
        { json: "key", js: "key", typ: "" },
        { json: "organizationId", js: "organizationId", typ: "" },
    ], false),
    "ResponseForSecretResponse": o([
        { json: "data", js: "data", typ: u(undefined, u(r("SecretResponse"), null)) },
        { json: "errorMessage", js: "errorMessage", typ: u(undefined, u(null, "")) },
        { json: "success", js: "success", typ: true },
    ], false),
    "SecretResponse": o([
        { json: "creationDate", js: "creationDate", typ: Date },
        { json: "id", js: "id", typ: "" },
        { json: "key", js: "key", typ: "" },
        { json: "note", js: "note", typ: "" },
        { json: "organizationId", js: "organizationId", typ: "" },
        { json: "projectId", js: "projectId", typ: u(undefined, u(null, "")) },
        { json: "revisionDate", js: "revisionDate", typ: Date },
        { json: "value", js: "value", typ: "" },
    ], false),
    "ResponseForSecretsResponse": o([
        { json: "data", js: "data", typ: u(undefined, u(r("SecretsResponse"), null)) },
        { json: "errorMessage", js: "errorMessage", typ: u(undefined, u(null, "")) },
        { json: "success", js: "success", typ: true },
    ], false),
    "SecretsResponse": o([
        { json: "data", js: "data", typ: a(r("SecretResponse")) },
    ], false),
    "ResponseForSecretsDeleteResponse": o([
        { json: "data", js: "data", typ: u(undefined, u(r("SecretsDeleteResponse"), null)) },
        { json: "errorMessage", js: "errorMessage", typ: u(undefined, u(null, "")) },
        { json: "success", js: "success", typ: true },
    ], false),
    "SecretsDeleteResponse": o([
        { json: "data", js: "data", typ: a(r("SecretDeleteResponse")) },
    ], false),
    "SecretDeleteResponse": o([
        { json: "error", js: "error", typ: u(undefined, u(null, "")) },
        { json: "id", js: "id", typ: "" },
    ], false),
    "ResponseForProjectResponse": o([
        { json: "data", js: "data", typ: u(undefined, u(r("ProjectResponse"), null)) },
        { json: "errorMessage", js: "errorMessage", typ: u(undefined, u(null, "")) },
        { json: "success", js: "success", typ: true },
    ], false),
    "ProjectResponse": o([
        { json: "creationDate", js: "creationDate", typ: Date },
        { json: "id", js: "id", typ: "" },
        { json: "name", js: "name", typ: "" },
        { json: "organizationId", js: "organizationId", typ: "" },
        { json: "revisionDate", js: "revisionDate", typ: Date },
    ], false),
    "ResponseForProjectsResponse": o([
        { json: "data", js: "data", typ: u(undefined, u(r("ProjectsResponse"), null)) },
        { json: "errorMessage", js: "errorMessage", typ: u(undefined, u(null, "")) },
        { json: "success", js: "success", typ: true },
    ], false),
    "ProjectsResponse": o([
        { json: "data", js: "data", typ: a(r("ProjectResponse")) },
    ], false),
    "ResponseForProjectsDeleteResponse": o([
        { json: "data", js: "data", typ: u(undefined, u(r("ProjectsDeleteResponse"), null)) },
        { json: "errorMessage", js: "errorMessage", typ: u(undefined, u(null, "")) },
        { json: "success", js: "success", typ: true },
    ], false),
    "ProjectsDeleteResponse": o([
        { json: "data", js: "data", typ: a(r("ProjectDeleteResponse")) },
    ], false),
    "ProjectDeleteResponse": o([
        { json: "error", js: "error", typ: u(undefined, u(null, "")) },
        { json: "id", js: "id", typ: "" },
    ], false),
    "ResponseForFingerprintResponse": o([
        { json: "data", js: "data", typ: u(undefined, u(r("FingerprintResponse"), null)) },
        { json: "errorMessage", js: "errorMessage", typ: u(undefined, u(null, "")) },
        { json: "success", js: "success", typ: true },
    ], false),
    "FingerprintResponse": o([
        { json: "fingerprint", js: "fingerprint", typ: "" },
    ], false),
    "ResponseForSyncResponse": o([
        { json: "data", js: "data", typ: u(undefined, u(r("SyncResponse"), null)) },
        { json: "errorMessage", js: "errorMessage", typ: u(undefined, u(null, "")) },
        { json: "success", js: "success", typ: true },
    ], false),
    "SyncResponse": o([
        { json: "ciphers", js: "ciphers", typ: a(r("CipherDetailsResponse")) },
        { json: "profile", js: "profile", typ: r("ProfileResponse") },
    ], false),
    "CipherDetailsResponse": o([
    ], false),
    "ProfileResponse": o([
        { json: "email", js: "email", typ: "" },
        { json: "id", js: "id", typ: "" },
        { json: "name", js: "name", typ: "" },
        { json: "organizations", js: "organizations", typ: a(r("ProfileOrganizationResponse")) },
    ], false),
    "ProfileOrganizationResponse": o([
        { json: "id", js: "id", typ: "" },
    ], false),
    "ResponseForUserAPIKeyResponse": o([
        { json: "data", js: "data", typ: u(undefined, u(r("UserAPIKeyResponse"), null)) },
        { json: "errorMessage", js: "errorMessage", typ: u(undefined, u(null, "")) },
        { json: "success", js: "success", typ: true },
    ], false),
    "UserAPIKeyResponse": o([
        { json: "apiKey", js: "apiKey", typ: "" },
    ], false),
    "DeviceType": [
        "Android",
        "AndroidAmazon",
        "ChromeBrowser",
        "ChromeExtension",
        "EdgeBrowser",
        "EdgeExtension",
        "FirefoxBrowser",
        "FirefoxExtension",
        "IEBrowser",
        "iOS",
        "LinuxDesktop",
        "MacOsDesktop",
        "OperaBrowser",
        "OperaExtension",
        "SDK",
        "SafariBrowser",
        "SafariExtension",
        "UWP",
        "UnknownBrowser",
        "VivaldiBrowser",
        "VivaldiExtension",
        "WindowsDesktop",
    ],
    "TwoFactorProvider": [
        "Authenticator",
        "Duo",
        "Email",
        "OrganizationDuo",
        "Remember",
        "U2f",
        "WebAuthn",
        "Yubikey",
    ],
};

