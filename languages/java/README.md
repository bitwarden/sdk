# Bitwarden Secrets Manager SDK

Java bindings for interacting with the [Bitwarden Secrets Manager]. This is a beta release and might be missing some
functionality.

## Create access token

Review the help documentation on [Access Tokens]

## Usage code snippets

### Create new Bitwarden client

```java
import com.bitwarden.sdk.*;

BitwardenSettings bitwardenSettings = new BitwardenSettings();
bitwardenSettings.setApiUrl("https://api.bitwarden.com");
bitwardenSettings.setIdentityUrl("https://identity.bitwarden.com");
BitwardenClient bitwardenClient = new BitwardenClient(bitwardenSettings);
bitwardenClient.accessTokenLogin("<access-token>");
```

### Create new project

```java
UUID organizationId = UUID.fromString("<organization-id>");
var projectResponse = bitwardenClient.projects().create(organizationId, "TestProject");
```

### List all projects

```java
var projectsResponse = bitwardenClient.projects().list(organizationId);
```

### Update project

```java
UUID projectId = projectResponse.getID();
projectResponse = bitwardenClient.projects().get(projectId);
projectResponse = bitwardenClient.projects().update(projectId, organizationId, "TestProjectUpdated");
```

### Add new secret

```java
String key = "key";
String value = "value";
String note = "note";
var secretResponse = bitwardenClient.secrets().create(key, value, note, organizationId, new UUID[]{projectId});
UUID secretId = secretResponse.getID();
```

### Update secret

```java
bitwardenClient.secrets().update(secretId, key2, value2, note2, organizationId, new UUID[]{projectId});
```

### List secrets

```java
var secretIdentifiersResponse = bitwardenClient.secrets().list(organizationId);
```

# Delete secret or project

```java
bitwardenClient.secrets().delete(new UUID[]{secretId});
bitwardenClient.projects().delete(new UUID[]{projectId});
```

[Access Tokens]: https://bitwarden.com/help/access-tokens/
[Bitwarden Secrets Manager]: https://bitwarden.com/products/secrets-manager/
