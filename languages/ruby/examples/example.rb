# NOTE - for example purpose only - import gem instead
require 'bitwarden-sdk'

token = '<insert access token here>'
organization_id = '<organization id here>'

bitwarden_settings = BitwardenSDK::BitwardenSettings.new(
  'https://api.bitwarden.com',
  'https://identity.bitwarden.com/connect/token'
)

bw_client = BitwardenSDK::BitwardenClient.new(bitwarden_settings)
response = bw_client.access_token_login(token)
puts response

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

# DELETE project
response = bw_client.project_client.delete_projects([project_id])
puts response
