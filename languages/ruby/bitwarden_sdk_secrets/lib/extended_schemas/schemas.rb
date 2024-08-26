
module BitwardenSDKSecrets
    class SelectiveCommand < Command
      attribute :password_login,     PasswordLoginRequest.optional.default(nil)
      attribute :api_key_login,      APIKeyLoginRequest.optional.default(nil)
      attribute :login_access_token, AccessTokenLoginRequest.optional.default(nil)
      attribute :get_user_api_key,   SecretVerificationRequest.optional.default(nil)
      attribute :fingerprint,        FingerprintRequest.optional.default(nil)
      attribute :sync,               SyncRequest.optional.default(nil)
      attribute :secrets,            SecretsCommand.optional.default(nil)
      attribute :projects,           ProjectsCommand.optional.default(nil)

      def to_dynamic
        {
          "passwordLogin"    => password_login&.to_dynamic,
          "apiKeyLogin"      => api_key_login&.to_dynamic,
          "loginAccessToken" => login_access_token&.to_dynamic,
          "getUserApiKey"    => get_user_api_key&.to_dynamic,
          "fingerprint"      => fingerprint&.to_dynamic,
          "sync"             => sync&.to_dynamic,
          "secrets"          => secrets&.to_dynamic,
          "projects"         => projects&.to_dynamic,
        }.compact
      end
    end

    class SelectiveProjectsCommand < ProjectsCommand
      attribute :get,    ProjectGetRequest.optional.default(nil)
      attribute :create, ProjectCreateRequest.optional.default(nil)
      attribute :list,   ProjectsListRequest.optional.default(nil)
      attribute :update, ProjectPutRequest.optional.default(nil)
      attribute :delete, ProjectsDeleteRequest.optional.default(nil)

      def to_dynamic
        {
          "get"    => get&.to_dynamic,
          "create" => create&.to_dynamic,
          "list"   => list&.to_dynamic,
          "update" => update&.to_dynamic,
          "delete" => delete&.to_dynamic,
        }.compact
      end
    end

    class SelectiveSecretsCommand < SecretsCommand
      attribute :get,        SecretGetRequest.optional.default(nil)
      attribute :get_by_ids, SecretsGetRequest.optional.default(nil)
      attribute :create,     SecretCreateRequest.optional.default(nil)
      attribute :list,       SecretIdentifiersRequest.optional.default(nil)
      attribute :update,     SecretPutRequest.optional.default(nil)
      attribute :delete,     SecretsDeleteRequest.optional.default(nil)

      def to_dynamic
        {
          "get"      => get&.to_dynamic,
          "getByIds" => get_by_ids&.to_dynamic,
          "create"   => create&.to_dynamic,
          "list"     => list&.to_dynamic,
          "update"   => update&.to_dynamic,
          "delete"   => delete&.to_dynamic,
        }.compact
      end
    end
end
