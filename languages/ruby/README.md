# Bitwarden Secrets Manager SDK

Ruby bindings for interacting with the [Bitwarden Secrets Manager]. This is a beta release and might
be missing some functionality.

## Installation

Requirements: Ruby >= 3.0

Install gem: `gem install bitwarden-sdk-secrets`

Import it: `require 'bitwarden-sdk-secrets'`

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
response = bw_client.auth.login_access_token(token, state_file)
puts response
```

After successful authorization you can interact with client to manage your projects and secrets.

```ruby

# CREATE project
project_name = 'Test project 1'
response = bw_client.projects.create(organization_id, project_name)
puts response
project_id = response['id']

# GET project
response = bw_client.projects.get(project_id)
puts response

# LIST projects
response = bw_client.projects.list(organization_id)
puts response

# UPDATE projects
name = 'Updated test project 1'
response = bw_client.projects.update(organization_id, project_id, name)
puts response

# DELETE project
response = bw_client.projects.delete_projects([project_id])
puts response
```

Similarly, you interact with secrets:

```ruby
# CREATE secret
key = 'AWS-SES'
note = 'Private account'
value = '8t27.dfj;'
response = bw_client.secrets.create(organization_id, key, value, note, [project_id])
puts response
secret_id = response['id']

# GET secret
response = bw_client.secrets.get(secret_id)
puts response

# GET secret by ids
response = bw_client.secrets.get_by_ids([secret_id])
puts response

# LIST secrets
response = bw_client.secrets.list(organization_id)
puts response

# SYNC secrets
response = bw_client.secrets.sync(organization_id, nil)
last_synced_date = Time.now.utc.strftime('%Y-%m-%dT%H:%M:%S.%6NZ')
puts response

response = bw_client.secrets.sync(organization_id, last_synced_date)
puts response

# UPDATE secret
note = 'updated password'
value = '7I.ert10AjK'
response = bw_client.secrets.update(organization_id, secret_id, key, value, note, [project_id])
puts response

# DELETE secret
response = bw_client.secrets.delete([secret_id])
puts response
```

## Development

Prerequisites:

- Ruby >= 3.0 installed
- Generate schemas `npm run schemas`

```bash
# Navigate to the ruby language folder
cd languages/ruby

# Make the binary folder if it doesn't exist already
mkdir -p ./bitwarden_sdk_secrets/lib/macos-arm64

# Build and copy the bitwarden-c library
cargo build --package bitwarden-c
cp ../../target/debug/libbitwarden_c.dylib ./bitwarden_sdk_secrets/lib/macos-arm64/libbitwarden_c.dylib

# Install ruby dependencies
cd ./bitwarden_sdk_secrets
bundle install

# Install the gem
bundle exec rake install

## Run example tests
cd ..
export ACCESS_TOKEN=""
export ORGANIZATION_ID=""

export API_URL=http://localhost:4000
export IDENTITY_URL=http://localhost:33656
ruby examples/example.rb
```

[Bitwarden Secrets Manager]: https://bitwarden.com/products/secrets-manager/
