# frozen_string_literal: true
require_relative 'bitwarden_error'

module BitwardenSDKSecrets
  class AuthClient
    def initialize(command_runner)
      @command_runner = command_runner
    end

    def login_access_token(access_token, state_file = nil)
      access_token_request = AccessTokenLoginRequest.new(access_token: access_token, state_file: state_file)
      @command_runner.run(SelectiveCommand.new(login_access_token: access_token_request))
      nil
    end
  end
end
