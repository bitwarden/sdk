# This code may look unusually verbose for Ruby (and it is), but
# it performs some subtle and complex validation of JSON data.
#
# To parse this JSON, add 'dry-struct' and 'dry-types' gems, then do:
#
#   client_settings = ClientSettings.from_json! "{…}"
#   puts client_settings.api_url
#
#   command = Command.from_json! "{…}"
#   puts command.projects&.delete&.ids.first
#
#   doc_ref = DocRef.from_json! "{…}"
#   puts doc_ref.totp_response&.code
#
#   response_for_api_key_login_response = ResponseForAPIKeyLoginResponse.from_json! "{…}"
#   puts response_for_api_key_login_response.data&.authenticated
#
#   response_for_fingerprint_response = ResponseForFingerprintResponse.from_json! "{…}"
#   puts response_for_fingerprint_response.data&.fingerprint
#
#   response_for_password_login_response = ResponseForPasswordLoginResponse.from_json! "{…}"
#   puts response_for_password_login_response.data&.authenticated
#
#   response_for_project_response = ResponseForProjectResponse.from_json! "{…}"
#   puts response_for_project_response.data&.creation_date
#
#   response_for_projects_delete_response = ResponseForProjectsDeleteResponse.from_json! "{…}"
#   puts response_for_projects_delete_response.data&.data.first.error.nil?
#
#   response_for_projects_response = ResponseForProjectsResponse.from_json! "{…}"
#   puts response_for_projects_response.data&.data.first.creation_date
#
#   response_for_secret_identifiers_response = ResponseForSecretIdentifiersResponse.from_json! "{…}"
#   puts response_for_secret_identifiers_response.data&.data.first.id
#
#   response_for_secret_response = ResponseForSecretResponse.from_json! "{…}"
#   puts response_for_secret_response.data&.creation_date
#
#   response_for_secrets_delete_response = ResponseForSecretsDeleteResponse.from_json! "{…}"
#   puts response_for_secrets_delete_response.data&.data.first.error.nil?
#
#   response_for_secrets_response = ResponseForSecretsResponse.from_json! "{…}"
#   puts response_for_secrets_response.data&.data.first.creation_date
#
#   response_for_sync_response = ResponseForSyncResponse.from_json! "{…}"
#   puts response_for_sync_response.data&.profile.organizations.first.id
#
#   response_for_user_api_key_response = ResponseForUserAPIKeyResponse.from_json! "{…}"
#   puts response_for_user_api_key_response.data&.api_key
#
# If from_json! succeeds, the value returned matches the schema.

require 'json'
require 'dry-types'
require 'dry-struct'

module Types
  include Dry.Types(default: :nominal)

  Integer            = Strict::Integer
  Nil                = Strict::Nil
  Bool               = Strict::Bool
  Hash               = Strict::Hash
  String             = Strict::String
  DeviceType         = Strict::String.enum("Android", "AndroidAmazon", "ChromeBrowser", "ChromeExtension", "EdgeBrowser", "EdgeExtension", "FirefoxBrowser", "FirefoxExtension", "IEBrowser", "iOS", "LinuxDesktop", "MacOsDesktop", "OperaBrowser", "OperaExtension", "SDK", "SafariBrowser", "SafariExtension", "UWP", "UnknownBrowser", "VivaldiBrowser", "VivaldiExtension", "WindowsDesktop")
  TwoFactorProvider  = Strict::String.enum("Authenticator", "Duo", "Email", "OrganizationDuo", "Remember", "U2f", "WebAuthn", "Yubikey")
  CipherType         = Strict::String.enum("Card", "Identity", "Login", "SecureNote")
  FieldType          = Strict::String.enum("Boolean", "Hidden", "Linked", "Text")
  LinkedIDType       = Strict::String.enum("Address1", "Address2", "Address3", "Brand", "CardholderName", "City", "Code", "Company", "Country", "Email", "ExpMonth", "ExpYear", "FirstName", "FullName", "LastName", "LicenseNumber", "MiddleName", "Number", "PassportNumber", "Password", "Phone", "PostalCode", "Ssn", "State", "Title", "Username")
  URIMatchType       = Strict::String.enum("domain", "exact", "host", "never", "regularExpression", "startsWith")
  CipherRepromptType = Strict::String.enum("None", "Password")
  SecureNoteType     = Strict::String.enum("Generic")
  SendType           = Strict::String.enum("File", "Text")
  ExportFormatEnum   = Strict::String.enum("AccountEncryptedJson", "Csv", "Json")
end

# Device type to send to Bitwarden. Defaults to SDK
module DeviceType
  Android          = "Android"
  AndroidAmazon    = "AndroidAmazon"
  ChromeBrowser    = "ChromeBrowser"
  ChromeExtension  = "ChromeExtension"
  EdgeBrowser      = "EdgeBrowser"
  EdgeExtension    = "EdgeExtension"
  FirefoxBrowser   = "FirefoxBrowser"
  FirefoxExtension = "FirefoxExtension"
  IEBrowser        = "IEBrowser"
  IOS              = "iOS"
  LinuxDesktop     = "LinuxDesktop"
  MACOSDesktop     = "MacOsDesktop"
  OperaBrowser     = "OperaBrowser"
  OperaExtension   = "OperaExtension"
  SDK              = "SDK"
  SafariBrowser    = "SafariBrowser"
  SafariExtension  = "SafariExtension"
  UWP              = "UWP"
  UnknownBrowser   = "UnknownBrowser"
  VivaldiBrowser   = "VivaldiBrowser"
  VivaldiExtension = "VivaldiExtension"
  WindowsDesktop   = "WindowsDesktop"
end

# Basic client behavior settings. These settings specify the various targets and behavior
# of the Bitwarden Client. They are optional and uneditable once the client is
# initialized.
#
# Defaults to
#
# ``` # use bitwarden_sdk::client::client_settings::{ClientSettings, DeviceType}; # use
# assert_matches::assert_matches; let settings = ClientSettings { identity_url:
# "https://identity.bitwarden.com".to_string(), api_url:
# "https://api.bitwarden.com".to_string(), user_agent: "Bitwarden Rust-SDK".to_string(),
# device_type: DeviceType::SDK, }; let default = ClientSettings::default();
# assert_matches!(settings, default); ```
#
# Targets `localhost:8080` for debug builds.
class ClientSettings < Dry::Struct

  # The api url of the targeted Bitwarden instance. Defaults to `https://api.bitwarden.com`
  attribute :api_url, Types::String.optional

  # Device type to send to Bitwarden. Defaults to SDK
  attribute :device_type, Types::DeviceType.optional

  # The identity url of the targeted Bitwarden instance. Defaults to
  # `https://identity.bitwarden.com`
  attribute :identity_url, Types::String.optional

  # The user_agent to sent to Bitwarden. Defaults to `Bitwarden Rust-SDK`
  attribute :user_agent, Types::String.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      api_url:      d["apiUrl"],
      device_type:  d["deviceType"],
      identity_url: d["identityUrl"],
      user_agent:   d["userAgent"],
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "apiUrl"      => api_url,
      "deviceType"  => device_type,
      "identityUrl" => identity_url,
      "userAgent"   => user_agent,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

# Login to Bitwarden with access token
class AccessTokenLoginRequest < Dry::Struct

  # Bitwarden service API access token
  attribute :access_token, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      access_token: d.fetch("accessToken"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "accessToken" => access_token,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

# Login to Bitwarden with Api Key
class APIKeyLoginRequest < Dry::Struct

  # Bitwarden account client_id
  attribute :client_id, Types::String

  # Bitwarden account client_secret
  attribute :client_secret, Types::String

  # Bitwarden account master password
  attribute :password, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      client_id:     d.fetch("clientId"),
      client_secret: d.fetch("clientSecret"),
      password:      d.fetch("password"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "clientId"     => client_id,
      "clientSecret" => client_secret,
      "password"     => password,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class FingerprintRequest < Dry::Struct

  # The input material, used in the fingerprint generation process.
  attribute :fingerprint_material, Types::String

  # The user's public key encoded with base64.
  attribute :public_key, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      fingerprint_material: d.fetch("fingerprintMaterial"),
      public_key:           d.fetch("publicKey"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "fingerprintMaterial" => fingerprint_material,
      "publicKey"           => public_key,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class SecretVerificationRequest < Dry::Struct

  # The user's master password to use for user verification. If supplied, this will be used
  # for verification purposes.
  attribute :master_password, Types::String.optional.optional

  # Alternate user verification method through OTP. This is provided for users who have no
  # master password due to use of Customer Managed Encryption. Must be present and valid if
  # master_password is absent.
  attribute :otp, Types::String.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      master_password: d["masterPassword"],
      otp:             d["otp"],
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "masterPassword" => master_password,
      "otp"            => otp,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class Argon2ID1 < Dry::Struct
  attribute :iterations,  Types::Integer
  attribute :memory,      Types::Integer
  attribute :parallelism, Types::Integer

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      iterations:  d.fetch("iterations"),
      memory:      d.fetch("memory"),
      parallelism: d.fetch("parallelism"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "iterations"  => iterations,
      "memory"      => memory,
      "parallelism" => parallelism,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class PBKDF21 < Dry::Struct
  attribute :iterations, Types::Integer

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      iterations: d.fetch("iterations"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "iterations" => iterations,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

# Kdf from prelogin
class PasswordLoginKdf < Dry::Struct
  attribute :p_bkdf2,   PBKDF21.optional
  attribute :argon2_id, Argon2ID1.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      p_bkdf2:   d["pBKDF2"] ? PBKDF21.from_dynamic!(d["pBKDF2"]) : nil,
      argon2_id: d["argon2id"] ? Argon2ID1.from_dynamic!(d["argon2id"]) : nil,
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "pBKDF2"   => p_bkdf2&.to_dynamic,
      "argon2id" => argon2_id&.to_dynamic,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

# Two-factor provider
module TwoFactorProvider
  Authenticator   = "Authenticator"
  Duo             = "Duo"
  Email           = "Email"
  OrganizationDuo = "OrganizationDuo"
  Remember        = "Remember"
  U2F             = "U2f"
  WebAuthn        = "WebAuthn"
  Yubikey         = "Yubikey"
end

class TwoFactorRequest < Dry::Struct

  # Two-factor provider
  attribute :provider, Types::TwoFactorProvider

  # Two-factor remember
  attribute :remember, Types::Bool

  # Two-factor Token
  attribute :token, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      provider: d.fetch("provider"),
      remember: d.fetch("remember"),
      token:    d.fetch("token"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "provider" => provider,
      "remember" => remember,
      "token"    => token,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

# Login to Bitwarden with Username and Password
class PasswordLoginRequest < Dry::Struct

  # Bitwarden account email address
  attribute :email, Types::String

  # Kdf from prelogin
  attribute :kdf, PasswordLoginKdf

  # Bitwarden account master password
  attribute :password, Types::String

  attribute :two_factor, TwoFactorRequest.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      email:      d.fetch("email"),
      kdf:        PasswordLoginKdf.from_dynamic!(d.fetch("kdf")),
      password:   d.fetch("password"),
      two_factor: d["twoFactor"] ? TwoFactorRequest.from_dynamic!(d["twoFactor"]) : nil,
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "email"     => email,
      "kdf"       => kdf.to_dynamic,
      "password"  => password,
      "twoFactor" => two_factor&.to_dynamic,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class ProjectCreateRequest < Dry::Struct
  attribute :project_create_request_name, Types::String

  # Organization where the project will be created
  attribute :organization_id, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      project_create_request_name: d.fetch("name"),
      organization_id:             d.fetch("organizationId"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "name"           => project_create_request_name,
      "organizationId" => organization_id,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class ProjectsDeleteRequest < Dry::Struct

  # IDs of the projects to delete
  attribute :ids, Types.Array(Types::String)

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      ids: d.fetch("ids"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "ids" => ids,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class ProjectGetRequest < Dry::Struct

  # ID of the project to retrieve
  attribute :id, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      id: d.fetch("id"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "id" => id,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class ProjectsListRequest < Dry::Struct

  # Organization to retrieve all the projects from
  attribute :organization_id, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      organization_id: d.fetch("organizationId"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "organizationId" => organization_id,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class ProjectPutRequest < Dry::Struct

  # ID of the project to modify
  attribute :id, Types::String

  attribute :project_put_request_name, Types::String

  # Organization ID of the project to modify
  attribute :organization_id, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      id:                       d.fetch("id"),
      project_put_request_name: d.fetch("name"),
      organization_id:          d.fetch("organizationId"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "id"             => id,
      "name"           => project_put_request_name,
      "organizationId" => organization_id,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

# > Requires Authentication > Requires using an Access Token for login or calling Sync at
# least once Retrieve a project by the provided identifier
#
# Returns: [ProjectResponse](bitwarden_sdk::secrets_manager::projects::ProjectResponse)
#
# > Requires Authentication > Requires using an Access Token for login or calling Sync at
# least once Creates a new project in the provided organization using the given data
#
# Returns: [ProjectResponse](bitwarden_sdk::secrets_manager::projects::ProjectResponse)
#
# > Requires Authentication > Requires using an Access Token for login or calling Sync at
# least once Lists all projects of the given organization
#
# Returns: [ProjectsResponse](bitwarden_sdk::secrets_manager::projects::ProjectsResponse)
#
# > Requires Authentication > Requires using an Access Token for login or calling Sync at
# least once Updates an existing project with the provided ID using the given data
#
# Returns: [ProjectResponse](bitwarden_sdk::secrets_manager::projects::ProjectResponse)
#
# > Requires Authentication > Requires using an Access Token for login or calling Sync at
# least once Deletes all the projects whose IDs match the provided ones
#
# Returns:
# [ProjectsDeleteResponse](bitwarden_sdk::secrets_manager::projects::ProjectsDeleteResponse)
class ProjectsCommand < Dry::Struct
  attribute :get,    ProjectGetRequest.optional
  attribute :create, ProjectCreateRequest.optional
  attribute :list,   ProjectsListRequest.optional
  attribute :update, ProjectPutRequest.optional
  attribute :delete, ProjectsDeleteRequest.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      get:    d["get"] ? ProjectGetRequest.from_dynamic!(d["get"]) : nil,
      create: d["create"] ? ProjectCreateRequest.from_dynamic!(d["create"]) : nil,
      list:   d["list"] ? ProjectsListRequest.from_dynamic!(d["list"]) : nil,
      update: d["update"] ? ProjectPutRequest.from_dynamic!(d["update"]) : nil,
      delete: d["delete"] ? ProjectsDeleteRequest.from_dynamic!(d["delete"]) : nil,
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "get"    => get&.to_dynamic,
      "create" => create&.to_dynamic,
      "list"   => list&.to_dynamic,
      "update" => update&.to_dynamic,
      "delete" => delete&.to_dynamic,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class SecretCreateRequest < Dry::Struct
  attribute :key,  Types::String
  attribute :note, Types::String

  # Organization where the secret will be created
  attribute :organization_id, Types::String

  # IDs of the projects that this secret will belong to
  attribute :project_ids, Types.Array(Types::String).optional.optional

  attribute :value, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      key:             d.fetch("key"),
      note:            d.fetch("note"),
      organization_id: d.fetch("organizationId"),
      project_ids:     d["projectIds"],
      value:           d.fetch("value"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "key"            => key,
      "note"           => note,
      "organizationId" => organization_id,
      "projectIds"     => project_ids,
      "value"          => value,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class SecretsDeleteRequest < Dry::Struct

  # IDs of the secrets to delete
  attribute :ids, Types.Array(Types::String)

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      ids: d.fetch("ids"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "ids" => ids,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class SecretGetRequest < Dry::Struct

  # ID of the secret to retrieve
  attribute :id, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      id: d.fetch("id"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "id" => id,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class SecretsGetRequest < Dry::Struct

  # IDs of the secrets to retrieve
  attribute :ids, Types.Array(Types::String)

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      ids: d.fetch("ids"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "ids" => ids,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class SecretIdentifiersRequest < Dry::Struct

  # Organization to retrieve all the secrets from
  attribute :organization_id, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      organization_id: d.fetch("organizationId"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "organizationId" => organization_id,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class SecretPutRequest < Dry::Struct

  # ID of the secret to modify
  attribute :id, Types::String

  attribute :key,  Types::String
  attribute :note, Types::String

  # Organization ID of the secret to modify
  attribute :organization_id, Types::String

  attribute :project_ids, Types.Array(Types::String).optional.optional
  attribute :value,       Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      id:              d.fetch("id"),
      key:             d.fetch("key"),
      note:            d.fetch("note"),
      organization_id: d.fetch("organizationId"),
      project_ids:     d["projectIds"],
      value:           d.fetch("value"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "id"             => id,
      "key"            => key,
      "note"           => note,
      "organizationId" => organization_id,
      "projectIds"     => project_ids,
      "value"          => value,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

# > Requires Authentication > Requires using an Access Token for login or calling Sync at
# least once Retrieve a secret by the provided identifier
#
# Returns: [SecretResponse](bitwarden_sdk::secrets_manager::secrets::SecretResponse)
#
# > Requires Authentication > Requires using an Access Token for login or calling Sync at
# least once Retrieve secrets by the provided identifiers
#
# Returns: [SecretsResponse](bitwarden_sdk::secrets_manager::secrets::SecretsResponse)
#
# > Requires Authentication > Requires using an Access Token for login or calling Sync at
# least once Creates a new secret in the provided organization using the given data
#
# Returns: [SecretResponse](bitwarden_sdk::secrets_manager::secrets::SecretResponse)
#
# > Requires Authentication > Requires using an Access Token for login or calling Sync at
# least once Lists all secret identifiers of the given organization, to then retrieve each
# secret, use `CreateSecret`
#
# Returns:
# [SecretIdentifiersResponse](bitwarden_sdk::secrets_manager::secrets::SecretIdentifiersResponse)
#
# > Requires Authentication > Requires using an Access Token for login or calling Sync at
# least once Updates an existing secret with the provided ID using the given data
#
# Returns: [SecretResponse](bitwarden_sdk::secrets_manager::secrets::SecretResponse)
#
# > Requires Authentication > Requires using an Access Token for login or calling Sync at
# least once Deletes all the secrets whose IDs match the provided ones
#
# Returns:
# [SecretsDeleteResponse](bitwarden_sdk::secrets_manager::secrets::SecretsDeleteResponse)
class SecretsCommand < Dry::Struct
  attribute :get,        SecretGetRequest.optional
  attribute :get_by_ids, SecretsGetRequest.optional
  attribute :create,     SecretCreateRequest.optional
  attribute :list,       SecretIdentifiersRequest.optional
  attribute :update,     SecretPutRequest.optional
  attribute :delete,     SecretsDeleteRequest.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      get:        d["get"] ? SecretGetRequest.from_dynamic!(d["get"]) : nil,
      get_by_ids: d["getByIds"] ? SecretsGetRequest.from_dynamic!(d["getByIds"]) : nil,
      create:     d["create"] ? SecretCreateRequest.from_dynamic!(d["create"]) : nil,
      list:       d["list"] ? SecretIdentifiersRequest.from_dynamic!(d["list"]) : nil,
      update:     d["update"] ? SecretPutRequest.from_dynamic!(d["update"]) : nil,
      delete:     d["delete"] ? SecretsDeleteRequest.from_dynamic!(d["delete"]) : nil,
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "get"      => get&.to_dynamic,
      "getByIds" => get_by_ids&.to_dynamic,
      "create"   => create&.to_dynamic,
      "list"     => list&.to_dynamic,
      "update"   => update&.to_dynamic,
      "delete"   => delete&.to_dynamic,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class SyncRequest < Dry::Struct

  # Exclude the subdomains from the response, defaults to false
  attribute :exclude_subdomains, Types::Bool.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      exclude_subdomains: d["excludeSubdomains"],
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "excludeSubdomains" => exclude_subdomains,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

# Login with username and password
#
# This command is for initiating an authentication handshake with Bitwarden. Authorization
# may fail due to requiring 2fa or captcha challenge completion despite accurate
# credentials.
#
# This command is not capable of handling authentication requiring 2fa or captcha.
#
# Returns: [PasswordLoginResponse](bitwarden_sdk::auth::login::PasswordLoginResponse)
#
# Login with API Key
#
# This command is for initiating an authentication handshake with Bitwarden.
#
# Returns: [ApiKeyLoginResponse](bitwarden_sdk::auth::login::ApiKeyLoginResponse)
#
# Login with Secrets Manager Access Token
#
# This command is for initiating an authentication handshake with Bitwarden.
#
# Returns: [ApiKeyLoginResponse](bitwarden_sdk::auth::login::ApiKeyLoginResponse)
#
# > Requires Authentication Get the API key of the currently authenticated user
#
# Returns: [UserApiKeyResponse](bitwarden_sdk::platform::UserApiKeyResponse)
#
# Get the user's passphrase
#
# Returns: String
#
# > Requires Authentication Retrieve all user data, ciphers and organizations the user is a
# part of
#
# Returns: [SyncResponse](bitwarden_sdk::platform::SyncResponse)
class Command < Dry::Struct
  attribute :password_login,     PasswordLoginRequest.optional
  attribute :api_key_login,      APIKeyLoginRequest.optional
  attribute :access_token_login, AccessTokenLoginRequest.optional
  attribute :get_user_api_key,   SecretVerificationRequest.optional
  attribute :fingerprint,        FingerprintRequest.optional
  attribute :sync,               SyncRequest.optional
  attribute :secrets,            SecretsCommand.optional
  attribute :projects,           ProjectsCommand.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      password_login:     d["passwordLogin"] ? PasswordLoginRequest.from_dynamic!(d["passwordLogin"]) : nil,
      api_key_login:      d["apiKeyLogin"] ? APIKeyLoginRequest.from_dynamic!(d["apiKeyLogin"]) : nil,
      access_token_login: d["accessTokenLogin"] ? AccessTokenLoginRequest.from_dynamic!(d["accessTokenLogin"]) : nil,
      get_user_api_key:   d["getUserApiKey"] ? SecretVerificationRequest.from_dynamic!(d["getUserApiKey"]) : nil,
      fingerprint:        d["fingerprint"] ? FingerprintRequest.from_dynamic!(d["fingerprint"]) : nil,
      sync:               d["sync"] ? SyncRequest.from_dynamic!(d["sync"]) : nil,
      secrets:            d["secrets"] ? SecretsCommand.from_dynamic!(d["secrets"]) : nil,
      projects:           d["projects"] ? ProjectsCommand.from_dynamic!(d["projects"]) : nil,
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "passwordLogin"    => password_login&.to_dynamic,
      "apiKeyLogin"      => api_key_login&.to_dynamic,
      "accessTokenLogin" => access_token_login&.to_dynamic,
      "getUserApiKey"    => get_user_api_key&.to_dynamic,
      "fingerprint"      => fingerprint&.to_dynamic,
      "sync"             => sync&.to_dynamic,
      "secrets"          => secrets&.to_dynamic,
      "projects"         => projects&.to_dynamic,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class Attachment < Dry::Struct
  attribute :file_name, Types::String.optional.optional
  attribute :id,        Types::String.optional.optional
  attribute :key,       Types::String.optional.optional
  attribute :size,      Types::String.optional.optional

  # Readable size, ex: "4.2 KB" or "1.43 GB"
  attribute :size_name, Types::String.optional.optional

  attribute :url, Types::String.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      file_name: d["fileName"],
      id:        d["id"],
      key:       d["key"],
      size:      d["size"],
      size_name: d["sizeName"],
      url:       d["url"],
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "fileName" => file_name,
      "id"       => id,
      "key"      => key,
      "size"     => size,
      "sizeName" => size_name,
      "url"      => url,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class Card < Dry::Struct
  attribute :brand,           Types::String.optional.optional
  attribute :cardholder_name, Types::String.optional.optional
  attribute :code,            Types::String.optional.optional
  attribute :exp_month,       Types::String.optional.optional
  attribute :exp_year,        Types::String.optional.optional
  attribute :number,          Types::String.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      brand:           d["brand"],
      cardholder_name: d["cardholderName"],
      code:            d["code"],
      exp_month:       d["expMonth"],
      exp_year:        d["expYear"],
      number:          d["number"],
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "brand"          => brand,
      "cardholderName" => cardholder_name,
      "code"           => code,
      "expMonth"       => exp_month,
      "expYear"        => exp_year,
      "number"         => number,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

module CipherType
  Card       = "Card"
  Identity   = "Identity"
  Login      = "Login"
  SecureNote = "SecureNote"
end

module FieldType
  Boolean = "Boolean"
  Hidden  = "Hidden"
  Linked  = "Linked"
  Text    = "Text"
end

module LinkedIDType
  Address1       = "Address1"
  Address2       = "Address2"
  Address3       = "Address3"
  Brand          = "Brand"
  CardholderName = "CardholderName"
  City           = "City"
  Code           = "Code"
  Company        = "Company"
  Country        = "Country"
  Email          = "Email"
  ExpMonth       = "ExpMonth"
  ExpYear        = "ExpYear"
  FirstName      = "FirstName"
  FullName       = "FullName"
  LastName       = "LastName"
  LicenseNumber  = "LicenseNumber"
  MiddleName     = "MiddleName"
  Number         = "Number"
  PassportNumber = "PassportNumber"
  Password       = "Password"
  Phone          = "Phone"
  PostalCode     = "PostalCode"
  Ssn            = "Ssn"
  State          = "State"
  Title          = "Title"
  Username       = "Username"
end

class Field < Dry::Struct
  attribute :linked_id,  Types::LinkedIDType.optional.optional
  attribute :field_name, Types::String.optional.optional
  attribute :field_type, Types::FieldType
  attribute :value,      Types::String.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      linked_id:  d["linkedId"],
      field_name: d["name"],
      field_type: d.fetch("type"),
      value:      d["value"],
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "linkedId" => linked_id,
      "name"     => field_name,
      "type"     => field_type,
      "value"    => value,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class Identity < Dry::Struct
  attribute :address1,        Types::String.optional.optional
  attribute :address2,        Types::String.optional.optional
  attribute :address3,        Types::String.optional.optional
  attribute :city,            Types::String.optional.optional
  attribute :company,         Types::String.optional.optional
  attribute :country,         Types::String.optional.optional
  attribute :email,           Types::String.optional.optional
  attribute :first_name,      Types::String.optional.optional
  attribute :last_name,       Types::String.optional.optional
  attribute :license_number,  Types::String.optional.optional
  attribute :middle_name,     Types::String.optional.optional
  attribute :passport_number, Types::String.optional.optional
  attribute :phone,           Types::String.optional.optional
  attribute :postal_code,     Types::String.optional.optional
  attribute :ssn,             Types::String.optional.optional
  attribute :state,           Types::String.optional.optional
  attribute :title,           Types::String.optional.optional
  attribute :username,        Types::String.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      address1:        d["address1"],
      address2:        d["address2"],
      address3:        d["address3"],
      city:            d["city"],
      company:         d["company"],
      country:         d["country"],
      email:           d["email"],
      first_name:      d["firstName"],
      last_name:       d["lastName"],
      license_number:  d["licenseNumber"],
      middle_name:     d["middleName"],
      passport_number: d["passportNumber"],
      phone:           d["phone"],
      postal_code:     d["postalCode"],
      ssn:             d["ssn"],
      state:           d["state"],
      title:           d["title"],
      username:        d["username"],
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "address1"       => address1,
      "address2"       => address2,
      "address3"       => address3,
      "city"           => city,
      "company"        => company,
      "country"        => country,
      "email"          => email,
      "firstName"      => first_name,
      "lastName"       => last_name,
      "licenseNumber"  => license_number,
      "middleName"     => middle_name,
      "passportNumber" => passport_number,
      "phone"          => phone,
      "postalCode"     => postal_code,
      "ssn"            => ssn,
      "state"          => state,
      "title"          => title,
      "username"       => username,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class LocalData < Dry::Struct
  attribute :last_launched,  Types::Integer.optional.optional
  attribute :last_used_date, Types::Integer.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      last_launched:  d["lastLaunched"],
      last_used_date: d["lastUsedDate"],
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "lastLaunched" => last_launched,
      "lastUsedDate" => last_used_date,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

module URIMatchType
  Domain            = "domain"
  Exact             = "exact"
  Host              = "host"
  Never             = "never"
  RegularExpression = "regularExpression"
  StartsWith        = "startsWith"
end

class LoginURI < Dry::Struct
  attribute :match, Types::URIMatchType.optional.optional
  attribute :uri,   Types::String.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      match: d["match"],
      uri:   d["uri"],
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "match" => match,
      "uri"   => uri,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class Login < Dry::Struct
  attribute :autofill_on_page_load,  Types::Bool.optional.optional
  attribute :password,               Types::String.optional.optional
  attribute :password_revision_date, Types::String.optional.optional
  attribute :totp,                   Types::String.optional.optional
  attribute :uris,                   Types.Array(LoginURI).optional.optional
  attribute :username,               Types::String.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      autofill_on_page_load:  d["autofillOnPageLoad"],
      password:               d["password"],
      password_revision_date: d["passwordRevisionDate"],
      totp:                   d["totp"],
      uris:                   d["uris"]&.map { |x| LoginURI.from_dynamic!(x) },
      username:               d["username"],
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "autofillOnPageLoad"   => autofill_on_page_load,
      "password"             => password,
      "passwordRevisionDate" => password_revision_date,
      "totp"                 => totp,
      "uris"                 => uris&.map { |x| x.to_dynamic },
      "username"             => username,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class PasswordHistory < Dry::Struct
  attribute :last_used_date, Types::String
  attribute :password,       Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      last_used_date: d.fetch("lastUsedDate"),
      password:       d.fetch("password"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "lastUsedDate" => last_used_date,
      "password"     => password,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

module CipherRepromptType
  None     = "None"
  Password = "Password"
end

module SecureNoteType
  Generic = "Generic"
end

class SecureNote < Dry::Struct
  attribute :secure_note_type, Types::SecureNoteType

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      secure_note_type: d.fetch("type"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "type" => secure_note_type,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class Cipher < Dry::Struct
  attribute :attachments,           Types.Array(Attachment).optional.optional
  attribute :card,                  Card.optional.optional
  attribute :collection_ids,        Types.Array(Types::String)
  attribute :creation_date,         Types::String
  attribute :deleted_date,          Types::String.optional.optional
  attribute :edit,                  Types::Bool
  attribute :favorite,              Types::Bool
  attribute :fields,                Types.Array(Field).optional.optional
  attribute :folder_id,             Types::String.optional.optional
  attribute :id,                    Types::String.optional.optional
  attribute :identity,              Identity.optional.optional
  attribute :local_data,            LocalData.optional.optional
  attribute :login,                 Login.optional.optional
  attribute :cipher_name,           Types::String
  attribute :notes,                 Types::String.optional.optional
  attribute :organization_id,       Types::String.optional.optional
  attribute :organization_use_totp, Types::Bool
  attribute :password_history,      Types.Array(PasswordHistory).optional.optional
  attribute :reprompt,              Types::CipherRepromptType
  attribute :revision_date,         Types::String
  attribute :secure_note,           SecureNote.optional.optional
  attribute :cipher_type,           Types::CipherType
  attribute :view_password,         Types::Bool

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      attachments:           d["attachments"]&.map { |x| Attachment.from_dynamic!(x) },
      card:                  d["card"] ? Card.from_dynamic!(d["card"]) : nil,
      collection_ids:        d.fetch("collectionIds"),
      creation_date:         d.fetch("creationDate"),
      deleted_date:          d["deletedDate"],
      edit:                  d.fetch("edit"),
      favorite:              d.fetch("favorite"),
      fields:                d["fields"]&.map { |x| Field.from_dynamic!(x) },
      folder_id:             d["folderId"],
      id:                    d["id"],
      identity:              d["identity"] ? Identity.from_dynamic!(d["identity"]) : nil,
      local_data:            d["localData"] ? LocalData.from_dynamic!(d["localData"]) : nil,
      login:                 d["login"] ? Login.from_dynamic!(d["login"]) : nil,
      cipher_name:           d.fetch("name"),
      notes:                 d["notes"],
      organization_id:       d["organizationId"],
      organization_use_totp: d.fetch("organizationUseTotp"),
      password_history:      d["passwordHistory"]&.map { |x| PasswordHistory.from_dynamic!(x) },
      reprompt:              d.fetch("reprompt"),
      revision_date:         d.fetch("revisionDate"),
      secure_note:           d["secureNote"] ? SecureNote.from_dynamic!(d["secureNote"]) : nil,
      cipher_type:           d.fetch("type"),
      view_password:         d.fetch("viewPassword"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "attachments"         => attachments&.map { |x| x.to_dynamic },
      "card"                => card&.to_dynamic,
      "collectionIds"       => collection_ids,
      "creationDate"        => creation_date,
      "deletedDate"         => deleted_date,
      "edit"                => edit,
      "favorite"            => favorite,
      "fields"              => fields&.map { |x| x.to_dynamic },
      "folderId"            => folder_id,
      "id"                  => id,
      "identity"            => identity&.to_dynamic,
      "localData"           => local_data&.to_dynamic,
      "login"               => login&.to_dynamic,
      "name"                => cipher_name,
      "notes"               => notes,
      "organizationId"      => organization_id,
      "organizationUseTotp" => organization_use_totp,
      "passwordHistory"     => password_history&.map { |x| x.to_dynamic },
      "reprompt"            => reprompt,
      "revisionDate"        => revision_date,
      "secureNote"          => secure_note&.to_dynamic,
      "type"                => cipher_type,
      "viewPassword"        => view_password,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class AttachmentView < Dry::Struct
  attribute :file_name, Types::String.optional.optional
  attribute :id,        Types::String.optional.optional
  attribute :key,       Types::String.optional.optional
  attribute :size,      Types::String.optional.optional
  attribute :size_name, Types::String.optional.optional
  attribute :url,       Types::String.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      file_name: d["fileName"],
      id:        d["id"],
      key:       d["key"],
      size:      d["size"],
      size_name: d["sizeName"],
      url:       d["url"],
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "fileName" => file_name,
      "id"       => id,
      "key"      => key,
      "size"     => size,
      "sizeName" => size_name,
      "url"      => url,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class CardView < Dry::Struct
  attribute :brand,           Types::String.optional.optional
  attribute :cardholder_name, Types::String.optional.optional
  attribute :code,            Types::String.optional.optional
  attribute :exp_month,       Types::String.optional.optional
  attribute :exp_year,        Types::String.optional.optional
  attribute :number,          Types::String.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      brand:           d["brand"],
      cardholder_name: d["cardholderName"],
      code:            d["code"],
      exp_month:       d["expMonth"],
      exp_year:        d["expYear"],
      number:          d["number"],
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "brand"          => brand,
      "cardholderName" => cardholder_name,
      "code"           => code,
      "expMonth"       => exp_month,
      "expYear"        => exp_year,
      "number"         => number,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class FieldView < Dry::Struct
  attribute :linked_id,       Types::LinkedIDType.optional.optional
  attribute :field_view_name, Types::String.optional.optional
  attribute :field_view_type, Types::FieldType
  attribute :value,           Types::String.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      linked_id:       d["linkedId"],
      field_view_name: d["name"],
      field_view_type: d.fetch("type"),
      value:           d["value"],
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "linkedId" => linked_id,
      "name"     => field_view_name,
      "type"     => field_view_type,
      "value"    => value,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class IdentityView < Dry::Struct
  attribute :address1,        Types::String.optional.optional
  attribute :address2,        Types::String.optional.optional
  attribute :address3,        Types::String.optional.optional
  attribute :city,            Types::String.optional.optional
  attribute :company,         Types::String.optional.optional
  attribute :country,         Types::String.optional.optional
  attribute :email,           Types::String.optional.optional
  attribute :first_name,      Types::String.optional.optional
  attribute :last_name,       Types::String.optional.optional
  attribute :license_number,  Types::String.optional.optional
  attribute :middle_name,     Types::String.optional.optional
  attribute :passport_number, Types::String.optional.optional
  attribute :phone,           Types::String.optional.optional
  attribute :postal_code,     Types::String.optional.optional
  attribute :ssn,             Types::String.optional.optional
  attribute :state,           Types::String.optional.optional
  attribute :title,           Types::String.optional.optional
  attribute :username,        Types::String.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      address1:        d["address1"],
      address2:        d["address2"],
      address3:        d["address3"],
      city:            d["city"],
      company:         d["company"],
      country:         d["country"],
      email:           d["email"],
      first_name:      d["firstName"],
      last_name:       d["lastName"],
      license_number:  d["licenseNumber"],
      middle_name:     d["middleName"],
      passport_number: d["passportNumber"],
      phone:           d["phone"],
      postal_code:     d["postalCode"],
      ssn:             d["ssn"],
      state:           d["state"],
      title:           d["title"],
      username:        d["username"],
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "address1"       => address1,
      "address2"       => address2,
      "address3"       => address3,
      "city"           => city,
      "company"        => company,
      "country"        => country,
      "email"          => email,
      "firstName"      => first_name,
      "lastName"       => last_name,
      "licenseNumber"  => license_number,
      "middleName"     => middle_name,
      "passportNumber" => passport_number,
      "phone"          => phone,
      "postalCode"     => postal_code,
      "ssn"            => ssn,
      "state"          => state,
      "title"          => title,
      "username"       => username,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class LocalDataView < Dry::Struct
  attribute :last_launched,  Types::Integer.optional.optional
  attribute :last_used_date, Types::Integer.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      last_launched:  d["lastLaunched"],
      last_used_date: d["lastUsedDate"],
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "lastLaunched" => last_launched,
      "lastUsedDate" => last_used_date,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class LoginURIView < Dry::Struct
  attribute :match, Types::URIMatchType.optional.optional
  attribute :uri,   Types::String.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      match: d["match"],
      uri:   d["uri"],
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "match" => match,
      "uri"   => uri,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class LoginView < Dry::Struct
  attribute :autofill_on_page_load,  Types::Bool.optional.optional
  attribute :password,               Types::String.optional.optional
  attribute :password_revision_date, Types::String.optional.optional
  attribute :totp,                   Types::String.optional.optional
  attribute :uris,                   Types.Array(LoginURIView).optional.optional
  attribute :username,               Types::String.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      autofill_on_page_load:  d["autofillOnPageLoad"],
      password:               d["password"],
      password_revision_date: d["passwordRevisionDate"],
      totp:                   d["totp"],
      uris:                   d["uris"]&.map { |x| LoginURIView.from_dynamic!(x) },
      username:               d["username"],
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "autofillOnPageLoad"   => autofill_on_page_load,
      "password"             => password,
      "passwordRevisionDate" => password_revision_date,
      "totp"                 => totp,
      "uris"                 => uris&.map { |x| x.to_dynamic },
      "username"             => username,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class PasswordHistoryView < Dry::Struct
  attribute :last_used_date, Types::String
  attribute :password,       Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      last_used_date: d.fetch("lastUsedDate"),
      password:       d.fetch("password"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "lastUsedDate" => last_used_date,
      "password"     => password,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class SecureNoteView < Dry::Struct
  attribute :secure_note_view_type, Types::SecureNoteType

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      secure_note_view_type: d.fetch("type"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "type" => secure_note_view_type,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class CipherView < Dry::Struct
  attribute :attachments,           Types.Array(AttachmentView).optional.optional
  attribute :card,                  CardView.optional.optional
  attribute :collection_ids,        Types.Array(Types::String)
  attribute :creation_date,         Types::String
  attribute :deleted_date,          Types::String.optional.optional
  attribute :edit,                  Types::Bool
  attribute :favorite,              Types::Bool
  attribute :fields,                Types.Array(FieldView).optional.optional
  attribute :folder_id,             Types::String.optional.optional
  attribute :id,                    Types::String.optional.optional
  attribute :identity,              IdentityView.optional.optional
  attribute :local_data,            LocalDataView.optional.optional
  attribute :login,                 LoginView.optional.optional
  attribute :cipher_view_name,      Types::String
  attribute :notes,                 Types::String.optional.optional
  attribute :organization_id,       Types::String.optional.optional
  attribute :organization_use_totp, Types::Bool
  attribute :password_history,      Types.Array(PasswordHistoryView).optional.optional
  attribute :reprompt,              Types::CipherRepromptType
  attribute :revision_date,         Types::String
  attribute :secure_note,           SecureNoteView.optional.optional
  attribute :cipher_view_type,      Types::CipherType
  attribute :view_password,         Types::Bool

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      attachments:           d["attachments"]&.map { |x| AttachmentView.from_dynamic!(x) },
      card:                  d["card"] ? CardView.from_dynamic!(d["card"]) : nil,
      collection_ids:        d.fetch("collectionIds"),
      creation_date:         d.fetch("creationDate"),
      deleted_date:          d["deletedDate"],
      edit:                  d.fetch("edit"),
      favorite:              d.fetch("favorite"),
      fields:                d["fields"]&.map { |x| FieldView.from_dynamic!(x) },
      folder_id:             d["folderId"],
      id:                    d["id"],
      identity:              d["identity"] ? IdentityView.from_dynamic!(d["identity"]) : nil,
      local_data:            d["localData"] ? LocalDataView.from_dynamic!(d["localData"]) : nil,
      login:                 d["login"] ? LoginView.from_dynamic!(d["login"]) : nil,
      cipher_view_name:      d.fetch("name"),
      notes:                 d["notes"],
      organization_id:       d["organizationId"],
      organization_use_totp: d.fetch("organizationUseTotp"),
      password_history:      d["passwordHistory"]&.map { |x| PasswordHistoryView.from_dynamic!(x) },
      reprompt:              d.fetch("reprompt"),
      revision_date:         d.fetch("revisionDate"),
      secure_note:           d["secureNote"] ? SecureNoteView.from_dynamic!(d["secureNote"]) : nil,
      cipher_view_type:      d.fetch("type"),
      view_password:         d.fetch("viewPassword"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "attachments"         => attachments&.map { |x| x.to_dynamic },
      "card"                => card&.to_dynamic,
      "collectionIds"       => collection_ids,
      "creationDate"        => creation_date,
      "deletedDate"         => deleted_date,
      "edit"                => edit,
      "favorite"            => favorite,
      "fields"              => fields&.map { |x| x.to_dynamic },
      "folderId"            => folder_id,
      "id"                  => id,
      "identity"            => identity&.to_dynamic,
      "localData"           => local_data&.to_dynamic,
      "login"               => login&.to_dynamic,
      "name"                => cipher_view_name,
      "notes"               => notes,
      "organizationId"      => organization_id,
      "organizationUseTotp" => organization_use_totp,
      "passwordHistory"     => password_history&.map { |x| x.to_dynamic },
      "reprompt"            => reprompt,
      "revisionDate"        => revision_date,
      "secureNote"          => secure_note&.to_dynamic,
      "type"                => cipher_view_type,
      "viewPassword"        => view_password,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class Collection < Dry::Struct
  attribute :external_id,     Types::String.optional.optional
  attribute :hide_passwords,  Types::Bool
  attribute :id,              Types::String
  attribute :collection_name, Types::String
  attribute :organization_id, Types::String
  attribute :read_only,       Types::Bool

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      external_id:     d["externalId"],
      hide_passwords:  d.fetch("hidePasswords"),
      id:              d.fetch("id"),
      collection_name: d.fetch("name"),
      organization_id: d.fetch("organizationId"),
      read_only:       d.fetch("readOnly"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "externalId"     => external_id,
      "hidePasswords"  => hide_passwords,
      "id"             => id,
      "name"           => collection_name,
      "organizationId" => organization_id,
      "readOnly"       => read_only,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class SendFile < Dry::Struct
  attribute :file_name, Types::String
  attribute :id,        Types::String
  attribute :size,      Types::String

  # Readable size, ex: "4.2 KB" or "1.43 GB"
  attribute :size_name, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      file_name: d.fetch("fileName"),
      id:        d.fetch("id"),
      size:      d.fetch("size"),
      size_name: d.fetch("sizeName"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "fileName" => file_name,
      "id"       => id,
      "size"     => size,
      "sizeName" => size_name,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

module SendType
  File = "File"
  Text = "Text"
end

class SendText < Dry::Struct
  attribute :hidden, Types::Bool
  attribute :text,   Types::String.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      hidden: d.fetch("hidden"),
      text:   d["text"],
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "hidden" => hidden,
      "text"   => text,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class Send < Dry::Struct
  attribute :access_count,     Types::Integer
  attribute :access_id,        Types::String
  attribute :deletion_date,    Types::String
  attribute :disabled,         Types::Bool
  attribute :expiration_date,  Types::String.optional.optional
  attribute :file,             SendFile.optional.optional
  attribute :hide_email,       Types::Bool
  attribute :id,               Types::String
  attribute :key,              Types::String
  attribute :max_access_count, Types::Integer.optional.optional
  attribute :send_name,        Types::String
  attribute :notes,            Types::String.optional.optional
  attribute :password,         Types::String.optional.optional
  attribute :revision_date,    Types::String
  attribute :text,             SendText.optional.optional
  attribute :send_type,        Types::SendType

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      access_count:     d.fetch("accessCount"),
      access_id:        d.fetch("accessId"),
      deletion_date:    d.fetch("deletionDate"),
      disabled:         d.fetch("disabled"),
      expiration_date:  d["expirationDate"],
      file:             d["file"] ? SendFile.from_dynamic!(d["file"]) : nil,
      hide_email:       d.fetch("hideEmail"),
      id:               d.fetch("id"),
      key:              d.fetch("key"),
      max_access_count: d["maxAccessCount"],
      send_name:        d.fetch("name"),
      notes:            d["notes"],
      password:         d["password"],
      revision_date:    d.fetch("revisionDate"),
      text:             d["text"] ? SendText.from_dynamic!(d["text"]) : nil,
      send_type:        d.fetch("type"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "accessCount"    => access_count,
      "accessId"       => access_id,
      "deletionDate"   => deletion_date,
      "disabled"       => disabled,
      "expirationDate" => expiration_date,
      "file"           => file&.to_dynamic,
      "hideEmail"      => hide_email,
      "id"             => id,
      "key"            => key,
      "maxAccessCount" => max_access_count,
      "name"           => send_name,
      "notes"          => notes,
      "password"       => password,
      "revisionDate"   => revision_date,
      "text"           => text&.to_dynamic,
      "type"           => send_type,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class EncryptedJSON < Dry::Struct
  attribute :password, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      password: d.fetch("password"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "password" => password,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class ExportFormatClass < Dry::Struct
  attribute :encrypted_json, EncryptedJSON

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      encrypted_json: EncryptedJSON.from_dynamic!(d.fetch("EncryptedJson")),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "EncryptedJson" => encrypted_json.to_dynamic,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

module ExportFormatEnum
  AccountEncryptedJSON = "AccountEncryptedJson"
  CSV                  = "Csv"
  JSON                 = "Json"
end

class ExportFormat < Dry::Struct
  attribute :enum,                Types::ExportFormatEnum.optional
  attribute :export_format_class, ExportFormatClass.optional

  def self.from_dynamic!(d)
    if schema[:enum].right.valid? d
      return new(enum: d, export_format_class: nil)
    end
    begin
      value = ExportFormatClass.from_dynamic!(d)
      if schema[:export_format_class].right.valid? value
        return new(export_format_class: value, enum: nil)
      end
    rescue
    end
    raise "Invalid union"
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    if enum != nil
      enum
    elsif export_format_class != nil
      export_format_class.to_dynamic
    end
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class Folder < Dry::Struct
  attribute :id,            Types::String
  attribute :folder_name,   Types::String
  attribute :revision_date, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      id:            d.fetch("id"),
      folder_name:   d.fetch("name"),
      revision_date: d.fetch("revisionDate"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "id"           => id,
      "name"         => folder_name,
      "revisionDate" => revision_date,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class FolderView < Dry::Struct
  attribute :id,               Types::String
  attribute :folder_view_name, Types::String
  attribute :revision_date,    Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      id:               d.fetch("id"),
      folder_view_name: d.fetch("name"),
      revision_date:    d.fetch("revisionDate"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "id"           => id,
      "name"         => folder_view_name,
      "revisionDate" => revision_date,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class Argon2ID2 < Dry::Struct
  attribute :iterations,  Types::Integer
  attribute :memory,      Types::Integer
  attribute :parallelism, Types::Integer

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      iterations:  d.fetch("iterations"),
      memory:      d.fetch("memory"),
      parallelism: d.fetch("parallelism"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "iterations"  => iterations,
      "memory"      => memory,
      "parallelism" => parallelism,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class PBKDF22 < Dry::Struct
  attribute :iterations, Types::Integer

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      iterations: d.fetch("iterations"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "iterations" => iterations,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

# The user's KDF parameters, as received from the prelogin request
class InitCryptoRequestKdf < Dry::Struct
  attribute :p_bkdf2,   PBKDF22.optional
  attribute :argon2_id, Argon2ID2.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      p_bkdf2:   d["pBKDF2"] ? PBKDF22.from_dynamic!(d["pBKDF2"]) : nil,
      argon2_id: d["argon2id"] ? Argon2ID2.from_dynamic!(d["argon2id"]) : nil,
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "pBKDF2"   => p_bkdf2&.to_dynamic,
      "argon2id" => argon2_id&.to_dynamic,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class InitCryptoRequest < Dry::Struct

  # The user's email address
  attribute :email, Types::String

  # The user's KDF parameters, as received from the prelogin request
  attribute :kdf_params, InitCryptoRequestKdf

  # The encryption keys for all the organizations the user is a part of
  attribute :organization_keys, Types::Hash.meta(of: Types::String)

  # The user's master password
  attribute :password, Types::String

  # The user's encryptred private key
  attribute :private_key, Types::String

  # The user's encrypted symmetric crypto key
  attribute :user_key, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      email:             d.fetch("email"),
      kdf_params:        InitCryptoRequestKdf.from_dynamic!(d.fetch("kdfParams")),
      organization_keys: Types::Hash[d.fetch("organizationKeys")].map { |k, v| [k, Types::String[v]] }.to_h,
      password:          d.fetch("password"),
      private_key:       d.fetch("privateKey"),
      user_key:          d.fetch("userKey"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "email"            => email,
      "kdfParams"        => kdf_params.to_dynamic,
      "organizationKeys" => organization_keys,
      "password"         => password,
      "privateKey"       => private_key,
      "userKey"          => user_key,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class MasterPasswordPolicyOptions < Dry::Struct

  # Flag to indicate if the policy should be enforced on login. If true, and the user's
  # password does not meet the policy requirements, the user will be forced to update their
  # password.
  attribute :enforce_on_login, Types::Bool

  attribute :min_complexity,  Types::Integer
  attribute :min_length,      Types::Integer
  attribute :require_lower,   Types::Bool
  attribute :require_numbers, Types::Bool
  attribute :require_special, Types::Bool
  attribute :require_upper,   Types::Bool

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      enforce_on_login: d.fetch("enforce_on_login"),
      min_complexity:   d.fetch("min_complexity"),
      min_length:       d.fetch("min_length"),
      require_lower:    d.fetch("require_lower"),
      require_numbers:  d.fetch("require_numbers"),
      require_special:  d.fetch("require_special"),
      require_upper:    d.fetch("require_upper"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "enforce_on_login" => enforce_on_login,
      "min_complexity"   => min_complexity,
      "min_length"       => min_length,
      "require_lower"    => require_lower,
      "require_numbers"  => require_numbers,
      "require_special"  => require_special,
      "require_upper"    => require_upper,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

# Passphrase generator request.
#
# The default separator is `-` and default number of words is 3.
class PassphraseGeneratorRequest < Dry::Struct
  attribute :capitalize,     Types::Bool.optional.optional
  attribute :include_number, Types::Bool.optional.optional
  attribute :num_words,      Types::Integer.optional.optional
  attribute :word_separator, Types::String.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      capitalize:     d["capitalize"],
      include_number: d["includeNumber"],
      num_words:      d["numWords"],
      word_separator: d["wordSeparator"],
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "capitalize"    => capitalize,
      "includeNumber" => include_number,
      "numWords"      => num_words,
      "wordSeparator" => word_separator,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

# Password generator request. If all options are false, the default is to generate a
# password with: - lowercase - uppercase - numbers
#
# The default length is 16.
class PasswordGeneratorRequest < Dry::Struct
  attribute :avoid_ambiguous, Types::Bool.optional.optional
  attribute :length,          Types::Integer.optional.optional
  attribute :lowercase,       Types::Bool
  attribute :min_lowercase,   Types::Bool.optional.optional
  attribute :min_number,      Types::Bool.optional.optional
  attribute :min_special,     Types::Bool.optional.optional
  attribute :min_uppercase,   Types::Bool.optional.optional
  attribute :numbers,         Types::Bool
  attribute :special,         Types::Bool
  attribute :uppercase,       Types::Bool

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      avoid_ambiguous: d["avoidAmbiguous"],
      length:          d["length"],
      lowercase:       d.fetch("lowercase"),
      min_lowercase:   d["minLowercase"],
      min_number:      d["minNumber"],
      min_special:     d["minSpecial"],
      min_uppercase:   d["minUppercase"],
      numbers:         d.fetch("numbers"),
      special:         d.fetch("special"),
      uppercase:       d.fetch("uppercase"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "avoidAmbiguous" => avoid_ambiguous,
      "length"         => length,
      "lowercase"      => lowercase,
      "minLowercase"   => min_lowercase,
      "minNumber"      => min_number,
      "minSpecial"     => min_special,
      "minUppercase"   => min_uppercase,
      "numbers"        => numbers,
      "special"        => special,
      "uppercase"      => uppercase,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class SendListView < Dry::Struct
  attribute :access_id,           Types::String
  attribute :deletion_date,       Types::String
  attribute :disabled,            Types::Bool
  attribute :expiration_date,     Types::String.optional.optional
  attribute :id,                  Types::String
  attribute :send_list_view_name, Types::String
  attribute :revision_date,       Types::String
  attribute :send_list_view_type, Types::SendType

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      access_id:           d.fetch("accessId"),
      deletion_date:       d.fetch("deletionDate"),
      disabled:            d.fetch("disabled"),
      expiration_date:     d["expirationDate"],
      id:                  d.fetch("id"),
      send_list_view_name: d.fetch("name"),
      revision_date:       d.fetch("revisionDate"),
      send_list_view_type: d.fetch("type"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "accessId"       => access_id,
      "deletionDate"   => deletion_date,
      "disabled"       => disabled,
      "expirationDate" => expiration_date,
      "id"             => id,
      "name"           => send_list_view_name,
      "revisionDate"   => revision_date,
      "type"           => send_list_view_type,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class SendFileView < Dry::Struct
  attribute :file_name, Types::String
  attribute :id,        Types::String
  attribute :size,      Types::String

  # Readable size, ex: "4.2 KB" or "1.43 GB"
  attribute :size_name, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      file_name: d.fetch("fileName"),
      id:        d.fetch("id"),
      size:      d.fetch("size"),
      size_name: d.fetch("sizeName"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "fileName" => file_name,
      "id"       => id,
      "size"     => size,
      "sizeName" => size_name,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class SendTextView < Dry::Struct
  attribute :hidden, Types::Bool
  attribute :text,   Types::String.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      hidden: d.fetch("hidden"),
      text:   d["text"],
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "hidden" => hidden,
      "text"   => text,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class SendView < Dry::Struct
  attribute :access_count,     Types::Integer
  attribute :access_id,        Types::String
  attribute :deletion_date,    Types::String
  attribute :disabled,         Types::Bool
  attribute :expiration_date,  Types::String.optional.optional
  attribute :file,             SendFileView.optional.optional
  attribute :hide_email,       Types::Bool
  attribute :id,               Types::String
  attribute :key,              Types::String
  attribute :max_access_count, Types::Integer.optional.optional
  attribute :send_view_name,   Types::String
  attribute :notes,            Types::String.optional.optional
  attribute :password,         Types::String.optional.optional
  attribute :revision_date,    Types::String
  attribute :text,             SendTextView.optional.optional
  attribute :send_view_type,   Types::SendType

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      access_count:     d.fetch("accessCount"),
      access_id:        d.fetch("accessId"),
      deletion_date:    d.fetch("deletionDate"),
      disabled:         d.fetch("disabled"),
      expiration_date:  d["expirationDate"],
      file:             d["file"] ? SendFileView.from_dynamic!(d["file"]) : nil,
      hide_email:       d.fetch("hideEmail"),
      id:               d.fetch("id"),
      key:              d.fetch("key"),
      max_access_count: d["maxAccessCount"],
      send_view_name:   d.fetch("name"),
      notes:            d["notes"],
      password:         d["password"],
      revision_date:    d.fetch("revisionDate"),
      text:             d["text"] ? SendTextView.from_dynamic!(d["text"]) : nil,
      send_view_type:   d.fetch("type"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "accessCount"    => access_count,
      "accessId"       => access_id,
      "deletionDate"   => deletion_date,
      "disabled"       => disabled,
      "expirationDate" => expiration_date,
      "file"           => file&.to_dynamic,
      "hideEmail"      => hide_email,
      "id"             => id,
      "key"            => key,
      "maxAccessCount" => max_access_count,
      "name"           => send_view_name,
      "notes"          => notes,
      "password"       => password,
      "revisionDate"   => revision_date,
      "text"           => text&.to_dynamic,
      "type"           => send_view_type,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class TotpResponse < Dry::Struct

  # Generated TOTP code
  attribute :code, Types::String

  # Time period
  attribute :period, Types::Integer

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      code:   d.fetch("code"),
      period: d.fetch("period"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "code"   => code,
      "period" => period,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

# TOTP
class DocRef < Dry::Struct
  attribute :cipher,                         Cipher.optional
  attribute :cipher_view,                    CipherView.optional
  attribute :collection,                     Collection.optional
  attribute :folder,                         Folder.optional
  attribute :folder_view,                    FolderView.optional
  attribute :doc_ref_send,                   Send.optional
  attribute :send_view,                      SendView.optional
  attribute :send_list_view,                 SendListView.optional
  attribute :init_crypto_request,            InitCryptoRequest.optional
  attribute :password_generator_request,     PasswordGeneratorRequest.optional
  attribute :passphrase_generator_request,   PassphraseGeneratorRequest.optional
  attribute :export_format,                  Types.Instance(ExportFormat).optional
  attribute :master_password_policy_options, MasterPasswordPolicyOptions.optional
  attribute :kdf,                            InitCryptoRequestKdf.optional
  attribute :totp_response,                  TotpResponse.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      cipher:                         d["Cipher"] ? Cipher.from_dynamic!(d["Cipher"]) : nil,
      cipher_view:                    d["CipherView"] ? CipherView.from_dynamic!(d["CipherView"]) : nil,
      collection:                     d["Collection"] ? Collection.from_dynamic!(d["Collection"]) : nil,
      folder:                         d["Folder"] ? Folder.from_dynamic!(d["Folder"]) : nil,
      folder_view:                    d["FolderView"] ? FolderView.from_dynamic!(d["FolderView"]) : nil,
      doc_ref_send:                   d["Send"] ? Send.from_dynamic!(d["Send"]) : nil,
      send_view:                      d["SendView"] ? SendView.from_dynamic!(d["SendView"]) : nil,
      send_list_view:                 d["SendListView"] ? SendListView.from_dynamic!(d["SendListView"]) : nil,
      init_crypto_request:            d["InitCryptoRequest"] ? InitCryptoRequest.from_dynamic!(d["InitCryptoRequest"]) : nil,
      password_generator_request:     d["PasswordGeneratorRequest"] ? PasswordGeneratorRequest.from_dynamic!(d["PasswordGeneratorRequest"]) : nil,
      passphrase_generator_request:   d["PassphraseGeneratorRequest"] ? PassphraseGeneratorRequest.from_dynamic!(d["PassphraseGeneratorRequest"]) : nil,
      export_format:                  d["ExportFormat"] ? ExportFormat.from_dynamic!(d["ExportFormat"]) : nil,
      master_password_policy_options: d["MasterPasswordPolicyOptions"] ? MasterPasswordPolicyOptions.from_dynamic!(d["MasterPasswordPolicyOptions"]) : nil,
      kdf:                            d["Kdf"] ? InitCryptoRequestKdf.from_dynamic!(d["Kdf"]) : nil,
      totp_response:                  d["TotpResponse"] ? TotpResponse.from_dynamic!(d["TotpResponse"]) : nil,
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "Cipher"                      => cipher&.to_dynamic,
      "CipherView"                  => cipher_view&.to_dynamic,
      "Collection"                  => collection&.to_dynamic,
      "Folder"                      => folder&.to_dynamic,
      "FolderView"                  => folder_view&.to_dynamic,
      "Send"                        => doc_ref_send&.to_dynamic,
      "SendView"                    => send_view&.to_dynamic,
      "SendListView"                => send_list_view&.to_dynamic,
      "InitCryptoRequest"           => init_crypto_request&.to_dynamic,
      "PasswordGeneratorRequest"    => password_generator_request&.to_dynamic,
      "PassphraseGeneratorRequest"  => passphrase_generator_request&.to_dynamic,
      "ExportFormat"                => export_format&.to_dynamic,
      "MasterPasswordPolicyOptions" => master_password_policy_options&.to_dynamic,
      "Kdf"                         => kdf&.to_dynamic,
      "TotpResponse"                => totp_response&.to_dynamic,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class Authenticator1 < Dry::Struct

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class Duo1 < Dry::Struct
  attribute :host,      Types::String
  attribute :signature, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      host:      d.fetch("host"),
      signature: d.fetch("signature"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "host"      => host,
      "signature" => signature,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class Email1 < Dry::Struct

  # The email to request a 2fa TOTP for
  attribute :email, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      email: d.fetch("email"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "email" => email,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class Remember1 < Dry::Struct

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class WebAuthn1 < Dry::Struct

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class YubiKey1 < Dry::Struct

  # Whether the stored yubikey supports near field communication
  attribute :nfc, Types::Bool

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      nfc: d.fetch("nfc"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "nfc" => nfc,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class APIKeyLoginResponseTwoFactorProviders < Dry::Struct
  attribute :authenticator, Authenticator1.optional.optional

  # Duo-backed 2fa
  attribute :duo, Duo1.optional.optional

  # Email 2fa
  attribute :email, Email1.optional.optional

  # Duo-backed 2fa operated by an organization the user is a member of
  attribute :organization_duo, Duo1.optional.optional

  # Presence indicates the user has stored this device as bypassing 2fa
  attribute :remember, Remember1.optional.optional

  # WebAuthn-backed 2fa
  attribute :web_authn, WebAuthn1.optional.optional

  # Yubikey-backed 2fa
  attribute :yubi_key, YubiKey1.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      authenticator:    d["authenticator"] ? Authenticator1.from_dynamic!(d["authenticator"]) : nil,
      duo:              d["duo"] ? Duo1.from_dynamic!(d["duo"]) : nil,
      email:            d["email"] ? Email1.from_dynamic!(d["email"]) : nil,
      organization_duo: d["organizationDuo"] ? Duo1.from_dynamic!(d["organizationDuo"]) : nil,
      remember:         d["remember"] ? Remember1.from_dynamic!(d["remember"]) : nil,
      web_authn:        d["webAuthn"] ? WebAuthn1.from_dynamic!(d["webAuthn"]) : nil,
      yubi_key:         d["yubiKey"] ? YubiKey1.from_dynamic!(d["yubiKey"]) : nil,
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "authenticator"   => authenticator&.to_dynamic,
      "duo"             => duo&.to_dynamic,
      "email"           => email&.to_dynamic,
      "organizationDuo" => organization_duo&.to_dynamic,
      "remember"        => remember&.to_dynamic,
      "webAuthn"        => web_authn&.to_dynamic,
      "yubiKey"         => yubi_key&.to_dynamic,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class APIKeyLoginResponse < Dry::Struct
  attribute :authenticated, Types::Bool

  # Whether or not the user is required to update their master password
  attribute :force_password_reset, Types::Bool

  # TODO: What does this do?
  attribute :reset_master_password, Types::Bool

  attribute :two_factor, APIKeyLoginResponseTwoFactorProviders.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      authenticated:         d.fetch("authenticated"),
      force_password_reset:  d.fetch("forcePasswordReset"),
      reset_master_password: d.fetch("resetMasterPassword"),
      two_factor:            d["twoFactor"] ? APIKeyLoginResponseTwoFactorProviders.from_dynamic!(d["twoFactor"]) : nil,
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "authenticated"       => authenticated,
      "forcePasswordReset"  => force_password_reset,
      "resetMasterPassword" => reset_master_password,
      "twoFactor"           => two_factor&.to_dynamic,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class ResponseForAPIKeyLoginResponse < Dry::Struct

  # The response data. Populated if `success` is true.
  attribute :data, APIKeyLoginResponse.optional.optional

  # A message for any error that may occur. Populated if `success` is false.
  attribute :error_message, Types::String.optional.optional

  # Whether or not the SDK request succeeded.
  attribute :success, Types::Bool

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data:          d["data"] ? APIKeyLoginResponse.from_dynamic!(d["data"]) : nil,
      error_message: d["errorMessage"],
      success:       d.fetch("success"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "data"         => data&.to_dynamic,
      "errorMessage" => error_message,
      "success"      => success,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class FingerprintResponse < Dry::Struct
  attribute :fingerprint, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      fingerprint: d.fetch("fingerprint"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "fingerprint" => fingerprint,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class ResponseForFingerprintResponse < Dry::Struct

  # The response data. Populated if `success` is true.
  attribute :data, FingerprintResponse.optional.optional

  # A message for any error that may occur. Populated if `success` is false.
  attribute :error_message, Types::String.optional.optional

  # Whether or not the SDK request succeeded.
  attribute :success, Types::Bool

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data:          d["data"] ? FingerprintResponse.from_dynamic!(d["data"]) : nil,
      error_message: d["errorMessage"],
      success:       d.fetch("success"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "data"         => data&.to_dynamic,
      "errorMessage" => error_message,
      "success"      => success,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class CAPTCHAResponse < Dry::Struct

  # hcaptcha site key
  attribute :site_key, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      site_key: d.fetch("siteKey"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "siteKey" => site_key,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class Authenticator2 < Dry::Struct

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class Duo2 < Dry::Struct
  attribute :host,      Types::String
  attribute :signature, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      host:      d.fetch("host"),
      signature: d.fetch("signature"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "host"      => host,
      "signature" => signature,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class Email2 < Dry::Struct

  # The email to request a 2fa TOTP for
  attribute :email, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      email: d.fetch("email"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "email" => email,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class Remember2 < Dry::Struct

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class WebAuthn2 < Dry::Struct

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class YubiKey2 < Dry::Struct

  # Whether the stored yubikey supports near field communication
  attribute :nfc, Types::Bool

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      nfc: d.fetch("nfc"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "nfc" => nfc,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class PasswordLoginResponseTwoFactorProviders < Dry::Struct
  attribute :authenticator, Authenticator2.optional.optional

  # Duo-backed 2fa
  attribute :duo, Duo2.optional.optional

  # Email 2fa
  attribute :email, Email2.optional.optional

  # Duo-backed 2fa operated by an organization the user is a member of
  attribute :organization_duo, Duo2.optional.optional

  # Presence indicates the user has stored this device as bypassing 2fa
  attribute :remember, Remember2.optional.optional

  # WebAuthn-backed 2fa
  attribute :web_authn, WebAuthn2.optional.optional

  # Yubikey-backed 2fa
  attribute :yubi_key, YubiKey2.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      authenticator:    d["authenticator"] ? Authenticator2.from_dynamic!(d["authenticator"]) : nil,
      duo:              d["duo"] ? Duo2.from_dynamic!(d["duo"]) : nil,
      email:            d["email"] ? Email2.from_dynamic!(d["email"]) : nil,
      organization_duo: d["organizationDuo"] ? Duo2.from_dynamic!(d["organizationDuo"]) : nil,
      remember:         d["remember"] ? Remember2.from_dynamic!(d["remember"]) : nil,
      web_authn:        d["webAuthn"] ? WebAuthn2.from_dynamic!(d["webAuthn"]) : nil,
      yubi_key:         d["yubiKey"] ? YubiKey2.from_dynamic!(d["yubiKey"]) : nil,
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "authenticator"   => authenticator&.to_dynamic,
      "duo"             => duo&.to_dynamic,
      "email"           => email&.to_dynamic,
      "organizationDuo" => organization_duo&.to_dynamic,
      "remember"        => remember&.to_dynamic,
      "webAuthn"        => web_authn&.to_dynamic,
      "yubiKey"         => yubi_key&.to_dynamic,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class PasswordLoginResponse < Dry::Struct
  attribute :authenticated, Types::Bool

  # The information required to present the user with a captcha challenge. Only present when
  # authentication fails due to requiring validation of a captcha challenge.
  attribute :captcha, CAPTCHAResponse.optional.optional

  # Whether or not the user is required to update their master password
  attribute :force_password_reset, Types::Bool

  # TODO: What does this do?
  attribute :reset_master_password, Types::Bool

  # The available two factor authentication options. Present only when authentication fails
  # due to requiring a second authentication factor.
  attribute :two_factor, PasswordLoginResponseTwoFactorProviders.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      authenticated:         d.fetch("authenticated"),
      captcha:               d["captcha"] ? CAPTCHAResponse.from_dynamic!(d["captcha"]) : nil,
      force_password_reset:  d.fetch("forcePasswordReset"),
      reset_master_password: d.fetch("resetMasterPassword"),
      two_factor:            d["twoFactor"] ? PasswordLoginResponseTwoFactorProviders.from_dynamic!(d["twoFactor"]) : nil,
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "authenticated"       => authenticated,
      "captcha"             => captcha&.to_dynamic,
      "forcePasswordReset"  => force_password_reset,
      "resetMasterPassword" => reset_master_password,
      "twoFactor"           => two_factor&.to_dynamic,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class ResponseForPasswordLoginResponse < Dry::Struct

  # The response data. Populated if `success` is true.
  attribute :data, PasswordLoginResponse.optional.optional

  # A message for any error that may occur. Populated if `success` is false.
  attribute :error_message, Types::String.optional.optional

  # Whether or not the SDK request succeeded.
  attribute :success, Types::Bool

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data:          d["data"] ? PasswordLoginResponse.from_dynamic!(d["data"]) : nil,
      error_message: d["errorMessage"],
      success:       d.fetch("success"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "data"         => data&.to_dynamic,
      "errorMessage" => error_message,
      "success"      => success,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class ProjectResponse < Dry::Struct
  attribute :creation_date,         Types::String
  attribute :id,                    Types::String
  attribute :project_response_name, Types::String
  attribute :organization_id,       Types::String
  attribute :revision_date,         Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      creation_date:         d.fetch("creationDate"),
      id:                    d.fetch("id"),
      project_response_name: d.fetch("name"),
      organization_id:       d.fetch("organizationId"),
      revision_date:         d.fetch("revisionDate"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "creationDate"   => creation_date,
      "id"             => id,
      "name"           => project_response_name,
      "organizationId" => organization_id,
      "revisionDate"   => revision_date,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class ResponseForProjectResponse < Dry::Struct

  # The response data. Populated if `success` is true.
  attribute :data, ProjectResponse.optional.optional

  # A message for any error that may occur. Populated if `success` is false.
  attribute :error_message, Types::String.optional.optional

  # Whether or not the SDK request succeeded.
  attribute :success, Types::Bool

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data:          d["data"] ? ProjectResponse.from_dynamic!(d["data"]) : nil,
      error_message: d["errorMessage"],
      success:       d.fetch("success"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "data"         => data&.to_dynamic,
      "errorMessage" => error_message,
      "success"      => success,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class ProjectDeleteResponse < Dry::Struct
  attribute :error, Types::String.optional.optional
  attribute :id,    Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      error: d["error"],
      id:    d.fetch("id"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "error" => error,
      "id"    => id,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class ProjectsDeleteResponse < Dry::Struct
  attribute :data, Types.Array(ProjectDeleteResponse)

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data: d.fetch("data").map { |x| ProjectDeleteResponse.from_dynamic!(x) },
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "data" => data.map { |x| x.to_dynamic },
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class ResponseForProjectsDeleteResponse < Dry::Struct

  # The response data. Populated if `success` is true.
  attribute :data, ProjectsDeleteResponse.optional.optional

  # A message for any error that may occur. Populated if `success` is false.
  attribute :error_message, Types::String.optional.optional

  # Whether or not the SDK request succeeded.
  attribute :success, Types::Bool

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data:          d["data"] ? ProjectsDeleteResponse.from_dynamic!(d["data"]) : nil,
      error_message: d["errorMessage"],
      success:       d.fetch("success"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "data"         => data&.to_dynamic,
      "errorMessage" => error_message,
      "success"      => success,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class DatumElement < Dry::Struct
  attribute :creation_date,         Types::String
  attribute :id,                    Types::String
  attribute :project_response_name, Types::String
  attribute :organization_id,       Types::String
  attribute :revision_date,         Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      creation_date:         d.fetch("creationDate"),
      id:                    d.fetch("id"),
      project_response_name: d.fetch("name"),
      organization_id:       d.fetch("organizationId"),
      revision_date:         d.fetch("revisionDate"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "creationDate"   => creation_date,
      "id"             => id,
      "name"           => project_response_name,
      "organizationId" => organization_id,
      "revisionDate"   => revision_date,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class ProjectsResponse < Dry::Struct
  attribute :data, Types.Array(DatumElement)

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data: d.fetch("data").map { |x| DatumElement.from_dynamic!(x) },
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "data" => data.map { |x| x.to_dynamic },
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class ResponseForProjectsResponse < Dry::Struct

  # The response data. Populated if `success` is true.
  attribute :data, ProjectsResponse.optional.optional

  # A message for any error that may occur. Populated if `success` is false.
  attribute :error_message, Types::String.optional.optional

  # Whether or not the SDK request succeeded.
  attribute :success, Types::Bool

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data:          d["data"] ? ProjectsResponse.from_dynamic!(d["data"]) : nil,
      error_message: d["errorMessage"],
      success:       d.fetch("success"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "data"         => data&.to_dynamic,
      "errorMessage" => error_message,
      "success"      => success,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class SecretIdentifierResponse < Dry::Struct
  attribute :id,              Types::String
  attribute :key,             Types::String
  attribute :organization_id, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      id:              d.fetch("id"),
      key:             d.fetch("key"),
      organization_id: d.fetch("organizationId"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "id"             => id,
      "key"            => key,
      "organizationId" => organization_id,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class SecretIdentifiersResponse < Dry::Struct
  attribute :data, Types.Array(SecretIdentifierResponse)

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data: d.fetch("data").map { |x| SecretIdentifierResponse.from_dynamic!(x) },
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "data" => data.map { |x| x.to_dynamic },
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class ResponseForSecretIdentifiersResponse < Dry::Struct

  # The response data. Populated if `success` is true.
  attribute :data, SecretIdentifiersResponse.optional.optional

  # A message for any error that may occur. Populated if `success` is false.
  attribute :error_message, Types::String.optional.optional

  # Whether or not the SDK request succeeded.
  attribute :success, Types::Bool

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data:          d["data"] ? SecretIdentifiersResponse.from_dynamic!(d["data"]) : nil,
      error_message: d["errorMessage"],
      success:       d.fetch("success"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "data"         => data&.to_dynamic,
      "errorMessage" => error_message,
      "success"      => success,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class SecretResponse < Dry::Struct
  attribute :creation_date,   Types::String
  attribute :id,              Types::String
  attribute :key,             Types::String
  attribute :note,            Types::String
  attribute :organization_id, Types::String
  attribute :project_id,      Types::String.optional.optional
  attribute :revision_date,   Types::String
  attribute :value,           Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      creation_date:   d.fetch("creationDate"),
      id:              d.fetch("id"),
      key:             d.fetch("key"),
      note:            d.fetch("note"),
      organization_id: d.fetch("organizationId"),
      project_id:      d["projectId"],
      revision_date:   d.fetch("revisionDate"),
      value:           d.fetch("value"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "creationDate"   => creation_date,
      "id"             => id,
      "key"            => key,
      "note"           => note,
      "organizationId" => organization_id,
      "projectId"      => project_id,
      "revisionDate"   => revision_date,
      "value"          => value,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class ResponseForSecretResponse < Dry::Struct

  # The response data. Populated if `success` is true.
  attribute :data, SecretResponse.optional.optional

  # A message for any error that may occur. Populated if `success` is false.
  attribute :error_message, Types::String.optional.optional

  # Whether or not the SDK request succeeded.
  attribute :success, Types::Bool

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data:          d["data"] ? SecretResponse.from_dynamic!(d["data"]) : nil,
      error_message: d["errorMessage"],
      success:       d.fetch("success"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "data"         => data&.to_dynamic,
      "errorMessage" => error_message,
      "success"      => success,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class SecretDeleteResponse < Dry::Struct
  attribute :error, Types::String.optional.optional
  attribute :id,    Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      error: d["error"],
      id:    d.fetch("id"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "error" => error,
      "id"    => id,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class SecretsDeleteResponse < Dry::Struct
  attribute :data, Types.Array(SecretDeleteResponse)

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data: d.fetch("data").map { |x| SecretDeleteResponse.from_dynamic!(x) },
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "data" => data.map { |x| x.to_dynamic },
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class ResponseForSecretsDeleteResponse < Dry::Struct

  # The response data. Populated if `success` is true.
  attribute :data, SecretsDeleteResponse.optional.optional

  # A message for any error that may occur. Populated if `success` is false.
  attribute :error_message, Types::String.optional.optional

  # Whether or not the SDK request succeeded.
  attribute :success, Types::Bool

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data:          d["data"] ? SecretsDeleteResponse.from_dynamic!(d["data"]) : nil,
      error_message: d["errorMessage"],
      success:       d.fetch("success"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "data"         => data&.to_dynamic,
      "errorMessage" => error_message,
      "success"      => success,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class DatumClass < Dry::Struct
  attribute :creation_date,   Types::String
  attribute :id,              Types::String
  attribute :key,             Types::String
  attribute :note,            Types::String
  attribute :organization_id, Types::String
  attribute :project_id,      Types::String.optional.optional
  attribute :revision_date,   Types::String
  attribute :value,           Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      creation_date:   d.fetch("creationDate"),
      id:              d.fetch("id"),
      key:             d.fetch("key"),
      note:            d.fetch("note"),
      organization_id: d.fetch("organizationId"),
      project_id:      d["projectId"],
      revision_date:   d.fetch("revisionDate"),
      value:           d.fetch("value"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "creationDate"   => creation_date,
      "id"             => id,
      "key"            => key,
      "note"           => note,
      "organizationId" => organization_id,
      "projectId"      => project_id,
      "revisionDate"   => revision_date,
      "value"          => value,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class SecretsResponse < Dry::Struct
  attribute :data, Types.Array(DatumClass)

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data: d.fetch("data").map { |x| DatumClass.from_dynamic!(x) },
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "data" => data.map { |x| x.to_dynamic },
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class ResponseForSecretsResponse < Dry::Struct

  # The response data. Populated if `success` is true.
  attribute :data, SecretsResponse.optional.optional

  # A message for any error that may occur. Populated if `success` is false.
  attribute :error_message, Types::String.optional.optional

  # Whether or not the SDK request succeeded.
  attribute :success, Types::Bool

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data:          d["data"] ? SecretsResponse.from_dynamic!(d["data"]) : nil,
      error_message: d["errorMessage"],
      success:       d.fetch("success"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "data"         => data&.to_dynamic,
      "errorMessage" => error_message,
      "success"      => success,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class CipherDetailsResponse < Dry::Struct

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class ProfileOrganizationResponse < Dry::Struct
  attribute :id, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      id: d.fetch("id"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "id" => id,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

# Data about the user, including their encryption keys and the organizations they are a
# part of
class ProfileResponse < Dry::Struct
  attribute :email,                 Types::String
  attribute :id,                    Types::String
  attribute :profile_response_name, Types::String
  attribute :organizations,         Types.Array(ProfileOrganizationResponse)

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      email:                 d.fetch("email"),
      id:                    d.fetch("id"),
      profile_response_name: d.fetch("name"),
      organizations:         d.fetch("organizations").map { |x| ProfileOrganizationResponse.from_dynamic!(x) },
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "email"         => email,
      "id"            => id,
      "name"          => profile_response_name,
      "organizations" => organizations.map { |x| x.to_dynamic },
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class SyncResponse < Dry::Struct

  # List of ciphers accesible by the user
  attribute :ciphers, Types.Array(CipherDetailsResponse)

  # Data about the user, including their encryption keys and the organizations they are a
  # part of
  attribute :profile, ProfileResponse

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      ciphers: d.fetch("ciphers").map { |x| CipherDetailsResponse.from_dynamic!(x) },
      profile: ProfileResponse.from_dynamic!(d.fetch("profile")),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "ciphers" => ciphers.map { |x| x.to_dynamic },
      "profile" => profile.to_dynamic,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class ResponseForSyncResponse < Dry::Struct

  # The response data. Populated if `success` is true.
  attribute :data, SyncResponse.optional.optional

  # A message for any error that may occur. Populated if `success` is false.
  attribute :error_message, Types::String.optional.optional

  # Whether or not the SDK request succeeded.
  attribute :success, Types::Bool

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data:          d["data"] ? SyncResponse.from_dynamic!(d["data"]) : nil,
      error_message: d["errorMessage"],
      success:       d.fetch("success"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "data"         => data&.to_dynamic,
      "errorMessage" => error_message,
      "success"      => success,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class UserAPIKeyResponse < Dry::Struct

  # The user's API key, which represents the client_secret portion of an oauth request.
  attribute :api_key, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      api_key: d.fetch("apiKey"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "apiKey" => api_key,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class ResponseForUserAPIKeyResponse < Dry::Struct

  # The response data. Populated if `success` is true.
  attribute :data, UserAPIKeyResponse.optional.optional

  # A message for any error that may occur. Populated if `success` is false.
  attribute :error_message, Types::String.optional.optional

  # Whether or not the SDK request succeeded.
  attribute :success, Types::Bool

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data:          d["data"] ? UserAPIKeyResponse.from_dynamic!(d["data"]) : nil,
      error_message: d["errorMessage"],
      success:       d.fetch("success"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "data"         => data&.to_dynamic,
      "errorMessage" => error_message,
      "success"      => success,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

