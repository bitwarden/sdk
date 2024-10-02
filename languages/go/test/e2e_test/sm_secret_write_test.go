package sdk_test

import (
	"fmt"
	"testing"

	sdk "github.com/bitwarden/sdk-go"
)

func TestCreateSecret(t *testing.T) {
	project := GetProject(WithRunId("for_write_tests"), actualMutableProjects)
	toCreate := TestSecretData{
		Key: WithRunId("test-secret"),
		Value: "test-value",
		Note: "test-note",
		ProjectName: project.Name,
		ProjectId: project.ID,
	}
	created, err := mutableClient.Secrets().Create(toCreate.Key, toCreate.Value, toCreate.Note, organizationId, []string{toCreate.ProjectId})

	if err != nil {
		t.Fatal(err)
	}

	if !SecretsEqual(toCreate, created) {
		t.Fatalf("Expected %v, got %v", toCreate, created)
	}
}

func TestUpdateSecret(t *testing.T) {
	project := GetProject(WithRunId("for_write_tests"), actualMutableProjects)
	toUpdate := GetSecret(WithRunId("to_update"), actualMutableSecrets)
	newData := TestSecretData{
		Key: WithRunId("updated-test-secret"),
		Value: "updated-test-value",
		Note: "updated-test-note",
		ProjectName: project.Name,
		ProjectId: project.ID,
	}

	updated, err := mutableClient.Secrets().Update(toUpdate.ID, newData.Key, newData.Value, newData.Note, organizationId, []string{newData.ProjectId})

	if err != nil {
		t.Fatal(err)
	}

	if !SecretsEqual(newData, updated) {
		t.Fatalf("Expected %v, got %v", newData, updated)
	}
}

func TestDeleteSecret(t *testing.T) {
	toDelete := GetSecret(WithRunId("to_delete"), actualMutableSecrets)

	deleted, err := mutableClient.Secrets().Delete([]string{toDelete.ID})
	if err != nil {
		t.Fatal(err)
	}

	var wasNotDeleted = true
	for _, deletedData := range deleted.Data {
		if deletedData.ID == toDelete.ID {
			wasNotDeleted = false
		}
	}
	if wasNotDeleted {
		t.Fatalf("Expected %v, got %v", toDelete, deleted)
	}
}

func GetProject(name string, projects []sdk.ProjectResponse) sdk.ProjectResponse {
	for _, project := range projects {
		if project.Name == name {
			return project
		}
	}

	panic(fmt.Sprintf("Project %s not found in %#v", name, actualMutableProjects))
}

func GetSecret(key string, secrets []sdk.SecretIdentifierResponse) sdk.SecretIdentifierResponse {
	for _, secret := range secrets {
		if secret.Key == key {
			return secret
		}
	}

	panic(fmt.Sprintf("Secret %s not found in %#v", key, actualSecrets))
}
