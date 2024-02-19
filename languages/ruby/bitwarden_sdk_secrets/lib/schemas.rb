# This code may look unusually verbose for Ruby (and it is), but
# it performs some subtle and complex validation of JSON data.
#
# To parse this JSON, add 'dry-struct' and 'dry-types' gems, then do:
#
#   client_settings = ClientSettings.from_json! "{…}"
#   puts client_settings.api_url
#
#   device_type = DeviceType.from_json! "…"
#   puts device_type == DeviceType::Android
#
#   command = Command.from_json! "{…}"
#   puts command.projects&.delete&.ids.first
#
#   password_login_request = PasswordLoginRequest.from_json! "{…}"
#   puts password_login_request.kdf.argon2_id&.iterations.even?
#
#   two_factor_request = TwoFactorRequest.from_json! "{…}"
#   puts two_factor_request.provider == TwoFactorProvider::Authenticator
#
#   two_factor_provider = TwoFactorProvider.from_json! "…"
#   puts two_factor_provider == TwoFactorProvider::Authenticator
#
#   kdf = Kdf.from_json! "{…}"
#   puts kdf.argon2_id&.iterations.even?
#
#   api_key_login_request = APIKeyLoginRequest.from_json! "{…}"
#   puts api_key_login_request.client_id
#
#   access_token_login_request = AccessTokenLoginRequest.from_json! "{…}"
#   puts access_token_login_request.access_token
#
#   secret_verification_request = SecretVerificationRequest.from_json! "{…}"
#   puts secret_verification_request.master_password.nil?
#
#   fingerprint_request = FingerprintRequest.from_json! "{…}"
#   puts fingerprint_request.fingerprint_material
#
#   sync_request = SyncRequest.from_json! "{…}"
#   puts sync_request.exclude_subdomains.nil?
#
#   secrets_command = SecretsCommand.from_json! "{…}"
#   puts secrets_command.delete&.ids.first
#
#   secret_get_request = SecretGetRequest.from_json! "{…}"
#   puts secret_get_request.id
#
#   secrets_get_request = SecretsGetRequest.from_json! "{…}"
#   puts secrets_get_request.ids.first
#
#   secret_create_request = SecretCreateRequest.from_json! "{…}"
#   puts secret_create_request.key
#
#   secret_identifiers_request = SecretIdentifiersRequest.from_json! "{…}"
#   puts secret_identifiers_request.organization_id
#
#   secret_put_request = SecretPutRequest.from_json! "{…}"
#   puts secret_put_request.id
#
#   secrets_delete_request = SecretsDeleteRequest.from_json! "{…}"
#   puts secrets_delete_request.ids.first
#
#   projects_command = ProjectsCommand.from_json! "{…}"
#   puts projects_command.delete&.ids.first
#
#   project_get_request = ProjectGetRequest.from_json! "{…}"
#   puts project_get_request.id
#
#   project_create_request = ProjectCreateRequest.from_json! "{…}"
#   puts project_create_request.project_create_request_name
#
#   projects_list_request = ProjectsListRequest.from_json! "{…}"
#   puts projects_list_request.organization_id
#
#   project_put_request = ProjectPutRequest.from_json! "{…}"
#   puts project_put_request.id
#
#   projects_delete_request = ProjectsDeleteRequest.from_json! "{…}"
#   puts projects_delete_request.ids.first
#
#   response_for_api_key_login_response = ResponseForAPIKeyLoginResponse.from_json! "{…}"
#   puts response_for_api_key_login_response.data&.authenticated
#
#   api_key_login_response = APIKeyLoginResponse.from_json! "{…}"
#   puts api_key_login_response.authenticated
#
#   two_factor_providers = TwoFactorProviders.from_json! "{…}"
#   puts two_factor_providers.authenticator
#
#   authenticator = Authenticator.from_json! "{…}"
#   puts authenticator
#
#   email = Email.from_json! "{…}"
#   puts email.email
#
#   duo = Duo.from_json! "{…}"
#   puts duo.host
#
#   yubi_key = YubiKey.from_json! "{…}"
#   puts yubi_key.nfc
#
#   remember = Remember.from_json! "{…}"
#   puts remember
#
#   web_authn = WebAuthn.from_json! "{…}"
#   puts web_authn
#
#   response_for_password_login_response = ResponseForPasswordLoginResponse.from_json! "{…}"
#   puts response_for_password_login_response.data&.authenticated
#
#   password_login_response = PasswordLoginResponse.from_json! "{…}"
#   puts password_login_response.authenticated
#
#   captcha_response = CAPTCHAResponse.from_json! "{…}"
#   puts captcha_response.site_key
#
#   response_for_access_token_login_response = ResponseForAccessTokenLoginResponse.from_json! "{…}"
#   puts response_for_access_token_login_response.data&.authenticated
#
#   access_token_login_response = AccessTokenLoginResponse.from_json! "{…}"
#   puts access_token_login_response.authenticated
#
#   response_for_secret_identifiers_response = ResponseForSecretIdentifiersResponse.from_json! "{…}"
#   puts response_for_secret_identifiers_response.data&.data.first.id
#
#   secret_identifiers_response = SecretIdentifiersResponse.from_json! "{…}"
#   puts secret_identifiers_response.data.first.id
#
#   secret_identifier_response = SecretIdentifierResponse.from_json! "{…}"
#   puts secret_identifier_response.id
#
#   response_for_secret_response = ResponseForSecretResponse.from_json! "{…}"
#   puts response_for_secret_response.data&.creation_date
#
#   secret_response = SecretResponse.from_json! "{…}"
#   puts secret_response.creation_date
#
#   response_for_secrets_response = ResponseForSecretsResponse.from_json! "{…}"
#   puts response_for_secrets_response.data&.data.first.creation_date
#
#   secrets_response = SecretsResponse.from_json! "{…}"
#   puts secrets_response.data.first.creation_date
#
#   response_for_secrets_delete_response = ResponseForSecretsDeleteResponse.from_json! "{…}"
#   puts response_for_secrets_delete_response.data&.data.first.error.nil?
#
#   secrets_delete_response = SecretsDeleteResponse.from_json! "{…}"
#   puts secrets_delete_response.data.first.error.nil?
#
#   secret_delete_response = SecretDeleteResponse.from_json! "{…}"
#   puts secret_delete_response.error.nil?
#
#   response_for_project_response = ResponseForProjectResponse.from_json! "{…}"
#   puts response_for_project_response.data&.creation_date
#
#   project_response = ProjectResponse.from_json! "{…}"
#   puts project_response.creation_date
#
#   response_for_projects_response = ResponseForProjectsResponse.from_json! "{…}"
#   puts response_for_projects_response.data&.data.first.creation_date
#
#   projects_response = ProjectsResponse.from_json! "{…}"
#   puts projects_response.data.first.creation_date
#
#   response_for_projects_delete_response = ResponseForProjectsDeleteResponse.from_json! "{…}"
#   puts response_for_projects_delete_response.data&.data.first.error.nil?
#
#   projects_delete_response = ProjectsDeleteResponse.from_json! "{…}"
#   puts projects_delete_response.data.first.error.nil?
#
#   project_delete_response = ProjectDeleteResponse.from_json! "{…}"
#   puts project_delete_response.error.nil?
#
#   response_for_fingerprint_response = ResponseForFingerprintResponse.from_json! "{…}"
#   puts response_for_fingerprint_response.data&.fingerprint
#
#   fingerprint_response = FingerprintResponse.from_json! "{…}"
#   puts fingerprint_response.fingerprint
#
#   response_for_sync_response = ResponseForSyncResponse.from_json! "{…}"
#   puts response_for_sync_response.data&.sends.first.access_count.even?
#
#   sync_response = SyncResponse.from_json! "{…}"
#   puts sync_response.sends.first.access_count.even?
#
#   profile_response = ProfileResponse.from_json! "{…}"
#   puts profile_response.organizations.first.id
#
#   profile_organization_response = ProfileOrganizationResponse.from_json! "{…}"
#   puts profile_organization_response.id
#
#   folder = Folder.from_json! "{…}"
#   puts folder.id.nil?
#
#   enc_string = EncString.from_json! "…"
#   puts enc_string
#
#   collection = Collection.from_json! "{…}"
#   puts collection.external_id.nil?
#
#   cipher = Cipher.from_json! "{…}"
#   puts cipher.collection_ids.first
#
#   cipher_type = CipherType.from_json! "…"
#   puts cipher_type == CipherType::Card
#
#   login = Login.from_json! "{…}"
#   puts login.autofill_on_page_load.nil?
#
#   login_uri = LoginURI.from_json! "{…}"
#   puts login_uri.match.nil?
#
#   uri_match_type = URIMatchType.from_json! "…"
#   puts uri_match_type == URIMatchType::Domain
#
#   identity = Identity.from_json! "{…}"
#   puts identity.address1.nil?
#
#   card = Card.from_json! "{…}"
#   puts card.brand.nil?
#
#   secure_note = SecureNote.from_json! "{…}"
#   puts secure_note.secure_note_type == SecureNoteType::Generic
#
#   secure_note_type = SecureNoteType.from_json! "…"
#   puts secure_note_type == SecureNoteType::Generic
#
#   cipher_reprompt_type = CipherRepromptType.from_json! "…"
#   puts cipher_reprompt_type == CipherRepromptType::None
#
#   local_data = LocalData.from_json! "{…}"
#   puts local_data.last_launched.nil?
#
#   attachment = Attachment.from_json! "{…}"
#   puts attachment.file_name.nil?
#
#   field = Field.from_json! "{…}"
#   puts field.linked_id.nil?
#
#   field_type = FieldType.from_json! "…"
#   puts field_type == FieldType::Boolean
#
#   linked_id_type = LinkedIDType.from_json! "…"
#   puts linked_id_type == LinkedIDType::Address1
#
#   login_linked_id_type = LoginLinkedIDType.from_json! "…"
#   puts login_linked_id_type == LoginLinkedIDType::Password
#
#   card_linked_id_type = CardLinkedIDType.from_json! "…"
#   puts card_linked_id_type == CardLinkedIDType::Brand
#
#   identity_linked_id_type = IdentityLinkedIDType.from_json! "…"
#   puts identity_linked_id_type == IdentityLinkedIDType::Address1
#
#   password_history = PasswordHistory.from_json! "{…}"
#   puts password_history.last_used_date
#
#   domain_response = DomainResponse.from_json! "{…}"
#   puts domain_response.global_equivalent_domains.first.domains.first
#
#   global_domains = GlobalDomains.from_json! "{…}"
#   puts global_domains.domains.first
#
#   policy = Policy.from_json! "{…}"
#   puts policy.data&["…"]
#
#   policy_type = PolicyType.from_json! "…"
#   puts policy_type == PolicyType::ActivateAutofill
#
#   send = Send.from_json! "{…}"
#   puts send.access_count.even?
#
#   send_type = SendType.from_json! "…"
#   puts send_type == SendType::File
#
#   send_file = SendFile.from_json! "{…}"
#   puts send_file.file_name
#
#   send_text = SendText.from_json! "{…}"
#   puts send_text.hidden
#
#   response_for_user_api_key_response = ResponseForUserAPIKeyResponse.from_json! "{…}"
#   puts response_for_user_api_key_response.data&.api_key
#
#   user_api_key_response = UserAPIKeyResponse.from_json! "{…}"
#   puts user_api_key_response.api_key
#
# If from_json! succeeds, the value returned matches the schema.

require 'json'
require 'dry-types'
require 'dry-struct'

module Types
  include Dry.Types(default: :nominal)

  Integer              = Strict::Integer
  Nil                  = Strict::Nil
  Bool                 = Strict::Bool
  Hash                 = Strict::Hash
  String               = Strict::String
  DeviceType           = Strict::String.enum("Android", "AndroidAmazon", "ChromeBrowser", "ChromeExtension", "EdgeBrowser", "EdgeExtension", "FirefoxBrowser", "FirefoxExtension", "IEBrowser", "iOS", "LinuxDesktop", "MacOsDesktop", "OperaBrowser", "OperaExtension", "SDK", "SafariBrowser", "SafariExtension", "UWP", "UnknownBrowser", "VivaldiBrowser", "VivaldiExtension", "WindowsDesktop")
  TwoFactorProvider    = Strict::String.enum("Authenticator", "Duo", "Email", "OrganizationDuo", "Remember", "U2f", "WebAuthn", "Yubikey")
  CipherType           = Strict::String.enum("Card", "Identity", "Login", "SecureNote")
  FieldType            = Strict::String.enum("Boolean", "Hidden", "Linked", "Text")
  LinkedIDType         = Strict::String.enum("Address1", "Address2", "Address3", "Brand", "CardholderName", "City", "Code", "Company", "Country", "Email", "ExpMonth", "ExpYear", "FirstName", "FullName", "LastName", "LicenseNumber", "MiddleName", "Number", "PassportNumber", "Password", "Phone", "PostalCode", "Ssn", "State", "Title", "Username")
  URIMatchType         = Strict::String.enum("domain", "exact", "host", "never", "regularExpression", "startsWith")
  CipherRepromptType   = Strict::String.enum("None", "Password")
  SecureNoteType       = Strict::String.enum("Generic")
  PolicyType           = Strict::String.enum("ActivateAutofill", "DisablePersonalVaultExport", "DisableSend", "MasterPassword", "MaximumVaultTimeout", "PasswordGenerator", "PersonalOwnership", "RequireSso", "ResetPassword", "SendOptions", "SingleOrg", "TwoFactorAuthentication")
  SendType             = Strict::String.enum("File", "Text")
  LoginLinkedIDType    = Strict::String.enum("Password", "Username")
  CardLinkedIDType     = Strict::String.enum("Brand", "CardholderName", "Code", "ExpMonth", "ExpYear", "Number")
  IdentityLinkedIDType = Strict::String.enum("Address1", "Address2", "Address3", "City", "Company", "Country", "Email", "FirstName", "FullName", "LastName", "LicenseNumber", "MiddleName", "PassportNumber", "Phone", "PostalCode", "Ssn", "State", "Title", "Username")
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
# ``` # use bitwarden::client::client_settings::{ClientSettings, DeviceType}; let settings
# = ClientSettings { identity_url: "https://identity.bitwarden.com".to_string(), api_url:
# "https://api.bitwarden.com".to_string(), user_agent: "Bitwarden Rust-SDK".to_string(),
# device_type: DeviceType::SDK, }; let default = ClientSettings::default(); ```
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

  attribute :state_file, Types::String.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      access_token: d.fetch("accessToken"),
      state_file:   d["stateFile"],
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "accessToken" => access_token,
      "stateFile"   => state_file,
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

class Argon2ID < Dry::Struct
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

class PBKDF2 < Dry::Struct
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
class Kdf < Dry::Struct
  attribute :p_bkdf2,   PBKDF2.optional
  attribute :argon2_id, Argon2ID.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      p_bkdf2:   d["pBKDF2"] ? PBKDF2.from_dynamic!(d["pBKDF2"]) : nil,
      argon2_id: d["argon2id"] ? Argon2ID.from_dynamic!(d["argon2id"]) : nil,
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
  attribute :kdf, Kdf

  # Bitwarden account master password
  attribute :password, Types::String

  attribute :two_factor, TwoFactorRequest.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      email:      d.fetch("email"),
      kdf:        Kdf.from_dynamic!(d.fetch("kdf")),
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
# Returns: [ProjectResponse](bitwarden::secrets_manager::projects::ProjectResponse)
#
# > Requires Authentication > Requires using an Access Token for login or calling Sync at
# least once Creates a new project in the provided organization using the given data
#
# Returns: [ProjectResponse](bitwarden::secrets_manager::projects::ProjectResponse)
#
# > Requires Authentication > Requires using an Access Token for login or calling Sync at
# least once Lists all projects of the given organization
#
# Returns: [ProjectsResponse](bitwarden::secrets_manager::projects::ProjectsResponse)
#
# > Requires Authentication > Requires using an Access Token for login or calling Sync at
# least once Updates an existing project with the provided ID using the given data
#
# Returns: [ProjectResponse](bitwarden::secrets_manager::projects::ProjectResponse)
#
# > Requires Authentication > Requires using an Access Token for login or calling Sync at
# least once Deletes all the projects whose IDs match the provided ones
#
# Returns:
# [ProjectsDeleteResponse](bitwarden::secrets_manager::projects::ProjectsDeleteResponse)
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
# Returns: [SecretResponse](bitwarden::secrets_manager::secrets::SecretResponse)
#
# > Requires Authentication > Requires using an Access Token for login or calling Sync at
# least once Retrieve secrets by the provided identifiers
#
# Returns: [SecretsResponse](bitwarden::secrets_manager::secrets::SecretsResponse)
#
# > Requires Authentication > Requires using an Access Token for login or calling Sync at
# least once Creates a new secret in the provided organization using the given data
#
# Returns: [SecretResponse](bitwarden::secrets_manager::secrets::SecretResponse)
#
# > Requires Authentication > Requires using an Access Token for login or calling Sync at
# least once Lists all secret identifiers of the given organization, to then retrieve each
# secret, use `CreateSecret`
#
# Returns:
# [SecretIdentifiersResponse](bitwarden::secrets_manager::secrets::SecretIdentifiersResponse)
#
# > Requires Authentication > Requires using an Access Token for login or calling Sync at
# least once Updates an existing secret with the provided ID using the given data
#
# Returns: [SecretResponse](bitwarden::secrets_manager::secrets::SecretResponse)
#
# > Requires Authentication > Requires using an Access Token for login or calling Sync at
# least once Deletes all the secrets whose IDs match the provided ones
#
# Returns:
# [SecretsDeleteResponse](bitwarden::secrets_manager::secrets::SecretsDeleteResponse)
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
# Returns: [PasswordLoginResponse](bitwarden::auth::login::PasswordLoginResponse)
#
# Login with API Key
#
# This command is for initiating an authentication handshake with Bitwarden.
#
# Returns: [ApiKeyLoginResponse](bitwarden::auth::login::ApiKeyLoginResponse)
#
# Login with Secrets Manager Access Token
#
# This command is for initiating an authentication handshake with Bitwarden.
#
# Returns: [ApiKeyLoginResponse](bitwarden::auth::login::ApiKeyLoginResponse)
#
# > Requires Authentication Get the API key of the currently authenticated user
#
# Returns: [UserApiKeyResponse](bitwarden::platform::UserApiKeyResponse)
#
# Get the user's passphrase
#
# Returns: String
#
# > Requires Authentication Retrieve all user data, ciphers and organizations the user is a
# part of
#
# Returns: [SyncResponse](bitwarden::platform::SyncResponse)
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

class Authenticator < Dry::Struct

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

class Duo < Dry::Struct
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

class Email < Dry::Struct

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

class Remember < Dry::Struct

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

class WebAuthn < Dry::Struct

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

class YubiKey < Dry::Struct

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

class TwoFactorProviders < Dry::Struct
  attribute :authenticator, Authenticator.optional.optional

  # Duo-backed 2fa
  attribute :duo, Duo.optional.optional

  # Email 2fa
  attribute :email, Email.optional.optional

  # Duo-backed 2fa operated by an organization the user is a member of
  attribute :organization_duo, Duo.optional.optional

  # Presence indicates the user has stored this device as bypassing 2fa
  attribute :remember, Remember.optional.optional

  # WebAuthn-backed 2fa
  attribute :web_authn, WebAuthn.optional.optional

  # Yubikey-backed 2fa
  attribute :yubi_key, YubiKey.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      authenticator:    d["authenticator"] ? Authenticator.from_dynamic!(d["authenticator"]) : nil,
      duo:              d["duo"] ? Duo.from_dynamic!(d["duo"]) : nil,
      email:            d["email"] ? Email.from_dynamic!(d["email"]) : nil,
      organization_duo: d["organizationDuo"] ? Duo.from_dynamic!(d["organizationDuo"]) : nil,
      remember:         d["remember"] ? Remember.from_dynamic!(d["remember"]) : nil,
      web_authn:        d["webAuthn"] ? WebAuthn.from_dynamic!(d["webAuthn"]) : nil,
      yubi_key:         d["yubiKey"] ? YubiKey.from_dynamic!(d["yubiKey"]) : nil,
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

  attribute :two_factor, TwoFactorProviders.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      authenticated:         d.fetch("authenticated"),
      force_password_reset:  d.fetch("forcePasswordReset"),
      reset_master_password: d.fetch("resetMasterPassword"),
      two_factor:            d["twoFactor"] ? TwoFactorProviders.from_dynamic!(d["twoFactor"]) : nil,
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
  attribute :two_factor, TwoFactorProviders.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      authenticated:         d.fetch("authenticated"),
      captcha:               d["captcha"] ? CAPTCHAResponse.from_dynamic!(d["captcha"]) : nil,
      force_password_reset:  d.fetch("forcePasswordReset"),
      reset_master_password: d.fetch("resetMasterPassword"),
      two_factor:            d["twoFactor"] ? TwoFactorProviders.from_dynamic!(d["twoFactor"]) : nil,
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

class AccessTokenLoginResponse < Dry::Struct
  attribute :authenticated, Types::Bool

  # Whether or not the user is required to update their master password
  attribute :force_password_reset, Types::Bool

  # TODO: What does this do?
  attribute :reset_master_password, Types::Bool

  attribute :two_factor, TwoFactorProviders.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      authenticated:         d.fetch("authenticated"),
      force_password_reset:  d.fetch("forcePasswordReset"),
      reset_master_password: d.fetch("resetMasterPassword"),
      two_factor:            d["twoFactor"] ? TwoFactorProviders.from_dynamic!(d["twoFactor"]) : nil,
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

class ResponseForAccessTokenLoginResponse < Dry::Struct

  # The response data. Populated if `success` is true.
  attribute :data, AccessTokenLoginResponse.optional.optional

  # A message for any error that may occur. Populated if `success` is false.
  attribute :error_message, Types::String.optional.optional

  # Whether or not the SDK request succeeded.
  attribute :success, Types::Bool

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data:          d["data"] ? AccessTokenLoginResponse.from_dynamic!(d["data"]) : nil,
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

class SecretsResponse < Dry::Struct
  attribute :data, Types.Array(SecretResponse)

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data: d.fetch("data").map { |x| SecretResponse.from_dynamic!(x) },
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

class ProjectsResponse < Dry::Struct
  attribute :data, Types.Array(ProjectResponse)

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data: d.fetch("data").map { |x| ProjectResponse.from_dynamic!(x) },
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
  attribute :attachments,    Types.Array(Attachment).optional.optional
  attribute :card,           Card.optional.optional
  attribute :collection_ids, Types.Array(Types::String)
  attribute :creation_date,  Types::String
  attribute :deleted_date,   Types::String.optional.optional
  attribute :edit,           Types::Bool
  attribute :favorite,       Types::Bool
  attribute :fields,         Types.Array(Field).optional.optional
  attribute :folder_id,      Types::String.optional.optional
  attribute :id,             Types::String.optional.optional
  attribute :identity,       Identity.optional.optional

  # More recent ciphers uses individual encryption keys to encrypt the other fields of the
  # Cipher.
  attribute :key, Types::String.optional.optional

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
      key:                   d["key"],
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
      "key"                 => key,
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

class Collection < Dry::Struct
  attribute :external_id,     Types::String.optional.optional
  attribute :hide_passwords,  Types::Bool
  attribute :id,              Types::String.optional.optional
  attribute :collection_name, Types::String
  attribute :organization_id, Types::String
  attribute :read_only,       Types::Bool

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      external_id:     d["externalId"],
      hide_passwords:  d.fetch("hidePasswords"),
      id:              d["id"],
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

class GlobalDomains < Dry::Struct
  attribute :domains,             Types.Array(Types::String)
  attribute :excluded,            Types::Bool
  attribute :global_domains_type, Types::Integer

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      domains:             d.fetch("domains"),
      excluded:            d.fetch("excluded"),
      global_domains_type: d.fetch("type"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "domains"  => domains,
      "excluded" => excluded,
      "type"     => global_domains_type,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class DomainResponse < Dry::Struct
  attribute :equivalent_domains,        Types.Array(Types.Array(Types::String))
  attribute :global_equivalent_domains, Types.Array(GlobalDomains)

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      equivalent_domains:        d.fetch("equivalentDomains"),
      global_equivalent_domains: d.fetch("globalEquivalentDomains").map { |x| GlobalDomains.from_dynamic!(x) },
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "equivalentDomains"       => equivalent_domains,
      "globalEquivalentDomains" => global_equivalent_domains.map { |x| x.to_dynamic },
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class Folder < Dry::Struct
  attribute :id,            Types::String.optional.optional
  attribute :folder_name,   Types::String
  attribute :revision_date, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      id:            d["id"],
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

module PolicyType
  ActivateAutofill           = "ActivateAutofill"
  DisablePersonalVaultExport = "DisablePersonalVaultExport"
  DisableSend                = "DisableSend"
  MasterPassword             = "MasterPassword"
  MaximumVaultTimeout        = "MaximumVaultTimeout"
  PasswordGenerator          = "PasswordGenerator"
  PersonalOwnership          = "PersonalOwnership"
  RequireSso                 = "RequireSso"
  ResetPassword              = "ResetPassword"
  SendOptions                = "SendOptions"
  SingleOrg                  = "SingleOrg"
  TwoFactorAuthentication    = "TwoFactorAuthentication"
end

class Policy < Dry::Struct
  attribute :data,            Types::Hash.meta(of: Types::Any).optional.optional
  attribute :enabled,         Types::Bool
  attribute :id,              Types::String
  attribute :organization_id, Types::String
  attribute :policy_type,     Types::PolicyType

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data:            Types::Hash.optional[d["data"]]&.map { |k, v| [k, Types::Any[v]] }&.to_h,
      enabled:         d.fetch("enabled"),
      id:              d.fetch("id"),
      organization_id: d.fetch("organization_id"),
      policy_type:     d.fetch("type"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "data"            => data,
      "enabled"         => enabled,
      "id"              => id,
      "organization_id" => organization_id,
      "type"            => policy_type,
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

class SendFile < Dry::Struct
  attribute :file_name, Types::String
  attribute :id,        Types::String.optional.optional
  attribute :size,      Types::String.optional.optional

  # Readable size, ex: "4.2 KB" or "1.43 GB"
  attribute :size_name, Types::String.optional.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      file_name: d.fetch("fileName"),
      id:        d["id"],
      size:      d["size"],
      size_name: d["sizeName"],
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
  attribute :access_id,        Types::String.optional.optional
  attribute :deletion_date,    Types::String
  attribute :disabled,         Types::Bool
  attribute :expiration_date,  Types::String.optional.optional
  attribute :file,             SendFile.optional.optional
  attribute :hide_email,       Types::Bool
  attribute :id,               Types::String.optional.optional
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
      access_id:        d["accessId"],
      deletion_date:    d.fetch("deletionDate"),
      disabled:         d.fetch("disabled"),
      expiration_date:  d["expirationDate"],
      file:             d["file"] ? SendFile.from_dynamic!(d["file"]) : nil,
      hide_email:       d.fetch("hideEmail"),
      id:               d["id"],
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

class SyncResponse < Dry::Struct

  # List of ciphers accessible by the user
  attribute :ciphers, Types.Array(Cipher)

  attribute :collections, Types.Array(Collection)
  attribute :domains,     DomainResponse.optional.optional
  attribute :folders,     Types.Array(Folder)
  attribute :policies,    Types.Array(Policy)

  # Data about the user, including their encryption keys and the organizations they are a
  # part of
  attribute :profile, ProfileResponse

  attribute :sends, Types.Array(Send)

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      ciphers:     d.fetch("ciphers").map { |x| Cipher.from_dynamic!(x) },
      collections: d.fetch("collections").map { |x| Collection.from_dynamic!(x) },
      domains:     d["domains"] ? DomainResponse.from_dynamic!(d["domains"]) : nil,
      folders:     d.fetch("folders").map { |x| Folder.from_dynamic!(x) },
      policies:    d.fetch("policies").map { |x| Policy.from_dynamic!(x) },
      profile:     ProfileResponse.from_dynamic!(d.fetch("profile")),
      sends:       d.fetch("sends").map { |x| Send.from_dynamic!(x) },
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "ciphers"     => ciphers.map { |x| x.to_dynamic },
      "collections" => collections.map { |x| x.to_dynamic },
      "domains"     => domains&.to_dynamic,
      "folders"     => folders.map { |x| x.to_dynamic },
      "policies"    => policies.map { |x| x.to_dynamic },
      "profile"     => profile.to_dynamic,
      "sends"       => sends.map { |x| x.to_dynamic },
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

module LoginLinkedIDType
  Password = "Password"
  Username = "Username"
end

module CardLinkedIDType
  Brand          = "Brand"
  CardholderName = "CardholderName"
  Code           = "Code"
  ExpMonth       = "ExpMonth"
  ExpYear        = "ExpYear"
  Number         = "Number"
end

module IdentityLinkedIDType
  Address1       = "Address1"
  Address2       = "Address2"
  Address3       = "Address3"
  City           = "City"
  Company        = "Company"
  Country        = "Country"
  Email          = "Email"
  FirstName      = "FirstName"
  FullName       = "FullName"
  LastName       = "LastName"
  LicenseNumber  = "LicenseNumber"
  MiddleName     = "MiddleName"
  PassportNumber = "PassportNumber"
  Phone          = "Phone"
  PostalCode     = "PostalCode"
  Ssn            = "Ssn"
  State          = "State"
  Title          = "Title"
  Username       = "Username"
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

class EncString
  def self.from_json!(json)
    JSON.parse(json, quirks_mode: true)
  end
end

