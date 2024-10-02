package sdk_test

// Root for end to end testing the secrets manager go language binding

import (
	"os"
	"testing"

	sdk "github.com/bitwarden/sdk-go"
)

var client sdk.BitwardenClientInterface
var mutableClient sdk.BitwardenClientInterface
var organizationId string
var actualProjects []sdk.ProjectResponse
var actualMutableProjects []sdk.ProjectResponse
var actualSecrets []sdk.SecretIdentifierResponse
var actualMutableSecrets []sdk.SecretIdentifierResponse
var testDataPath string

func TestMain(m *testing.M) {
	apiURL := RequiredEnv("API_URL")
	identityURL := RequiredEnv("IDENTITY_URL")
	organizationId = RequiredEnv("ORGANIZATION_ID")
	testDataPath = RequiredEnv("TEST_DATA_FILE")

	// Setup clients
	var client_error error
	client, client_error = sdk.NewBitwardenClient(&apiURL, &identityURL)
	if client_error != nil {
		panic(client_error)
	}
	mutableClient, client_error = sdk.NewBitwardenClient(&apiURL, &identityURL)
	if client_error != nil {
		panic(client_error)
	}

	accessToken := RequiredEnv("ACCESS_TOKEN");
	mutableAccessToken := RequiredEnv("MUTABLE_ACCESS_TOKEN")
	stateFile := "state.json"
	mutableStateFile := "mutable_state.json"
	client_error = client.AccessTokenLogin(accessToken, &stateFile)
	if client_error != nil {
		panic(client_error)
	}
	client_error = mutableClient.AccessTokenLogin(mutableAccessToken, &stateFile)
	if client_error != nil {
		panic(client_error)
	}

	// Read projects
	projectsResponse, err := client.Projects().List(organizationId)
	if err != nil {
		panic(err)
	}
	actualProjects = FilterProjectsToThisRun(projectsResponse)

	// Read mutable projects
	mutableProjectsResponse, err := mutableClient.Projects().List(organizationId)
	if err != nil {
		panic(err)
	}
	actualMutableProjects = FilterProjectsToThisRun(mutableProjectsResponse)

	// Read secrets
	secretsResponse, err := client.Secrets().List(organizationId)
	if err != nil {
		panic(err)
	}
	actualSecrets = FilterSecretsToThisRun(secretsResponse)

	// Read mutable secrets
	mutableSecretsResponse, err := mutableClient.Secrets().List(organizationId)
	if err != nil {
		panic(err)
	}
	actualMutableSecrets = FilterSecretsToThisRun(mutableSecretsResponse)

	code := m.Run()

	// Clean up state files
	os.Remove(stateFile)
	os.Remove(mutableStateFile)

	os.Exit(code)
}
