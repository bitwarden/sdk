package sdk_test

import (
	"testing"

	sdk "github.com/bitwarden/sdk-go"
)

func TestListProjects(t *testing.T) {
	expectedProjects := GetTestData(testDataPath, actualProjects).Projects
	projects, err := client.Projects().List(organizationId)
	if err != nil {
		t.Fatal(err)
	}

	if len(projects.Data) != len(expectedProjects) {
		t.Fatalf("Expected %d projects, got %d", len(expectedProjects), len(projects.Data))
	}

out:
	for _, project := range projects.Data {
		for _, expectedProject := range expectedProjects {
			if project.Name == expectedProject.Name {
				break out
			}
		}
		t.Fatalf("Project %s not found in expected projects (%v)", project.Name, expectedProjects)
	}

	if len(projects.Data) == 0 {
		t.Fatal("No projects found")
	}
}

func TestListSecrets(t *testing.T) {
	expectedSecrets := GetTestData(testDataPath, actualProjects).Secrets
	secrets, err := client.Secrets().List(organizationId)
	if err != nil {
		t.Fatal(err)
	}

	if len(secrets.Data) != len(expectedSecrets) {
		t.Fatalf("Expected %d secrets, got %d", len(expectedSecrets), len(secrets.Data))
	}

out:
	for _, secret := range secrets.Data {
		for _, expectedSecret := range expectedSecrets {
			if secret.Key == expectedSecret.Key {
				break out
			}
		}
		t.Fatalf("Secret %s not found in expected secrets (%v)", secret.Key, expectedSecrets)
	}

	if len(secrets.Data) == 0 {
		t.Fatal("No secrets found")
	}
}

func TestGetProject(t *testing.T) {
	for _, expectedProject := range actualProjects {
		project, err := client.Projects().Get(expectedProject.ID)
		if err != nil {
			t.Fatal(err)
		}

		if project.Name != expectedProject.Name {
			t.Fatalf("Expected project name %s, got %s", expectedProject.Name, project.Name)
		}
	}
}

func TestGetSecret(t *testing.T) {
	for _, actualSecret := range actualSecrets {
		expectedSecret := GetExpectedSecret(testDataPath, actualSecret.Key, actualProjects)
		secret, err := client.Secrets().Get(actualSecret.ID)
		if err != nil {
			t.Fatal(err)
		}

		if !SecretsEqual(expectedSecret, secret) {
			t.Fatalf("Expected secret %v, got %v", actualSecret, secret)
		}
	}
}

func SecretsEqual(expected TestSecretData, actual *sdk.SecretResponse) bool {
	return expected.Key == actual.Key && expected.Value == actual.Value && expected.Note == actual.Note && expected.ProjectId == *actual.ProjectID
}
