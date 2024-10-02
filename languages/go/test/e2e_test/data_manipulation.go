package sdk_test

import (
	"os"
	"strings"

	sdk "github.com/bitwarden/sdk-go"
)

func RequiredEnv(key string) string {
	val := os.Getenv(key)
	if val == "" {
		panic("Missing required environment variable: " + key)
	}

	return val
}

func WithRunId(s string) string {
	return s + "-" + RequiredEnv("RUN_ID")
}

func FilterProjectsToThisRun(projects *sdk.ProjectsResponse) []sdk.ProjectResponse {
	var filteredProjects []sdk.ProjectResponse
	runId := RequiredEnv("RUN_ID")
	for _, project := range projects.Data {
		if strings.HasSuffix(project.Name, runId) {
			filteredProjects = append(filteredProjects, project)
		}
	}
	return filteredProjects
}

func FilterSecretsToThisRun(secrets *sdk.SecretIdentifiersResponse) []sdk.SecretIdentifierResponse {
	var filteredSecrets []sdk.SecretIdentifierResponse
	runId := RequiredEnv("RUN_ID")
	for _, secret := range secrets.Data {
		if strings.HasSuffix(secret.Key, runId) {
			filteredSecrets = append(filteredSecrets, secret)
		}
	}
	return filteredSecrets
}
