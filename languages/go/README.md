# Bitwarden SDK in Go

This SDK is designed to interact with Bitwarden services in Go. It includes implementations for
managing projects and secrets, as well as a client interface to facilitate operations like login.

## Prerequisites

- Go installed
- C environment to run CGO

## Installation

Download the SDK files and place them in your Go project directory.

## Table of Contents

- [Initialization](#initialization)
- [Login](#login)
- [Projects](#projects)
- [Secrets](#secrets)
- [Close Client](#close-client)

---

### Initialization

To initialize the client, you need to import the SDK and create a new `BitwardenClient` instance.

```go
import "github.com/bitwarden/sdk-go"

bitwardenClient, _ := sdk.NewBitwardenClient(&apiURL, &identityURL)
```

---

### Login

To login using an access token. Define some `stateFile` and pass it to use state, or pass `nil`
instead to not use state.

```go
stateFile := os.Getenv("STATE_DIR")

err := bitwardenClient.AccessTokenLogin(accessToken, &stateFile)
```

---

### Projects

#### Create a Project

```go
project, err := client.Projects().Create("organization_id", "project_name")
```

#### List Projects

```go
projects, err := client.Projects().List("organization_id")
```

#### Update a Project

```go
project, err := client.Projects().Update("project_id", "organization_id", "new_project_name")
```

#### Delete Projects

```go
project, err := client.Projects().Delete([]string{"project_id_1", "project_id_2"})
```

---

### Secrets

#### Create a Secret

```go
secret, err := client.Secrets().Create("key", "value", "note", "organization_id", []string{"project_id"})
```

#### List Secrets

```go
secrets, err := client.Secrets().List("organization_id")
```

#### Update a Secret

```go
secret, err := client.Secrets().Update("secret_id", "new_key", "new_value", "new_note", "organization_id", []string{"project_id"})
```

#### Delete Secrets

```go
secret, err := client.Secrets().Delete([]string{"secret_id_1", "secret_id_2"})
```

#### Secrets Sync

```go
secretsSync, err := client.Secrets().Sync("organization_id", nil)

lastSyncedDate := time.Now()
secretsSync, err := client.Secrets().Sync("organization_id", lastSyncedDate)
```

---

### Close Client

To free up resources:

```go
defer bitwardenClient.Close()
```

---

For more detailed information, refer to the code comments and method signatures.
