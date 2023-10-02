package sdk

import (
	"encoding/json"
	"fmt"
)

type ProjectsInterface interface {
	Create(organizationID string, name string) (ResponseForProjectResponse, error)
	List(organizationID string) (ResponseForProjectsResponse, error)
	Get(projectID string) (ResponseForProjectResponse, error)
	Update(projectID string, organizationID string, name string) (ResponseForProjectResponse, error)
	Delete(projectIDs []string) (ResponseForProjectsDeleteResponse, error)
}

type Projects struct {
	CommandRunner CommandRunnerInterface
}

func NewProjects(commandRunner CommandRunnerInterface) *Projects {
	return &Projects{CommandRunner: commandRunner}
}

func (p *Projects) Get(id string) (ResponseForProjectResponse, error) {
	command := Command{
		Projects: &ProjectsCommand{
			Get: &ProjectGetRequest{
				ID: id,
			},
		},
	}

	responseStr := p.CommandRunner.RunCommand(command)
	var response ResponseForProjectResponse
	err := json.Unmarshal([]byte(responseStr), &response)
	if err != nil {
		return ResponseForProjectResponse{}, fmt.Errorf("failed to unmarshal response: %v", err)
	}

	return response, nil
}

func (p *Projects) Create(organizationId string, name string) (ResponseForProjectResponse, error) {
	command := Command{
		Projects: &ProjectsCommand{
			Create: &ProjectCreateRequest{
				OrganizationID: organizationId,
				Name:           name,
			},
		},
	}

	return p.executeCommand(command)
}

func (p *Projects) Update(id, organizationId string, name string) (ResponseForProjectResponse, error) {
	command := Command{
		Projects: &ProjectsCommand{
			Update: &ProjectPutRequest{
				ID:             id,
				OrganizationID: organizationId,
				Name:           name,
			},
		},
	}

	return p.executeCommand(command)
}

func (p *Projects) Delete(ids []string) (ResponseForProjectsDeleteResponse, error) {
	command := Command{
		Projects: &ProjectsCommand{
			Delete: &ProjectsDeleteRequest{
				IDS: ids,
			},
		},
	}

	responseStr := p.CommandRunner.RunCommand(command)
	var response ResponseForProjectsDeleteResponse
	err := json.Unmarshal([]byte(responseStr), &response)
	if err != nil {
		return ResponseForProjectsDeleteResponse{}, fmt.Errorf("failed to unmarshal response: %v", err)
	}

	return response, nil
}

func (p *Projects) List(organizationId string) (ResponseForProjectsResponse, error) {
	command := Command{
		Projects: &ProjectsCommand{
			List: &ProjectsListRequest{
				OrganizationID: organizationId,
			},
		},
	}

	responseStr := p.CommandRunner.RunCommand(command)
	var response ResponseForProjectsResponse
	err := json.Unmarshal([]byte(responseStr), &response)
	if err != nil {
		return ResponseForProjectsResponse{}, fmt.Errorf("failed to unmarshal response: %v", err)
	}

	return response, nil
}

// Helper method for common command execution and response handling
func (p *Projects) executeCommand(command Command) (ResponseForProjectResponse, error) {
	responseStr := p.CommandRunner.RunCommand(command)
	var response ResponseForProjectResponse
	err := json.Unmarshal([]byte(responseStr), &response)
	if err != nil {
		return ResponseForProjectResponse{}, fmt.Errorf("failed to unmarshal response: %v", err)
	}

	return response, nil
}
