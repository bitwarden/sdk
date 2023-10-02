package sdk

import (
	"encoding/json"

	"github.com/bitwarden/sdk/languages/go/internal/cinterface"
)

type BitwardenClient struct {
	client        cinterface.ClientPointer
	lib           cinterface.BitwardenLibrary
	commandRunner CommandRunnerInterface
	Projects      ProjectsInterface
	Secrets       SecretsInterface
}

func NewBitwardenClient(settings ClientSettings, lib cinterface.BitwardenLibrary) *BitwardenClient {
	settingsJSON, err := json.Marshal(settings)
	if err != nil {
		panic(err)
	}

	client, err := lib.Init(string(settingsJSON))
	if err != nil {
		panic(err)
	}
	runner := NewCommandRunner(client, lib)

	return &BitwardenClient{
		lib:           lib,
		client:        client,
		commandRunner: runner,
		Projects:      NewProjects(runner),
		Secrets:       NewSecrets(runner),
	}
}

func (c *BitwardenClient) AccessTokenLogin(accessToken string) ResponseForAPIKeyLoginResponse {
	req := AccessTokenLoginRequest{AccessToken: accessToken}
	command := Command{AccessTokenLogin: &req}

	responseStr := c.commandRunner.RunCommand(command)

	var response ResponseForAPIKeyLoginResponse
	if err := json.Unmarshal([]byte(responseStr), &response); err != nil {
		panic(err)
	}

	return response
}

func (c *BitwardenClient) Close() {
	c.lib.FreeMem(c.client)
}
