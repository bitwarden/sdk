# Bitwarden Secrets Manager SDK

Ruby bindings for interacting with the [Bitwarden Secrets Manager]. This is a beta release and might be missing some functionality.

## Installation

Requirements: Ruby >= 3.0

Install gem: `gem install bitwarden-sdk`

Import it: require 'bitwarden-sdk'


## Usage

To interact with client first you need to obtain access token from Bitwarden.
The you need to instantate ClientSettings object with api_url, identity_url and user_agent.
You can now initialize BitwardenSDK by passing client settings.
Finally, authorize by passing access token to access_token_login method.

```ruby
require 'bitwarden-sdk'

api_url = ENV['BITWARDEN_API_URL'] || 'https://api.bitwarden.com'
identity_url = ENV['BITWARDEN_IDENTITY_URL'] || 'https://identity.bitwarden.com'
user_agent = ENV['BITWARDEN_USER_AGENT'] || 'SDK'

client_settings = ClientSettings.new({'api_url': api_url, 'identity_url': identity_url, device_type: user_agent, user_agent: nil})

bw_client = BitwardenSDK::BitwardenClient.new(client_settings)
response = bw_client.access_token_login(token)
puts response
```

After successful authorization you can interact with client to manage your projects and secrets.
```ruby

# CREATE project
project_name = 'Test project 1'
response = bw_client.project_client.create_project(project_name, organization_id)
puts response
project_id = response['id']

# GET project
response = bw_client.project_client.get(project_id)
puts response

# LIST projects
response = bw_client.project_client.list_projects(organization_id)
puts response

# UPDATE projects
name = 'Updated test project 1'
response = bw_client.project_client.update_project(project_id, name, organization_id)
puts response

# DELETE project
response = bw_client.project_client.delete_projects([project_id])
puts response
```

Similarly, you interact with secrets:
```ruby
# CREATE secret
key = 'AWS-SES'
note = 'Private account'
value = '8t27.dfj;'
response = bw_client.secrets_client.create(key, note, organization_id, [project_id], value)
puts response
secret_id = response['id']

# GET secret
response = bw_client.secrets_client.get(secret_id)
puts response

# GET secret by ids
response = bw_client.secrets_client.get_by_ids([secret_id])
puts response

# LIST secrets
response = bw_client.secrets_client.list(organization_id)
puts response

# UPDATE secret
note = 'updated password'
value = '7I.ert10AjK'
response = bw_client.secrets_client.update(secret_id, key, note,organization_id, [project_id], value)
puts response

# DELETE secret
response = bw_client.secrets_client.delete_secret([secret_id])
puts response
```
[Bitwarden Secrets Manager]: https://bitwarden.com/products/secrets-manager/
