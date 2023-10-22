# Bitwarden Secrets Manager SDK

C++ bindings for interacting with the [Bitwarden Secrets Manager]. This is a beta release and might be missing some functionality.

## Create access token

Review the help documentation on [Access Tokens]

## Usage code snippets

### CLient settings

boost::optional<std::string> apiUrl("https://api.bitwarden.com");
boost::optional<std::string> identityUrl("https://identity.bitwarden.com");
boost::optional<std::string> user_agent("Bitwarden CPP-SDK");

ClientSettings clientSettings;
clientSettings.set_api_url(apiUrl);
clientSettings.set_identity_url(identityUrl);
clientSettings.set_device_type(Bitwarden::Sdk::DeviceType::SDK);
clientSettings.set_user_agent(user_agent);

### Create new Bitwarden client

```c++
std::string accessToken = "<access-token>";
BitwardenClient bitwardenClient = BitwardenClient(clientSettings);
ResponseForApiKeyLoginResponse responseForApiKeyLoginResponse = bitwardenClient.accessTokenLogin(accessToken);
```

### Create new project

```c++
boost::uuids::uuid organizationUuid = boost::uuids::string_generator()("<organization-id>");
ResponseForProjectResponse responseForProjectResponseCreate = bitwardenClient.createProject(organizationUuid, "TestProject");
```

### List all projects

```c++
ResponseForProjectsResponse responseForProjectResponseList = bitwardenClient.listProjects(organizationUuid);
```

### Get project details

```c++
boost::uuids::uuid projectId = boost::uuids::string_generator()(responseForProjectResponseCreate.get_data()->get_id());
ResponseForProjectResponse responseForProjectResponseGet = bitwardenClient.getProject(projectId);
```

### Update project

```c++
boost::uuids::uuid projectId = boost::uuids::string_generator()(responseForProjectResponseCreate.get_data()->get_id());
ResponseForProjectResponse responseForProjectResponseUpdate = bitwardenClient.updateProject(projectId, organizationUuid, "TestProjectUpdated");
```

### Delete projects

```c++
ResponseForSecretsDeleteResponse responseForSecretsDeleteResponse = bitwardenClient.deleteSecrets({secretId});
```

### Add new secret

```c++
std::string key = "key";
std::string value = "value";
std::string note = "note";
ResponseForSecretResponse responseForSecretResponseCreate = bitwardenClient.createSecret(key, value, note, organizationUuid, {projectId});
```

### List secrets

```c++
ResponseForSecretIdentifiersResponse responseForSecretIdentifiersResponse = bitwardenClient.listSecrets(organizationUuid);
```

### Get secret details

```
boost::uuids::uuid secretId = boost::uuids::string_generator()(responseForSecretResponseCreate.get_data()->get_id());
ResponseForSecretResponse responseForSecretResponseGet = bitwardenClient.getSecret(secretId);
```

### Update secret
```c++
ResponseForSecretResponse responseForSecretResponseUpdate = bitwardenClient.updateSecret(secretId, "key2", "value2", "note2", organizationUuid, {projectId});
```

# Delete secrets

```c++
ResponseForSecretsDeleteResponse responseForSecretsDeleteResponse = bitwardenClient.deleteSecrets({secretId});
```

[Access Tokens]: https://bitwarden.com/help/access-tokens/
[Bitwarden Secrets Manager]: https://bitwarden.com/products/secrets-manager/
