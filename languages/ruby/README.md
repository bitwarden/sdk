# Bitwarden Secrets Manager SDK

Ruby bindings for interacting with the [Bitwarden Secrets Manager]. This is a beta release and might
be missing some functionality.

## Installation

Requirements: Ruby >= 3.0

Install gem: `gem install bitwarden-sdk-secrets`

Import it: require 'bitwarden-sdk-secrets'

## Usage

To interact with client first you need to obtain access token from Bitwarden. Client will be
initialized with default client settings if they are not provided via env variables.

```ruby
require 'bitwarden-sdk-secrets'

# then you can initialize BitwardenSettings:
bitwarden_settings = BitwardenSDK::BitwardenSettings.new(
  'https://api.bitwarden.com',
  'https://identity.bitwarden.com'
)

# By passing these setting you can initialize BitwardenClient

bw_client = BitwardenSDK::BitwardenClient.new(bitwarden_settings)
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

## Development

```bash
cargo build --package bitwarden-c
cp ../../target/debug/libbitwarden_c.dylib ./bitwarden_sdk_secrets/lib/macos-arm64/libbitwarden_c.dylib

cd ./bitwarden_sdk_secrets
gem build bitwarden-sdk-secrets.gemspec
gem install ./bitwarden-sdk-secrets-0.0.0.gem

## Run example tests
cd ..
export ACCESS_TOKEN=""
export ORGANIZATION_ID=""

export API_URL=https://localhost:8080/api
export IDENTITY_URL=https://localhost:8080/identity
ruby examples/example.rb
```

[Bitwarden Secrets Manager]: https://bitwarden.com/products/secrets-manager/
