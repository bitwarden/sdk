require "json"
require "dry-types"

require_relative "schemas"
require_relative "extended_schemas/schemas"
require_relative "command_runner"
require_relative "bitwarden_lib"
require_relative "projects"
require_relative "secrets"


class BitwardenClient
  attr_reader :bitwarden, :project_client, :secrets_client

  def initialize
    api_url = ENV["BITWARDEN_API_URL"] || "https://api.bitwarden.com"
    device_type = ENV["BITWARDEN_DEVICE_TYPE"] || "MacOsDesktop"
    identity_url = ENV["BITWARDEN_IDENTITY_URL"] || "https://identity.bitwarden.com"
    user_agent = ENV["BITWARDEN_USER_AGENT"] || "SDK"

    client_settings = ClientSettings.new(
      api_url: api_url,
      device_type: device_type,
      identity_url: identity_url,
      user_agent: user_agent
    )

    @bitwarden = BitwardenLib
    @handle = @bitwarden.init(client_settings.to_json)
    @command_runner = CommandRunner.new(@bitwarden, @handle)
    @project_client = ProjectsClient.new(@command_runner)
    @secrets_client = SecretsClient.new(@command_runner)
  end

  def authorize(access_token)
    access_token_request = AccessTokenLoginRequest.new(access_token: access_token)
    @command_runner.run( SelectiveCommand.new(access_token_login: access_token_request))
  end

  def free_mem
    @bitwarden.free_mem(@handle)
  end
end


b = BitwardenClient.new
c = b.authorize("0.fa60cd1f-f6d8-48d5-81e5-b07e00f98cf0.uUanL2XzWPIclcao1PfJh7O0R9ixs6:+EW/yAt6u70vQTgs+zL5fA==")
puts c

# GET project
# project = b.project_client.get("b23818dd-827b-4a22-b97a-b07e010ae9d4")
# puts project.to_json
# CREATE
# project = b.project_client.create_project("test_project_22", "5688da1f-cc25-41d7-bb9f-b0740144ef1d")
# puts project.to_json

# c = b.authorize(SelectiveProjectsCommand.new(get: ProjectGetRequest.new(id: "b23818dd-827b-4a22-b97a-b07e010ae9d4")))
# puts c.to_json
# LIST projects
# project = b.project_client.list_projects("5688da1f-cc25-41d7-bb9f-b0740144ef1d")
# puts project.to_json
# UPDATE projects
# project = b.project_client.update_project("ef9d3d37-f0dc-4b21-a842-b0810129bf02", "test_project_x", "5688da1f-cc25-41d7-bb9f-b0740144ef1d")
# puts project.to_json
# DELETE
# project = b.project_client.delete_projects(["13a015aa-e3dc-4854-875a-b08101512d2f"])
# puts project.to_json

# create secret
# s = b.secrets_client.create("this", "hola!", "5688da1f-cc25-41d7-bb9f-b0740144ef1d", ["ef9d3d37-f0dc-4b21-a842-b0810129bf02"], "blah")
# puts s.to_json

# get secret
s = b.secrets_client.get("fa175a5b-da76-48c3-b44b-b0810151638c")
puts s

# get by ids - TODO fix
# s = b.secrets_client.get_by_ids(["e8561721-0455-438c-bbbe-b0810152f534"])
# puts s.to_json

# list
# s = b.secrets_client.list("5688da1f-cc25-41d7-bb9f-b0740144ef1d")
# puts s.to_json

# delete
# s = b.secrets_client.delete_secret(["b03cf64b-e894-4675-9f59-b0810152abe6"])
# puts s

# update
# s = b.secrets_client.update("683c25f3-a463-49ba-bed4-b0810134a7b1", "hello", "meow", "5688da1f-cc25-41d7-bb9f-b0740144ef1d", ["4647aede-33f1-4ad1-a258-b07a014a48a7"], "meow")
# puts s.to_json
