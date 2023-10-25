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

class BitwardenClient
  attr_reader :bitwarden, :project_client, :secrets_client

  def initialize
    client_settings = ClientSettings.new(
      api_url: ENV['BITWARDEN_API_URL'],
      identity_url: ENV['BITWARDEN_IDENTITY_URL'],
      user_agent: 'Bitwarden RUBY-SDK',
      device_type: nil
    )

    @bitwarden = BitwardenLib
    @handle = @bitwarden.init(client_settings.to_json)
    @command_runner = CommandRunner.new(@bitwarden, @handle)
    @project_client = ProjectsClient.new(@command_runner)
    @secrets_client = SecretsClient.new(@command_runner)
  end

  def authorize(access_token)
    access_token_request = AccessTokenLoginRequest.new(access_token: access_token)
    @command_runner.run(SelectiveCommand.new(access_token_login: access_token_request))
  end

  def free_mem
    @bitwarden.free_mem(@handle)
  end
end



token = "0.de9a0406-fd0f-423e-a81a-b0a600cd2fc3.nquKtsPdgmpyCwF2MK6iA1CHT4iFTX:MedT1fedtDGqn33XsBPlhw=="

bw_client = BitwardenClient.new
response = bw_client.authorize(token)
puts response

# GET project
# response = bw_client.project_client.get("b23818dd-827b-4a22-b97a-b07e010ae9d4")
# puts response

# CREATE project
# response = bw_client.project_client.create_project("test_project_1", "5688da1f-cc25-41d7-bb9f-b0740144ef1d")
# puts response

# LIST projects  # TODO FIX
# response = bw_client.project_client.list_projects("5688da1f-cc25-41d7-bb9f-b0740144ef1d")
# puts response

# UPDATE projects
# response = bw_client.project_client.update_project("01f4bdc1-19ea-4f6b-b280-b0a600ce20cb", "test_project_1", "5688da1f-cc25-41d7-bb9f-b0740144ef1d")
# puts response

# DELETE
# response = bw_client.project_client.delete_projects(["01f4bdc1-19ea-4f6b-b280-b0a600ce20cb"])
# puts response

# CREATE secret checked TODO checked
# response = bw_client.secrets_client.create("AWS-SES", "important!", "5688da1f-cc25-41d7-bb9f-b0740144ef1d", ["b23818dd-827b-4a22-b97a-b07e010ae9d4"], "8t27.dfj;")
# puts response

# GET secret checked TODO checked
# response = bw_client.secrets_client.get("75d3a7ff-30ed-433a-91aa-b099016e4833")
# puts response

# GET secret by ids TODO checked
# response = bw_client.secrets_client.get_by_ids(["2a41fdf3-3787-4cb9-a781-b0a6017e2064"])
# puts response

# LIST secrets checked TODO checked
# response = bw_client.secrets_client.list("5688da1f-cc25-41d7-bb9f-b0740144ef1d")
# puts response

# DELETE secret checked TODO checked
# response = bw_client.secrets_client.delete_secret(["34df1b56-bc1d-4011-bdb0-b0a6017db6b0"])
# puts response

# UPDATE secret checked TODO checked
# response = bw_client.secrets_client.update("34df1b56-bc1d-4011-bdb0-b0a6017db6b0", "AWS-KEY", "very important!","5688da1f-cc25-41d7-bb9f-b0740144ef1d", ["b23818dd-827b-4a22-b97a-b07e010ae9d4"], "7I.ert10AjK")
# puts response
