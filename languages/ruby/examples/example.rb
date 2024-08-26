# NOTE - for example purpose only - import gem instead
require 'bitwarden-sdk-secrets'

token = ENV['ACCESS_TOKEN']
organization_id = ENV['ORGANIZATION_ID']
state_file = ENV['STATE_FILE']

# Configuring the URLS is optional, set them to nil to use the default values
api_url = ENV['API_URL']
identity_url = ENV['IDENTITY_URL']

bitwarden_settings = BitwardenSDKSecrets::BitwardenSettings.new(api_url, identity_url)

bw_client = BitwardenSDKSecrets::BitwardenClient.new(bitwarden_settings)
response = bw_client.auth.login_access_token(token, state_file)
puts response

# CREATE project
project_name = 'Test project 1'
response = bw_client.project_client.create_project(organization_id, project_name)
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
response = bw_client.project_client.update_project(organization_id, project_id, name)
puts response

# CREATE secret
key = 'AWS-SES'
note = 'Private account'
value = '8t27.dfj;'
response = bw_client.secrets_client.create(organization_id, key, value, note, [project_id])
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

# SYNC secrets
response = bw_client.secrets_client.sync(organization_id, nil)
last_synced_date = Time.now.utc.strftime('%Y-%m-%dT%H:%M:%S.%6NZ')
puts response

response = bw_client.secrets_client.sync(organization_id, last_synced_date)
puts response

# UPDATE secret
note = 'updated password'
value = '7I.ert10AjK'
response = bw_client.secrets_client.update(organization_id, secret_id, key, value, note, [project_id])
puts response

# DELETE secret
response = bw_client.secrets_client.delete_secret([secret_id])
puts response

# DELETE project
response = bw_client.project_client.delete_projects([project_id])
puts response
