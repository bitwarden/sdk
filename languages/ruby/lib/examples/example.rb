# NOTE - for example purpose only - import gem instead
require_relative '../ruby-bitwarden-sdk'

token = ""

bw_client = BitwardenClient.new
response = bw_client.authorize(token)
puts response

# GET project
response = bw_client.project_client.get("b23818dd-827b-4a22-b97a-b07e010ae9d4")
puts response

# CREATE project
response = bw_client.project_client.create_project("test_project_1", "5688da1f-cc25-41d7-bb9f-b0740144ef1d")
puts response

# LIST projects
response = bw_client.project_client.list_projects("5688da1f-cc25-41d7-bb9f-b0740144ef1d")
puts response

# UPDATE projects
response = bw_client.project_client.update_project("ef9d3d37-f0dc-4b21-a842-b0810129bf02", "test_project_1", "5688da1f-cc25-41d7-bb9f-b0740144ef1d")
puts response

# DELETE
response = bw_client.project_client.delete_projects(["13a015aa-e3dc-4854-875a-b08101512d2f"])
puts response

# CREATE secret
response = bw_client.secrets_client.create("AWS-SES", "important!", "5688da1f-cc25-41d7-bb9f-b0740144ef1d", ["ef9d3d37-f0dc-4b21-a842-b0810129bf02"], "8t27.dfj;")
puts response

# GET secret
response = bw_client.secrets_client.get("fa175a5b-da76-48c3-b44b-b0810151638c")
puts response

# GET secret by ids
response = bw_client.secrets_client.get_by_ids(["e8561721-0455-438c-bbbe-b0810152f534"])
puts response

# LIST secrets
response = bw_client.secrets_client.list("5688da1f-cc25-41d7-bb9f-b0740144ef1d")
puts response

# DELETE secret
response = bw_client.secrets_client.delete_secret(["b03cf64b-e894-4675-9f59-b0810152abe6"])
puts response

# UPDATE secret
response = bw_client.secrets_client.update("683c25f3-a463-49ba-bed4-b0810134a7b1", "AWS-SES", "very important!","5688da1f-cc25-41d7-bb9f-b0740144ef1d", ["4647aede-33f1-4ad1-a258-b07a014a48a7"], "7I.ert10AjK")
puts response
