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
  class BitwardenClient
    attr_reader :bitwarden, :project_client, :secrets_client

    def initialize
      client_settings = ClientSettings.new(
        api_url: ENV['BITWARDEN_API_URL'] || 'https://api.bitwarden.com',
        identity_url: ENV['BITWARDEN_IDENTITY_URL'] || 'https://identity.bitwarden.com/connect/token',
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
