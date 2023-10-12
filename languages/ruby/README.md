# Bitwarden Secrets Manager SDK

Ruby bindings for interacting with the [Bitwarden Secrets Manager]. This is a beta release and might be missing some functionality.

## Installation

Requirements: Ruby >= 2.7

Install gem: `gem install ruby-bitwarden-sdk`


## Usage

To interact with client first you need to obtain access token from Bitwarden.
Client will be initialized with default client settings if they are not provided
via env variables.

```angular2html
    api_url = ENV['BITWARDEN_API_URL'] || 'https://api.bitwarden.com'
    device_type = ENV['BITWARDEN_DEVICE_TYPE'] || 'MacOsDesktop'
    identity_url = ENV['BITWARDEN_IDENTITY_URL'] || 'https://identity.bitwarden.com'
    user_agent = ENV['BITWARDEN_USER_AGENT'] || 'SDK'
```

Authorization can be performed using access token like so:
```angular2html
client = BitwardenClient.new
client.authorize("<<YOUR ACCESS TOKEN HERE>>")
```

After successful authorization you can interact with client to manage your projects and secrets.
```angular2html
# get project
project = client.project_client.get("b23818dd-827b-4a22-b97a-b07e010ae9d4")

# Create project
project = client.project_client.create_project("new_project", "5688da1f-cc25-41d7-bb9f-b0740144ef1d")

# list projects
project s= client.project_client.list_projects("5688da1f-cc25-41d7-bb9f-b0740144ef1d")

# update project
project = client.project_client.update_project("ef9d3d37-f0dc-4b21-a842-b0810129bf02", "test_project_x", "5688da1f-cc25-41d7-bb9f-b0740144ef1d")

# delete projects
response = client.project_client.delete_projects(["13a015aa-e3dc-4854-875a-b08101512d2f"])
```

Similarly, you interact with secrets:
```angular2html
# get secret
secret = client.secrets_client.get("fa175a5b-da76-48c3-b44b-b0810151638c")

# get by ids
secrets = client.secrets_client.get_by_ids(["e8561721-0455-438c-bbbe-b0810152f534"])

# list
secrets = client.secrets_client.list("5688da1f-cc25-41d7-bb9f-b0740144ef1d")

# delete
result = client.secrets_client.delete_secret(["b03cf64b-e894-4675-9f59-b0810152abe6", "e8561721-0455-438c-bbbe-b0810152f534"])

# create
secret = client.secrets_client.create("this", "hola!", "5688da1f-cc25-41d7-bb9f-b0740144ef1d", ["ef9d3d37-f0dc-4b21-a842-b0810129bf02"], "blah")

# update
secret = client.secrets_client.update("683c25f3-a463-49ba-bed4-b0810134a7b1", "Title", "my pass", "5688da1f-cc25-41d7-bb9f-b0740144ef1d", ["4647aede-33f1-4ad1-a258-b07a014a48a7"], "supersecret77")
```


``

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/MaliRobot/ruby-sdk.
