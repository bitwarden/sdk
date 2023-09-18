# Bitwarden Secrets Manager SDK

.NET bindings for interacting with the [Bitwarden Secrets Manager]. This is a beta release and might be missing some functionality.

## Create access token

Review the help documentation on [Access Tokens]

## Usage code snippets

### Create new Bitwarden client

```csharp
const string accessToken = "<access-token>";
using var bitwardenClient = new BitwardenClient();
bitwardenClient.AccessTokenLogin(accessToken);
```

### Create new project

```csharp
const string organizationIdStr = "<organization-id>";
var organizationId = Guid.Parse(organizationIdStr);
var projectResponse = bitwardenClient.Projects().Create(organizationId, "TestProject");
```

### List all projects

```csharp
var response = bitwardenClient.Projects.List(organizationId);
```

### Update project

```csharp
var projectId = projectResponse.Id;
projectResponse = bitwardenClient.Projects.Get(projectId);
projectResponse = bitwardenClient.Projects.Update(projectId, organizationId, "TestProjectUpdated");
```

### Add new secret

```csharp
var key = "key";
var value = "value";
var note = "note";
var secretResponse = bitwardenClient.Secrets.Create(key, value, note, organizationId, new Guid[]{projectId});
var secretId = secretResponse.Id;
```

### List secrets

```csharp
var secretIdentifiersResponse = bitwardenClient.Secrets.List(organizationId);
```

# Delete secret or project

```csharp
var secretsDeleteResponse = bitwardenClient.Secrets.Delete(new Guid[]{secretId});
var projectsDeleteResponse = bitwardenClient.Projects.Delete(new Guid[]{projectId});
```

[Access Tokens]: https://bitwarden.com/help/access-tokens/
[Bitwarden Secrets Manager]: https://bitwarden.com/products/secrets-manager/
