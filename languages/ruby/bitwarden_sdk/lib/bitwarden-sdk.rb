# frozen_string_literal: true

require 'json'
require 'dry-types'

require_relative 'schemas'
require_relative 'extended_schemas/schemas'
require_relative 'command_runner'
require_relative 'bitwarden_lib'
require_relative 'bitwarden_error'
require_relative 'projects'
require_relative 'secrets'

module BitwardenSDK
  class BitwardenSettings
    attr_accessor :api_url, :identity_url

    def initialize(api_url, identity_url)
      @api_url = api_url
      @identity_url = identity_url
    end
  end

  class BitwardenClient
    attr_reader :bitwarden, :project_client, :secrets_client

    def initialize(bitwarden_settings)
      client_settings = ClientSettings.new(
        api_url: bitwarden_settings.api_url,
        identity_url: bitwarden_settings.identity_url,
        user_agent: 'Bitwarden RUBY-SDK',
        device_type: nil
      )

      @bitwarden = BitwardenLib
      @handle = @bitwarden.init(client_settings.to_json)
      @command_runner = CommandRunner.new(@bitwarden, @handle)
      @project_client = ProjectsClient.new(@command_runner)
      @secrets_client = SecretsClient.new(@command_runner)
    end

    def authorize(access_token_login)
      access_token_request = AccessTokenLoginRequest.new(access_token: access_token_login)
      @command_runner.run(SelectiveCommand.new(access_token_login: access_token_request))
    end

    def free_mem
      @bitwarden.free_mem(@handle)
    end
  end
end
