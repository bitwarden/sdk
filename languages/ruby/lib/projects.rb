# frozen_string_literal: true

require 'json'

class ProjectsClient
  def initialize(command_runner)
    @command_runner = command_runner
  end

  def create_project(project_name, organization_id)
    command = create_command(
      create: ProjectCreateRequest.new(
        project_create_request_name: project_name,
        organization_id: organization_id
      )
    )
    parse_response(command)
  end

  def get(project_id)
    command = create_command(get: ProjectGetRequest.new(id: project_id))
    parse_response(command)
  end

  def list_projects(organization_id)
    command = create_command(list: ProjectsListRequest.new(organization_id: organization_id))
    parse_response(command)
  end

  def update_project(id, project_put_request_name, organization_id)
    command = create_command(
      update: ProjectPutRequest.new(
        id: id,
        project_put_request_name: project_put_request_name,
        organization_id: organization_id
      )
    )
    parse_response(command)
  end

  def delete_projects(ids)
    command = create_command(delete: ProjectsDeleteRequest.new(ids: ids))
    parse_response(command)
  end

  private

  def create_command(commands)
    SelectiveCommand.new(projects: SelectiveProjectsCommand.new(commands))
  end

  def parse_response(command)
    response = ResponseForProjectResponse.from_json!(@command_runner.run(command))
    response.to_dynamic
  end
end
