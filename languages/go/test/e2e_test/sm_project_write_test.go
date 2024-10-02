package sdk_test

import (
	"testing"

	sdk "github.com/bitwarden/sdk-go"
)

func TestCreateProject(t *testing.T) {
	toCreate := TestProjectData{Name: WithRunId("test-project")}
	created, err := mutableClient.Projects().Create(organizationId, toCreate.Name)

	if err != nil {
		t.Fatal(err)
	}

	if !projectEqual(toCreate, created) {
		t.Fatalf("Expected %v, got %v", toCreate, created)
	}
}

func TestUpdateProject(t *testing.T) {
	newData := TestProjectData{Name: WithRunId("updated-test-project")}
	toUpdate := GetProject(WithRunId("to_update"), actualMutableProjects)

	updated, err := mutableClient.Projects().Update(toUpdate.ID, toUpdate.ID, newData.Name)
	if err != nil {
		t.Fatal(err)
	}

	if !projectEqual(newData, updated) {
		t.Fatalf("Expected %v, got %v", newData, updated)
	}
}

func TestDeleteProject(t *testing.T) {
	toDelete := GetProject(WithRunId("to_delete"), actualMutableProjects)

	deleted, err := mutableClient.Projects().Delete([]string{toDelete.ID})
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


func projectEqual(expected TestProjectData, actual *sdk.ProjectResponse) bool {
	return expected.Name == actual.Name
}
