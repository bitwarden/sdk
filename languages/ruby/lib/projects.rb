# frozen_string_literal: true

class ProjectsClient
  def initialize(command_runner)
    @command_runner = command_runner
  end

  def create_project(project_name, organization_id)
    @command_runner.run(
      SelectiveCommand.new(
        projects:
          SelectiveProjectsCommand.new(
            create: ProjectCreateRequest.new(
              project_create_request_name: project_name,
              organization_id: organization_id
          )
        )
      )
    )
  end

  def get(project_id)
    @command_runner.run(
      SelectiveCommand.new(
        projects:
          SelectiveProjectsCommand.new(
            get: ProjectGetRequest.new(id: project_id)
          )
      )
    )
  end

  def list_projects(organization_id)
    @command_runner.run(
      SelectiveCommand.new(
        projects:
          SelectiveProjectsCommand.new(
            list: ProjectsListRequest.new(organization_id: organization_id
          )
        )
      )
    )
  end

  def update_project(id, project_put_request_name, organization_id)
    @command_runner.run(
      SelectiveCommand.new(
        projects:
          SelectiveProjectsCommand.new(
            update: ProjectPutRequest.new(
              id: id,
              project_put_request_name: project_put_request_name,
              organization_id: organization_id
          )
        )
      )
    )
  end

  def delete_projects(ids)
    @command_runner.run(
      SelectiveCommand.new(
        projects:
          SelectiveProjectsCommand.new(
            delete:
              ProjectsDeleteRequest.new(
                ids: ids
          )
        )
      )
    )
  end
end
