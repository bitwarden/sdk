# Bitwarden SDK in Go

This SDK is designed to interact with Bitwarden services in Go. It includes implementations for managing projects and secrets, as well as a client interface to facilitate operations like login.

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
import "github.com/bitwarden/sdk/languages/go"

settings := ClientSettings{
  // Your settings here
}
lib := BitwardenLibraryImpl{}
client := NewBitwardenClient(settings, lib)
```

---

### Login

To login using an access token:

```go
response := client.AccessTokenLogin("your_access_token_here")
```

---

### Projects

#### Create a Project

```go
response, err := client.Projects.Create("organization_id", "project_name")
```

#### List Projects

```go
response, err := client.Projects.List("organization_id")
```

#### Update a Project

```go
response, err := client.Projects.Update("project_id", "organization_id", "new_project_name")
```

#### Delete Projects

```go
response, err := client.Projects.Delete([]string{"project_id_1", "project_id_2"})
```

---

### Secrets

#### Create a Secret

```go
response, err := client.Secrets.Create("key", "value", "note", "organization_id", []string{"project_id"})
```

#### List Secrets

```go
response, err := client.Secrets.List("organization_id")
```

#### Update a Secret

```go
response, err := client.Secrets.Update("secret_id", "new_key", "new_value", "new_note", "organization_id", []string{"project_id"})
```

#### Delete Secrets

```go
response, err := client.Secrets.Delete([]string{"secret_id_1", "secret_id_2"})
```

---

### Close Client

To free up resources:

```go
client.Close()
```

---

For more detailed information, refer to the code comments and method signatures.
