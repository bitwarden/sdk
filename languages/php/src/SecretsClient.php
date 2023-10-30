<?php

namespace Bitwarden\Sdk;

use Bitwarden\Sdk\Schemas\Command;
use Bitwarden\Sdk\Schemas\SecretCreateRequest;
use Bitwarden\Sdk\Schemas\SecretGetRequest;
use Bitwarden\Sdk\Schemas\SecretIdentifiersRequest;
use Bitwarden\Sdk\Schemas\SecretPutRequest;
use Bitwarden\Sdk\Schemas\SecretsCommand;
use Bitwarden\Sdk\Schemas\SecretsDeleteRequest;
use Bitwarden\Sdk\Schemas\SecretsGetRequest;

class SecretsClient
{
    private CommandRunner $commandRunner;

    public function __construct(CommandRunner $commandRunner)
    {
        $this->commandRunner = $commandRunner;
    }

    public function get(int $secret_id)
    {
        $secret_get_request = new SecretGetRequest();
        $secret_get_request->id = $secret_id;
        $secret_get_request->validate();
        $secret_command = new SecretsCommand();
        $secret_command->get = $secret_get_request;
        $secret_command->validate();
        $command = new Command();
        $command->secrets = $secret_get_request;
        $command->validate();
        $this->commandRunner->run($command);
    }

    public function get_by_ids(array $secret_ids)
    {
        $project_get_by_ids_request = new SecretsGetRequest();
        $project_get_by_ids_request->ids = $secret_ids;
        $project_get_by_ids_request->validate();
        $secrets_command = new SecretsCommand();
        $secrets_command->get_by_ids = $project_get_by_ids_request;
        $secrets_command->validate();
        $command = new Command();
        $command->secrets = $secrets_command;
        $command->validate();
        $this->commandRunner->run($command);
    }

    public function list(string $organization_id)
    {
        $secrets_list_request = new SecretIdentifiersRequest();
        $secrets_list_request->organizationId = $organization_id;
        $secrets_list_request->validate();
        $secrets_command = new SecretsCommand();
        $secrets_command->get_by_ids = $secrets_list_request;
        $secrets_command->validate();
        $command = new Command();
        $command->secrets = $secrets_command;
        $command->validate();
        $this->commandRunner->run($command);
    }

    public function create(string $key, string $note, string $organization_id, array $project_ids, string $value)
    {
        $secrets_create_request = new SecretCreateRequest();
        $secrets_create_request->organizationId = $organization_id;
        $secrets_create_request->projectIds = $project_ids;
        $secrets_create_request->key = $key;
        $secrets_create_request->note = $note;
        $secrets_create_request->value = $value;
        $secrets_create_request->validate();
        $secrets_command = new SecretsCommand();
        $secrets_command->get_by_ids = $secrets_create_request;
        $secrets_command->validate();
        $command = new Command();
        $command->secrets = $secrets_command;
        $command->validate();
        $this->commandRunner->run($command);
    }

    public function update(string $id, string $key, string $note, string $organization_id, array $project_ids, string $value)
    {
        $secrets_put_request = new SecretPutRequest();
        $secrets_put_request->id = $id;
        $secrets_put_request->organizationId = $organization_id;
        $secrets_put_request->projectIds = $project_ids;
        $secrets_put_request->key = $key;
        $secrets_put_request->note = $note;
        $secrets_put_request->value = $value;
        $secrets_put_request->validate();
        $secrets_command = new SecretsCommand();
        $secrets_command->get_by_ids = $secrets_put_request;
        $secrets_command->validate();
        $command = new Command();
        $command->secrets = $secrets_command;
        $command->validate();
        $this->commandRunner->run($command);
    }

    public function delete(array $secrets_ids)
    {
        $secrets_delete_request = new SecretsDeleteRequest();
        $secrets_delete_request->ids = $secrets_ids;
        $secrets_delete_request->validate();
        $secrets_command = new SecretsCommand();
        $secrets_command->get_by_ids = $secrets_delete_request;
        $secrets_command->validate();
        $command = new Command();
        $command->secrets = $secrets_command;
        $command->validate();
        $this->commandRunner->run($command);
    }
}
