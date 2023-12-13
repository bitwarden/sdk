<?php

namespace Bitwarden\Sdk;

use Bitwarden\Sdk\Schemas\Command;
use Bitwarden\Sdk\Schemas\ProjectCreateRequest;
use Bitwarden\Sdk\Schemas\ProjectGetRequest;
use Bitwarden\Sdk\Schemas\ProjectPutRequest;
use Bitwarden\Sdk\Schemas\ProjectsCommand;
use Bitwarden\Sdk\Schemas\ProjectsDeleteRequest;
use Bitwarden\Sdk\Schemas\ProjectsListRequest;

class ProjectsClient
{
    private CommandRunner $commandRunner;

    public function __construct(CommandRunner $commandRunner)
    {
        $this->commandRunner = $commandRunner;
    }

    public function get(string $project_id): \stdClass
    {
        $project_get_request = new ProjectGetRequest();
        $project_get_request->id = $project_id;
        $project_get_request->validate();
        $project_command = new ProjectsCommand();
        $project_command->get = $project_get_request->jsonSerialize();
        return $this->run_project_command($project_command);
    }

    public function list(string $organization_id): \stdClass
    {
        $project_list_request = new ProjectsListRequest();
        $project_list_request->organizationId = $organization_id;
        $project_list_request->validate();
        $project_command = new ProjectsCommand();
        $project_command->list = $project_list_request->jsonSerialize();
        return $this->run_project_command($project_command);
    }

    public function create(string $project_name, string $organization_id): \stdClass
    {
        $project_create_request = new ProjectCreateRequest();
        $project_create_request->name = $project_name;
        $project_create_request->organizationId = $organization_id;
        $project_create_request->validate();
        $project_command = new ProjectsCommand();
        $project_command->create = $project_create_request->jsonSerialize();
        return $this->run_project_command($project_command);
    }

    public function put(string $project_id, string $project_name, string $organization_id): \stdClass
    {
        $project_put_request = new ProjectPutRequest();
        $project_put_request->organizationId = $organization_id;
        $project_put_request->name = $project_name;
        $project_put_request->id = $project_id;
        $project_put_request->validate();
        $project_command = new ProjectsCommand();
        $project_command->update = $project_put_request->jsonSerialize();
        return $this->run_project_command($project_command);
    }

    public function delete(array $ids): \stdClass
    {
        $projects_delete_request = new ProjectsDeleteRequest();
        $projects_delete_request->ids = $ids;
        $projects_delete_request->validate();
        $project_command = new ProjectsCommand();
        $project_command->delete = $projects_delete_request->jsonSerialize();
        return $this->run_project_command($project_command);
    }

    public function run_project_command($projectCommand): \stdClass
    {
        $command = new Command();
        $command->projects = $projectCommand;
        return $this->commandRunner->run($command);
    }
}
