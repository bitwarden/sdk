# frozen_string_literal: true

module BitwardenSDKSecrets
  class CommandRunner
    def initialize(bitwarden_sdk, handle)
      @bitwarden_sdk = bitwarden_sdk
      @handle = handle
    end

    # @param [Dry-Struct] cmd
    def run(cmd)
      @bitwarden_sdk.run_command(cmd.to_json, @handle)
    end
  end
end
