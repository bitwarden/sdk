using Bitwarden.Sdk;


var accessToken = Environment.GetEnvironmentVariable("ACCESS_TOKEN");
var organizationIdStr = Environment.GetEnvironmentVariable("ORGANIZATION_ID");

using var bitwardenClient = new BitwardenClient();
bitwardenClient.AccessTokenLogin(accessToken);
var organizationId = Guid.Parse(organizationIdStr);
var projectResponse = bitwardenClient.Projects.Create(organizationId, "NewTestProject");
var projectId = projectResponse.Id;
var projectsResponse = bitwardenClient.Projects.List(organizationId);
projectResponse = bitwardenClient.Projects.Get(projectId);
projectResponse = bitwardenClient.Projects.Update(projectId, organizationId, "NewTestProject2");

var key = "key";
var value = "value";
var note = "note";
var secretResponse =
    bitwardenClient.Secrets.Create(key, value, note, organizationId, new Guid[] { projectId });
var secretId = secretResponse.Id;
var responseForSecretIdentifiersResponse = bitwardenClient.Secrets.List(organizationId);
secretResponse = bitwardenClient.Secrets.Get(secretId);
key = "key2";
value = "value2";
note = "note2";
secretResponse = bitwardenClient.Secrets
    .Update(secretId, key, value, note, organizationId, new Guid[] { projectId });

var responseForSecretsDeleteResponse = bitwardenClient.Secrets.Delete(new Guid[] { secretId });
var responseForProjectsDeleteResponse = bitwardenClient.Projects.Delete(new Guid[] { projectId });
