package sdk_test

import (
	"encoding/json"
	"fmt"
	"io"
	"os"

	"github.com/bitwarden/sdk-go"
)

type E2EData struct {
	Projects        []TestProjectData
	Secrets         []TestSecretData
	MutableProjects []TestProjectData
	MutableSecrets  []TestSecretData
}

type TestProjectData struct {
	Name string
}

type TestSecretData struct {
	Key         string
	Value       string
	Note        string
	ProjectName string `json:"project_name"`
	ProjectId   string `json:"project_id"`
}

func GetTestData(path string, projects []sdk.ProjectResponse) E2EData {
	jsonFile, err := os.Open(path)
	if err != nil {
		panic(err)
	}
	defer jsonFile.Close()

	byteValue, _ := io.ReadAll(jsonFile)
	var data E2EData

	json.Unmarshal(byteValue, &data)

	for i, project := range data.Projects {
		data.Projects[i].Name = WithRunId(project.Name)
	}

	for i, secret := range data.Secrets {
		data.Secrets[i].Key = WithRunId(secret.Key)
		if secret.ProjectName != "" {
			data.Secrets[i].ProjectName = WithRunId(secret.ProjectName)
		}
	}

	for i, project := range data.MutableProjects {
		data.MutableProjects[i].Name = WithRunId(project.Name)
	}

	for i, secret := range data.MutableSecrets {
		data.MutableSecrets[i].Key = WithRunId(secret.Key)
		if secret.ProjectName != "" {
			data.MutableSecrets[i].ProjectName = WithRunId(secret.ProjectName)
		}
	}

	data.Secrets = secretsWithProjectId(data.Secrets, projects)
	data.MutableSecrets = secretsWithProjectId(data.MutableSecrets, projects)

	return data
}

func secretsWithProjectId(secrets []TestSecretData, projects []sdk.ProjectResponse) []TestSecretData {
	for i, secret := range secrets {
		if secret.ProjectName != "" {
			for _, project := range projects {
				if project.Name == secret.ProjectName {
					secrets[i].ProjectId = project.ID
				}
			}
		}
	}

	return secrets
}

func GetExpectedSecret(testDataPath string, key string, projects []sdk.ProjectResponse) TestSecretData {
	expectedSecrets := GetTestData(testDataPath, projects).Secrets
	for _, secret := range expectedSecrets {
		if secret.Key == key {
			return secret
		}
	}
	
	panic(fmt.Sprintf("Secret %s not found", key))
}
