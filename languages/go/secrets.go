package sdk

import (
	"encoding/json"
	"fmt"
)

type SecretsInterface interface {
	Create(key, value, note string, organizationID string, projectIDs []string) (*SecretResponse, error)
	List(organizationID string) (*SecretIdentifiersResponse, error)
	Get(secretID string) (*SecretResponse, error)
	Update(secretID string, key, value, note string, organizationID string, projectIDs []string) (*SecretResponse, error)
	Delete(secretIDs []string) (*SecretsDeleteResponse, error)
}

type Secrets struct {
	CommandRunner CommandRunnerInterface
}

func NewSecrets(commandRunner CommandRunnerInterface) *Secrets {
	return &Secrets{CommandRunner: commandRunner}
}

func (s *Secrets) executeCommand(command Command, target interface{}) error {
	responseStr := s.CommandRunner.RunCommand(command)
	return checkSuccessAndError(responseStr, target)
}

func checkSuccessAndError(responseStr string, v interface{}) error {
	var wrapper struct {
		Success      bool    `json:"success"`
		ErrorMessage *string `json:"errorMessage"`
	}

	err := json.Unmarshal([]byte(responseStr), &wrapper)
	if err != nil {
		return fmt.Errorf("failed to unmarshal wrapper response: %v", err)
	}

	if !wrapper.Success {
		if wrapper.ErrorMessage != nil {
			return fmt.Errorf("API error: %s", *wrapper.ErrorMessage)
		}
		return fmt.Errorf("API error: unknown")
	}

	err = json.Unmarshal([]byte(responseStr), &v)
	if err != nil {
		return fmt.Errorf("failed to unmarshal response: %v", err)
	}

	return nil
}

func (s *Secrets) Create(key, value, note string, organizationID string, projectIDs []string) (*SecretResponse, error) {
	command := Command{
		Secrets: &SecretsCommand{
			Create: &SecretCreateRequest{
				Key:            key,
				Value:          value,
				Note:           note,
				OrganizationID: organizationID,
				ProjectIDS:     projectIDs,
			},
		},
	}

	var response SecretResponse
	if err := s.executeCommand(command, &response); err != nil {
		return nil, err
	}
	return &response, nil
}

func (s *Secrets) List(organizationID string) (*SecretIdentifiersResponse, error) {
	command := Command{
		Secrets: &SecretsCommand{
			List: &SecretIdentifiersRequest{
				OrganizationID: organizationID,
			},
		},
	}

	var response SecretIdentifiersResponse
	if err := s.executeCommand(command, &response); err != nil {
		return nil, err
	}
	return &response, nil
}

func (s *Secrets) Get(id string) (*SecretResponse, error) {
	command := Command{
		Secrets: &SecretsCommand{
			Get: &SecretGetRequest{
				ID: id,
			},
		},
	}

	var response SecretResponse
	if err := s.executeCommand(command, &response); err != nil {
		return nil, err
	}
	return &response, nil
}

func (s *Secrets) Update(id string, key, value, note string, organizationID string, projectIDs []string) (*SecretResponse, error) {
	command := Command{
		Secrets: &SecretsCommand{
			Update: &SecretPutRequest{
				ID:             id,
				Key:            key,
				Value:          value,
				Note:           note,
				OrganizationID: organizationID,
				ProjectIDS:     projectIDs,
			},
		},
	}

	var response SecretResponse
	if err := s.executeCommand(command, &response); err != nil {
		return nil, err
	}
	return &response, nil
}

func (s *Secrets) Delete(ids []string) (*SecretsDeleteResponse, error) {
	command := Command{
		Secrets: &SecretsCommand{
			Delete: &SecretsDeleteRequest{
				IDS: ids,
			},
		},
	}

	var response SecretsDeleteResponse
	if err := s.executeCommand(command, &response); err != nil {
		return nil, err
	}
	return &response, nil
}
