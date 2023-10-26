package sdk

import (
	"encoding/json"
	"fmt"
)

type ProjectsInterface interface {
	Create(organizationID string, name string) (*ProjectResponse, error)
	List(organizationID string) (*ProjectsResponse, error)
	Get(projectID string) (*ProjectResponse, error)
	Update(projectID string, organizationID string, name string) (*ProjectResponse, error)
	Delete(projectIDs []string) (*ProjectsDeleteResponse, error)
}

type Projects struct {
	CommandRunner CommandRunnerInterface
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

func NewProjects(commandRunner CommandRunnerInterface) *Projects {
	return &Projects{CommandRunner: commandRunner}
}

func (p *Projects) Get(id string) (*ProjectResponse, error) {
	command := Command{
		Projects: &ProjectsCommand{
			Get: &ProjectGetRequest{
				ID: id,
			},
		},
	}
	var response ProjectResponse
	if err := p.executeCommand(command, &response); err != nil {
		return nil, err
	}
	return &response, nil
}

func (p *Projects) Create(organizationID string, name string) (*ProjectResponse, error) {
	command := Command{
		Projects: &ProjectsCommand{
			Create: &ProjectCreateRequest{
				OrganizationID: organizationID,
				Name:           name,
			},
		},
	}

	var response ProjectResponse
	if err := p.executeCommand(command, &response); err != nil {
		return nil, err
	}
	return &response, nil
}

func (p *Projects) List(organizationID string) (*ProjectsResponse, error) {
	command := Command{
		Projects: &ProjectsCommand{
			List: &ProjectsListRequest{
				OrganizationID: organizationID,
			},
		},
	}

	var response ProjectResponse
	if err := p.executeCommand(command, &response); err != nil {
		return nil, err
	}
	return &response, nil
}

func (p *Projects) Update(projectID, organizationID, name string) (*ProjectResponse, error) {
	command := Command{
		Projects: &ProjectsCommand{
			Update: &ProjectPutRequest{
				ID:             projectID,
				OrganizationID: organizationID,
				Name:           name,
			},
		},
	}

	var response ProjectResponse
	if err := p.executeCommand(command, &response); err != nil {
		return nil, err
	}
	return &response, nil
}

func (p *Projects) Delete(projectIDs []string) (*ProjectsDeleteResponse, error) {
	command := Command{
		Projects: &ProjectsCommand{
			Delete: &ProjectsDeleteRequest{
				IDS: projectIDs,
			},
		},
	}

	var response ProjectResponse
	if err := p.executeCommand(command, &response); err != nil {
		return nil, err
	}
	return &response, nil
}

func (p *Projects) executeCommand(command Command, target interface{}) error {
	responseStr := p.CommandRunner.RunCommand(command)
	return checkSuccessAndError(responseStr, target)
}
