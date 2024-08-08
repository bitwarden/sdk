package sdk

type GeneratorsInterface interface {
	GenerateSecret(request GenerateSecretRequest) (*GenerateSecretResponse, error)
}

type Generators struct {
	CommandRunner CommandRunnerInterface
}

func NewGenerators(commandRunner CommandRunnerInterface) *Generators {
	return &Generators{CommandRunner: commandRunner}
}

func (p *Generators) GenerateSecret(request GenerateSecretRequest) (*GenerateSecretResponse, error) {
	command := Command{
		Generators: &GeneratorsCommand{
			GenerateSecret: request,
		},
	}
	var response GenerateSecretResponse
	if err := p.executeCommand(command, &response); err != nil {
		return nil, err
	}
	return &response, nil
}

func (g *Generators) executeCommand(command Command, target interface{}) error {
	responseStr, err := g.CommandRunner.RunCommand(command)
	if err != nil {
		return err
	}
	return checkSuccessAndError(responseStr, target)
}
