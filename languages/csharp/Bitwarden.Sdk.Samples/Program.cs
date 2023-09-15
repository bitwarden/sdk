using Bitwarden.Sdk;


var accessToken = Environment.GetEnvironmentVariable("ACCESS_TOKEN");
var organizationIdStr = Environment.GetEnvironmentVariable("ORGANIZATION_ID");

using var bitwardenClient = new BitwardenClient();
var loginResponse = bitwardenClient.AccessTokenLogin(accessToken);
var organizationId = Guid.Parse(organizationIdStr);
var responseForProjectResponse = bitwardenClient.Projects.Create(organizationId, "NewTestProject");
var projectId = responseForProjectResponse.Data.Id;
var responseForProjectsResponse = bitwardenClient.Projects.List(organizationId);
responseForProjectResponse = bitwardenClient.Projects.Get(projectId);
responseForProjectResponse = bitwardenClient.Projects.Update(projectId, organizationId, "NewTestProject2");

var key = "key";
var value = "value";
var note = "note";
var responseForSecretResponse =
    bitwardenClient.Secrets.Create(key, value, note, organizationId, new Guid[] { projectId });
var secretId = responseForSecretResponse.Data.Id;
var responseForSecretIdentifiersResponse = bitwardenClient.Secrets.List(organizationId);
responseForSecretResponse = bitwardenClient.Secrets.Get(secretId);
key = "key2";
value = "value2";
note = "note2";
responseForSecretResponse = bitwardenClient.Secrets
    .Update(secretId, key, value, note, organizationId, new Guid[] { projectId });

var responseForSecretsDeleteResponse = bitwardenClient.Secrets.Delete(new Guid[] { secretId });
var responseForProjectsDeleteResponse = bitwardenClient.Projects.Delete(new Guid[] { projectId });
