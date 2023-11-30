# Bitwarden Secrets Manager SDK

C++ bindings for interacting with the [Bitwarden Secrets Manager]. This is a beta release and might be missing some functionality.

## Create access token

Review the help documentation on [Access Tokens]

## Usage code snippets

### Client settings

```c++
// Optional - if not stressed, then default values are used
BitwardenSettings bitwardenSettings;
bitwardenSettings.set_api_url("<bitwarden-url>");
bitwardenSettings.set_identity_url("<bitwarden-identity>");
```


### Create new Bitwarden client

```c++
std::string accessToken = "<access-token>";
// Optional - argument in BitwardenClient
BitwardenClient bitwardenClient = BitwardenClient(bitwardenSettings);
bitwardenClient.accessTokenLogin(accessToken);
```

### Create new project

```c++
boost::uuids::uuid organizationUuid = boost::uuids::string_generator()("<organization-id>");
ProjectResponse projectResponseCreate = bitwardenClient.createProject(organizationUuid, "TestProject");
```

### List all projects

```c++
ProjectsResponse projectResponseList = bitwardenClient.listProjects(organizationUuid);
```

### Get project details

```c++
boost::uuids::uuid projectId = boost::uuids::string_generator()(projectResponseCreate.get_id());
ProjectResponse projectResponseGet = bitwardenClient.getProject(projectId);
```

### Update project

```c++
boost::uuids::uuid projectId = boost::uuids::string_generator()(projectResponseCreate.get_id());
ProjectResponse projectResponseUpdate = bitwardenClient.updateProject(projectId, organizationUuid, "TestProjectUpdated");
```

### Delete projects

```c++
SecretsDeleteResponse secretsDeleteResponse = bitwardenClient.deleteSecrets({secretId});
```

### Add new secret

```c++
std::string key = "key";
std::string value = "value";
std::string note = "note";
SecretResponse secretResponseCreate = bitwardenClient.createSecret(key, value, note, organizationUuid, {projectId});
```

### List secrets

```c++
SecretIdentifiersResponse secretIdentifiersResponse = bitwardenClient.listSecrets(organizationUuid);
```

### Get secret details

```
boost::uuids::uuid secretId = boost::uuids::string_generator()(secretResponseCreate.get_id());
SecretResponse secretResponseGet = bitwardenClient.getSecret(secretId);
```

### Update secret
```c++
SecretResponse secretResponseUpdate = bitwardenClient.updateSecret(secretId, "key2", "value2", "note2", organizationUuid, {projectId});
```

# Delete secrets

```c++
SecretsDeleteResponse secretsDeleteResponse = bitwardenClient.deleteSecrets({secretId});
```

[Access Tokens]: https://bitwarden.com/help/access-tokens/
[Bitwarden Secrets Manager]: https://bitwarden.com/products/secrets-manager/
