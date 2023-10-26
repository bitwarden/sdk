package sdk

type ProjectsInterface interface {
	Create(organizationID string, name string) (*ProjectResponse, error)
	List(organizationID string) (*ProjectsResponse, error)
	Get(projectID string) (*ProjectResponse, error)
	Update(projectID string, organizationID string, name string) (*ProjectResponse, error)
	Delete(projectIDs []string) (*ProjectResponse, error)
}

type Projects struct {
	CommandRunner CommandRunnerInterface
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

	var response ProjectsResponse
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

func (p *Projects) Delete(projectIDs []string) (*ProjectResponse, error) {
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
