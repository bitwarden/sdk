package main

import (
	"fmt"
	"os"

	sdk "github.com/bitwarden/sdk/languages/go"
	"github.com/gofrs/uuid"
)

func main() {
	apiURL := os.Getenv("API_URL")
	identityURL := os.Getenv("IDENTITY_URL")

	bitwardenClient, _ := sdk.NewBitwardenClient(&apiURL, &identityURL)

	accessToken := os.Getenv("ACCESS_TOKEN")
	organizationIDStr := os.Getenv("ORGANIZATION_ID")
	projectName := os.Getenv("PROJECT_NAME")

	if projectName == "" {
		projectName = "NewTestProject" // default value
	}

	err := bitwardenClient.AccessTokenLogin(accessToken)
	if err != nil {
		panic(err)
	}

	organizationID, err := uuid.FromString(organizationIDStr)
	if err != nil {
		panic(err)
	}

	project, err := bitwardenClient.Projects.Create(organizationID.String(), projectName)
	if err != nil {
		panic(err)
	}
	fmt.Println(project)
	projectID := project.ID
	fmt.Println(projectID)

	if _, err = bitwardenClient.Projects.List(organizationID.String()); err != nil {
		panic(err)
	}

	if _, err = bitwardenClient.Projects.Get(projectID); err != nil {
		panic(err)
	}

	if _, err = bitwardenClient.Projects.Update(projectID, organizationID.String(), projectName+"2"); err != nil {
		panic(err)
	}

	key := "key"
	value := "value"
	note := "note"

	secret, err := bitwardenClient.Secrets.Create(key, value, note, organizationID.String(), []string{projectID})
	if err != nil {
		panic(err)
	}
	secretID := secret.ID

	if _, err = bitwardenClient.Secrets.List(organizationID.String()); err != nil {
		panic(err)
	}

	if _, err = bitwardenClient.Secrets.Get(secretID); err != nil {
		panic(err)
	}

	if _, err = bitwardenClient.Secrets.Update(secretID, key, value, note, organizationID.String(), []string{projectID}); err != nil {
		panic(err)
	}

	if _, err = bitwardenClient.Secrets.Delete([]string{secretID}); err != nil {
		panic(err)
	}

	if _, err = bitwardenClient.Projects.Delete([]string{projectID}); err != nil {
		panic(err)
	}

	defer bitwardenClient.Close()
}
