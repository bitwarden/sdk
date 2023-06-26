// To parse this data:
//
//   import { Convert, ClientSettings, Command, ResponseForAPIKeyLoginResponse, ResponseForPasswordLoginResponse, ResponseForSecretIdentifiersResponse, ResponseForSecretResponse, ResponseForSecretsDeleteResponse, ResponseForSyncResponse, ResponseForUserAPIKeyResponse } from "./file";
//
//   const clientSettings = Convert.toClientSettings(json);
//   const command = Convert.toCommand(json);
//   const responseForAPIKeyLoginResponse = Convert.toResponseForAPIKeyLoginResponse(json);
//   const responseForPasswordLoginResponse = Convert.toResponseForPasswordLoginResponse(json);
//   const responseForSecretIdentifiersResponse = Convert.toResponseForSecretIdentifiersResponse(json);
//   const responseForSecretResponse = Convert.toResponseForSecretResponse(json);
//   const responseForSecretsDeleteResponse = Convert.toResponseForSecretsDeleteResponse(json);
//   const responseForSyncResponse = Convert.toResponseForSyncResponse(json);
//   const responseForUserAPIKeyResponse = Convert.toResponseForUserAPIKeyResponse(json);
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
    apiUrl: string;
    /**
     * Device type to send to Bitwarden. Defaults to SDK
     */
    deviceType: DeviceType;
    /**
     * The identity url of the targeted Bitwarden instance. Defaults to
     * `https://identity.bitwarden.com`
     */
    identityUrl: string;
    /**
     * The user_agent to sent to Bitwarden. Defaults to `Bitwarden Rust-SDK`
     */
    userAgent: string;
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
 * Returns: [PasswordLoginResponse](crate::sdk::auth::response::PasswordLoginResponse)
 *
 * Login with API Key
 *
 * This command is for initiating an authentication handshake with Bitwarden.
 *
 * Returns: [ApiKeyLoginResponse](crate::sdk::auth::response::ApiKeyLoginResponse)
 *
 * Login with Secrets Manager Access Token
 *
 * This command is for initiating an authentication handshake with Bitwarden.
 *
 * Returns: [ApiKeyLoginResponse](crate::sdk::auth::response::ApiKeyLoginResponse)
 *
 * > Requires Authentication Get the API key of the currently authenticated user
 *
 * Returns:
 * [UserApiKeyResponse](crate::sdk::response::user_api_key_response::UserApiKeyResponse)
 *
 * Get the user's passphrase
 *
 * Returns: String
 *
 * > Requires Authentication Retrieve all user data, ciphers and organizations the user is a
 * part of
 *
 * Returns: [SyncResponse](crate::sdk::response::sync_response::SyncResponse)
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
     * The user's public key
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
     * Bitwarden account master password
     */
    password: string;
}

/**
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at
 * least once Retrieve a project by the provided identifier
 *
 * Returns: [ProjectResponse](crate::sdk::response::projects_response::ProjectResponse)
 *
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at
 * least once Creates a new project in the provided organization using the given data
 *
 * Returns: [ProjectResponse](crate::sdk::response::projects_response::ProjectResponse)
 *
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at
 * least once Lists all projects of the given organization
 *
 * Returns: [ProjectsResponse](crate::sdk::response::projects_response::ProjectsResponse)
 *
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at
 * least once Updates an existing project with the provided ID using the given data
 *
 * Returns: [ProjectResponse](crate::sdk::response::projects_response::ProjectResponse)
 *
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at
 * least once Deletes all the projects whose IDs match the provided ones
 *
 * Returns:
 * [ProjectsDeleteResponse](crate::sdk::response::projects_response::ProjectsDeleteResponse)
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
 * Returns: [SecretResponse](crate::sdk::response::secrets_response::SecretResponse)
 *
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at
 * least once Creates a new secret in the provided organization using the given data
 *
 * Returns: [SecretResponse](crate::sdk::response::secrets_response::SecretResponse)
 *
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at
 * least once Lists all secret identifiers of the given organization, to then retrieve each
 * secret, use `CreateSecret`
 *
 * Returns:
 * [SecretIdentifiersResponse](crate::sdk::response::secrets_response::SecretIdentifiersResponse)
 *
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at
 * least once Updates an existing secret with the provided ID using the given data
 *
 * Returns: [SecretResponse](crate::sdk::response::secrets_response::SecretResponse)
 *
 * > Requires Authentication > Requires using an Access Token for login or calling Sync at
 * least once Deletes all the secrets whose IDs match the provided ones
 *
 * Returns:
 * [SecretsDeleteResponse](crate::sdk::response::secrets_response::SecretsDeleteResponse)
 */
export interface SecretsCommand {
    get?:    SecretGetRequest;
    create?: SecretCreateRequest;
    list?:   SecretIdentifiersRequest;
    update?: SecretPutRequest;
    delete?: SecretsDeleteRequest;
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
    twoFactor?:          APIKeyLoginResponseTwoFactorProviders | null;
}

export interface APIKeyLoginResponseTwoFactorProviders {
    authenticator?: PurpleAuthenticator | null;
    /**
     * Duo-backed 2fa
     */
    duo?: PurpleDuo | null;
    /**
     * Email 2fa
     */
    email?: PurpleEmail | null;
    /**
     * Duo-backed 2fa operated by an organization the user is a member of
     */
    organizationDuo?: PurpleDuo | null;
    /**
     * Presence indicates the user has stored this device as bypassing 2fa
     */
    remember?: PurpleRemember | null;
    /**
     * WebAuthn-backed 2fa
     */
    webAuthn?: PurpleWebAuthn | null;
    /**
     * Yubikey-backed 2fa
     */
    yubiKey?: PurpleYubiKey | null;
}

export interface PurpleAuthenticator {
}

export interface PurpleDuo {
    host:      string;
    signature: string;
}

export interface PurpleEmail {
    /**
     * The email to request a 2fa TOTP for
     */
    email: string;
}

export interface PurpleRemember {
}

export interface PurpleWebAuthn {
}

export interface PurpleYubiKey {
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
    twoFactor?: PasswordLoginResponseTwoFactorProviders | null;
}

export interface CAPTCHAResponse {
    /**
     * hcaptcha site key
     */
    siteKey: string;
}

export interface PasswordLoginResponseTwoFactorProviders {
    authenticator?: FluffyAuthenticator | null;
    /**
     * Duo-backed 2fa
     */
    duo?: FluffyDuo | null;
    /**
     * Email 2fa
     */
    email?: FluffyEmail | null;
    /**
     * Duo-backed 2fa operated by an organization the user is a member of
     */
    organizationDuo?: FluffyDuo | null;
    /**
     * Presence indicates the user has stored this device as bypassing 2fa
     */
    remember?: FluffyRemember | null;
    /**
     * WebAuthn-backed 2fa
     */
    webAuthn?: FluffyWebAuthn | null;
    /**
     * Yubikey-backed 2fa
     */
    yubiKey?: FluffyYubiKey | null;
}

export interface FluffyAuthenticator {
}

export interface FluffyDuo {
    host:      string;
    signature: string;
}

export interface FluffyEmail {
    /**
     * The email to request a 2fa TOTP for
     */
    email: string;
}

export interface FluffyRemember {
}

export interface FluffyWebAuthn {
}

export interface FluffyYubiKey {
    /**
     * Whether the stored yubikey supports near field communication
     */
    nfc: boolean;
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
    creationDate:   string;
    id:             string;
    key:            string;
    note:           string;
    object:         string;
    organizationId: string;
    projectId?:     null | string;
    revisionDate:   string;
    value:          string;
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

    public static toCommand(json: string): Command {
        return cast(JSON.parse(json), r("Command"));
    }

    public static commandToJson(value: Command): string {
        return JSON.stringify(uncast(value, r("Command")), null, 2);
    }

    public static toResponseForAPIKeyLoginResponse(json: string): ResponseForAPIKeyLoginResponse {
        return cast(JSON.parse(json), r("ResponseForAPIKeyLoginResponse"));
    }

    public static responseForAPIKeyLoginResponseToJson(value: ResponseForAPIKeyLoginResponse): string {
        return JSON.stringify(uncast(value, r("ResponseForAPIKeyLoginResponse")), null, 2);
    }

    public static toResponseForPasswordLoginResponse(json: string): ResponseForPasswordLoginResponse {
        return cast(JSON.parse(json), r("ResponseForPasswordLoginResponse"));
    }

    public static responseForPasswordLoginResponseToJson(value: ResponseForPasswordLoginResponse): string {
        return JSON.stringify(uncast(value, r("ResponseForPasswordLoginResponse")), null, 2);
    }

    public static toResponseForSecretIdentifiersResponse(json: string): ResponseForSecretIdentifiersResponse {
        return cast(JSON.parse(json), r("ResponseForSecretIdentifiersResponse"));
    }

    public static responseForSecretIdentifiersResponseToJson(value: ResponseForSecretIdentifiersResponse): string {
        return JSON.stringify(uncast(value, r("ResponseForSecretIdentifiersResponse")), null, 2);
    }

    public static toResponseForSecretResponse(json: string): ResponseForSecretResponse {
        return cast(JSON.parse(json), r("ResponseForSecretResponse"));
    }

    public static responseForSecretResponseToJson(value: ResponseForSecretResponse): string {
        return JSON.stringify(uncast(value, r("ResponseForSecretResponse")), null, 2);
    }

    public static toResponseForSecretsDeleteResponse(json: string): ResponseForSecretsDeleteResponse {
        return cast(JSON.parse(json), r("ResponseForSecretsDeleteResponse"));
    }

    public static responseForSecretsDeleteResponseToJson(value: ResponseForSecretsDeleteResponse): string {
        return JSON.stringify(uncast(value, r("ResponseForSecretsDeleteResponse")), null, 2);
    }

    public static toResponseForSyncResponse(json: string): ResponseForSyncResponse {
        return cast(JSON.parse(json), r("ResponseForSyncResponse"));
    }

    public static responseForSyncResponseToJson(value: ResponseForSyncResponse): string {
        return JSON.stringify(uncast(value, r("ResponseForSyncResponse")), null, 2);
    }

    public static toResponseForUserAPIKeyResponse(json: string): ResponseForUserAPIKeyResponse {
        return cast(JSON.parse(json), r("ResponseForUserAPIKeyResponse"));
    }

    public static responseForUserAPIKeyResponseToJson(value: ResponseForUserAPIKeyResponse): string {
        return JSON.stringify(uncast(value, r("ResponseForUserAPIKeyResponse")), null, 2);
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
        { json: "apiUrl", js: "apiUrl", typ: "" },
        { json: "deviceType", js: "deviceType", typ: r("DeviceType") },
        { json: "identityUrl", js: "identityUrl", typ: "" },
        { json: "userAgent", js: "userAgent", typ: "" },
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
        { json: "password", js: "password", typ: "" },
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
    "SecretIdentifiersRequest": o([
        { json: "organizationId", js: "organizationId", typ: "" },
    ], false),
    "SecretPutRequest": o([
        { json: "id", js: "id", typ: "" },
        { json: "key", js: "key", typ: "" },
        { json: "note", js: "note", typ: "" },
        { json: "organizationId", js: "organizationId", typ: "" },
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
        { json: "twoFactor", js: "twoFactor", typ: u(undefined, u(r("APIKeyLoginResponseTwoFactorProviders"), null)) },
    ], false),
    "APIKeyLoginResponseTwoFactorProviders": o([
        { json: "authenticator", js: "authenticator", typ: u(undefined, u(r("PurpleAuthenticator"), null)) },
        { json: "duo", js: "duo", typ: u(undefined, u(r("PurpleDuo"), null)) },
        { json: "email", js: "email", typ: u(undefined, u(r("PurpleEmail"), null)) },
        { json: "organizationDuo", js: "organizationDuo", typ: u(undefined, u(r("PurpleDuo"), null)) },
        { json: "remember", js: "remember", typ: u(undefined, u(r("PurpleRemember"), null)) },
        { json: "webAuthn", js: "webAuthn", typ: u(undefined, u(r("PurpleWebAuthn"), null)) },
        { json: "yubiKey", js: "yubiKey", typ: u(undefined, u(r("PurpleYubiKey"), null)) },
    ], false),
    "PurpleAuthenticator": o([
    ], false),
    "PurpleDuo": o([
        { json: "host", js: "host", typ: "" },
        { json: "signature", js: "signature", typ: "" },
    ], false),
    "PurpleEmail": o([
        { json: "email", js: "email", typ: "" },
    ], false),
    "PurpleRemember": o([
    ], false),
    "PurpleWebAuthn": o([
    ], false),
    "PurpleYubiKey": o([
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
        { json: "twoFactor", js: "twoFactor", typ: u(undefined, u(r("PasswordLoginResponseTwoFactorProviders"), null)) },
    ], false),
    "CAPTCHAResponse": o([
        { json: "siteKey", js: "siteKey", typ: "" },
    ], false),
    "PasswordLoginResponseTwoFactorProviders": o([
        { json: "authenticator", js: "authenticator", typ: u(undefined, u(r("FluffyAuthenticator"), null)) },
        { json: "duo", js: "duo", typ: u(undefined, u(r("FluffyDuo"), null)) },
        { json: "email", js: "email", typ: u(undefined, u(r("FluffyEmail"), null)) },
        { json: "organizationDuo", js: "organizationDuo", typ: u(undefined, u(r("FluffyDuo"), null)) },
        { json: "remember", js: "remember", typ: u(undefined, u(r("FluffyRemember"), null)) },
        { json: "webAuthn", js: "webAuthn", typ: u(undefined, u(r("FluffyWebAuthn"), null)) },
        { json: "yubiKey", js: "yubiKey", typ: u(undefined, u(r("FluffyYubiKey"), null)) },
    ], false),
    "FluffyAuthenticator": o([
    ], false),
    "FluffyDuo": o([
        { json: "host", js: "host", typ: "" },
        { json: "signature", js: "signature", typ: "" },
    ], false),
    "FluffyEmail": o([
        { json: "email", js: "email", typ: "" },
    ], false),
    "FluffyRemember": o([
    ], false),
    "FluffyWebAuthn": o([
    ], false),
    "FluffyYubiKey": o([
        { json: "nfc", js: "nfc", typ: true },
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
        { json: "creationDate", js: "creationDate", typ: "" },
        { json: "id", js: "id", typ: "" },
        { json: "key", js: "key", typ: "" },
        { json: "note", js: "note", typ: "" },
        { json: "object", js: "object", typ: "" },
        { json: "organizationId", js: "organizationId", typ: "" },
        { json: "projectId", js: "projectId", typ: u(undefined, u(null, "")) },
        { json: "revisionDate", js: "revisionDate", typ: "" },
        { json: "value", js: "value", typ: "" },
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
};

