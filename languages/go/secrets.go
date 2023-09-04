package main

import (
	"encoding/json"
	"fmt"
)

type SecretsInterface interface {
	Create(key, value, note string, organizationID string, projectIDs []string) (ResponseForSecretResponse, error)
	List(organizationID string) (ResponseForSecretIdentifiersResponse, error)
	Get(secretID string) (ResponseForSecretResponse, error)
	Update(secretID string, key, value, note string, organizationID string, projectIDs []string) (ResponseForSecretResponse, error)
	Delete(secretIDs []string) (ResponseForSecretsDeleteResponse, error)
}

type Secrets struct {
	CommandRunner CommandRunnerInterface
}

func NewSecrets(commandRunner CommandRunnerInterface) *Secrets {
	return &Secrets{CommandRunner: commandRunner}
}

func (s *Secrets) Get(id string) (ResponseForSecretResponse, error) {
	command := Command{
		Secrets: &SecretsCommand{
			Get: &SecretGetRequest{
				ID: id,
			},
		},
	}
	return s.executeCommand(command)
}

func (s *Secrets) Create(key, value, note string, organizationId string, projectIds []string) (ResponseForSecretResponse, error) {
	command := Command{
		Secrets: &SecretsCommand{
			Create: &SecretCreateRequest{
				Key:            key,
				Value:          value,
				Note:           note,
				OrganizationID: organizationId,
				ProjectIDS:     projectIds,
			},
		},
	}
	return s.executeCommand(command)
}

func (s *Secrets) Update(id string, key, value, note string, organizationId string, projectIds []string) (ResponseForSecretResponse, error) {
	command := Command{
		Secrets: &SecretsCommand{
			Update: &SecretPutRequest{
				ID:             id,
				Key:            key,
				Value:          value,
				Note:           note,
				OrganizationID: organizationId,
				ProjectIDS:     projectIds,
			},
		},
	}
	return s.executeCommand(command)
}

func (s *Secrets) Delete(ids []string) (ResponseForSecretsDeleteResponse, error) {
	command := Command{
		Secrets: &SecretsCommand{
			Delete: &SecretsDeleteRequest{
				IDS: ids,
			},
		},
	}
	responseStr := s.CommandRunner.RunCommand(command)
	var response ResponseForSecretsDeleteResponse
	err := json.Unmarshal([]byte(responseStr), &response)
	if err != nil {
		return ResponseForSecretsDeleteResponse{}, fmt.Errorf("failed to unmarshal response: %v", err)
	}
	return response, nil
}

func (s *Secrets) List(organizationId string) (ResponseForSecretIdentifiersResponse, error) {
	command := Command{
		Secrets: &SecretsCommand{
			List: &SecretIdentifiersRequest{
				OrganizationID: organizationId,
			},
		},
	}
	responseStr := s.CommandRunner.RunCommand(command)
	var response ResponseForSecretIdentifiersResponse
	err := json.Unmarshal([]byte(responseStr), &response)
	if err != nil {
		return ResponseForSecretIdentifiersResponse{}, fmt.Errorf("failed to unmarshal response: %v", err)
	}
	return response, nil
}

// Helper method for common command execution and response handling
func (s *Secrets) executeCommand(command Command) (ResponseForSecretResponse, error) {
	responseStr := s.CommandRunner.RunCommand(command)
	var response ResponseForSecretResponse
	err := json.Unmarshal([]byte(responseStr), &response)
	if err != nil {
		return ResponseForSecretResponse{}, fmt.Errorf("failed to unmarshal response: %v", err)
	}
	return response, nil
}
