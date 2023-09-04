package main

import (
	"encoding/json"
)

type CommandRunnerInterface interface {
	RunCommand(command Command) string
}

type CommandRunner struct {
	client ClientPointer
	lib    BitwardenLibrary
}

func NewCommandRunner(client ClientPointer, lib BitwardenLibrary) *CommandRunner {
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
