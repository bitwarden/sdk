package main

import (
	"fmt"
	"log"
	"sync"

	sdk "github.com/bitwarden/sdk-go"
)

var (
	ApiUrl         = "http://localhost:4000"
	IdentityUrl    = "http://localhost:33656"
	OrganizationId = ""
	AccessToken    = ""
	statePath      = ""
)

func main() {
	// create the client
	bitwardenClient, err := sdk.NewBitwardenClient(&ApiUrl, &IdentityUrl)
	if err != nil {
		log.Fatal(err)
	}

	// access token login
	err = bitwardenClient.AccessTokenLogin(AccessToken, &statePath)
	if err != nil {
		log.Fatal(err)
	}

	// build the waitgroup
	var wg sync.WaitGroup
	wg.Add(2)

	// build the goroutines
	go func() {
		defer wg.Done()
		for i := 0; i < 100; i++ {
			projects, err := bitwardenClient.Projects().List(OrganizationId)
			if err != nil {
				log.Println("Error listing projects:", err)
				return
			}

			fmt.Printf("# of Projects (iteration %d): %d\n", i+1, len(projects.Data))
			for _, project := range projects.Data {
				fmt.Printf("ID: %s\n", project.ID)
				fmt.Printf("Name: %s\n", project.Name)
			}
		}
	}()
	go func() {
		defer wg.Done()
		for i := 0; i < 100; i++ {
			secrets, err := bitwardenClient.Secrets().List(OrganizationId)
			if err != nil {
				log.Println("Error listing secrets:", err)
				return
			}

			fmt.Printf("# of Secrets (iteration %d): %d\n", i+1, len(secrets.Data))
			for _, secret := range secrets.Data {
				fmt.Printf("ID: %s\n", secret.ID)
				fmt.Printf("Name: %s\n", secret.Key)
			}
		}
	}()

	wg.Wait()
}
