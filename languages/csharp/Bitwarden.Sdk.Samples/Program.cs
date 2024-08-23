using Bitwarden.Sdk;

// Configure secrets
var accessToken = Environment.GetEnvironmentVariable("ACCESS_TOKEN")!;
var organizationIdString = Environment.GetEnvironmentVariable("ORGANIZATION_ID")!;
var organizationId = Guid.Parse(organizationIdString);

// Create SDK Client
using var bitwardenClient = new BitwardenClient();

// Authenticate
await bitwardenClient.AccessTokenLoginAsync(accessToken);

// Project operations
var projectResponse = bitwardenClient.Projects.Create(organizationId, "NewTestProject");
var projectsResponse = bitwardenClient.Projects.List(organizationId);
var projectId = projectResponse.Id;
projectResponse = bitwardenClient.Projects.Get(projectId);
projectResponse = bitwardenClient.Projects.Update(projectId, organizationId, "NewTestProject2");

// Secret operations
var secretResponse =
    bitwardenClient.Secrets.Create("key", "value", "note", organizationId, new[] { projectId });
var secretId = secretResponse.Id;
var secretIdentifiersResponse = bitwardenClient.Secrets.List(organizationId);
secretResponse = bitwardenClient.Secrets.Get(secretId);
secretResponse = bitwardenClient.Secrets
    .Update(secretId, "key2", "value2", "note2", organizationId, new[] { projectId });
bitwardenClient.Secrets.Delete(new[] { secretId });
bitwardenClient.Projects.Delete(new[] { projectId });
