# Bitwarden .NET SDK

The Bitwarden SDK (Software Development Kit) is a set of programming tools, libraries, and resources provided by Bitwarden for developers to integrate Bitwarden's password management and security capabilities into their own applications, services, or platforms. The SDK facilitates the seamless integration of Bitwarden's functionality into third-party software, enabling users to securely manage their passwords, credentials, and sensitive information across various digital environments.

## Create access token
- to create access token go to [access-tokens help page]

## Usage code snippets

### Create new Bitwarden client
```
const string accessToken = "<access-token>";
using var bitwardenClient = new BitwardenClient();
var loginResponse = bitwardenClient.AccessTokenLogin(accessToken);
```

### Create new project
- Organization is created in the UI
```
const string organizationIdStr = "<organization-id>";
var organizationId = Guid.Parse(organizationIdStr);
var responseForProjectResponse = bitwardenClient.Projects().Create(organizationId, "TestProject");
```

### List all projects
```
var response = bitwardenClient.Projects().List(organizationId);
```

### Update project
```
var projectId = responseForProjectResponse.Data.Id;
responseForProjectResponse = bitwardenClient.Projects().Get(projectId);
responseForProjectResponse = bitwardenClient.Projects().Update(projectId, organizationId, "TestProjectUpdated");
```

### Add new secret
```
var key = "key";
var value = "value";
var note = "note";
var responseForSecretResponse = bitwardenClient.Secrets().Create(key, value, note, organizationId, new Guid[]{projectId});
var secretId = responseForSecretResponse.Data.Id;
```

### List secrets
```
var responseForSecretIdentifiersResponse = bitwardenClient.Secrets().List(organizationId);
```

# Delete secret or project

```
var responseForSecretsDeleteResponse = bitwardenClient.Secrets().Delete(new Guid[]{secretId});
var responseForProjectsDeleteResponse = bitwardenClient.Projects().Delete(new Guid[]{projectId});
```

[access-tokens help page]: https://bitwarden.com/help/access-tokens/
