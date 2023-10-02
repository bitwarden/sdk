package sdk

import (
	"encoding/json"

	"github.com/bitwarden/sdk/languages/go/internal/cinterface"
)

type CommandRunnerInterface interface {
	RunCommand(command Command) string
}

type CommandRunner struct {
	client cinterface.ClientPointer
	lib    cinterface.BitwardenLibrary
}

func NewCommandRunner(client cinterface.ClientPointer, lib cinterface.BitwardenLibrary) *CommandRunner {
	return &CommandRunner{
		client: client,
		lib:    lib,
	}
}

func (c *CommandRunner) RunCommand(command Command) string {
	commandJSON, err := json.Marshal(command)
	if err != nil {
		panic(err)
	}

	responseStr, err := c.lib.RunCommand(string(commandJSON), c.client)
	if err != nil {
		panic(err)
	}

	return responseStr
}
