# frozen_string_literal: true

class SecretsClient
  def initialize(command_runner)
    @command_runner = command_runner
  end

  def get(id)
    @command_runner.run(
      SelectiveCommand.new(
        secrets: SelectiveSecretsCommand.new(
          get: SecretGetRequest.new(
            id: id
          )
        )
      )
    )
  end

  def get_by_ids(ids: Array[String])
    @command_runner.run(
      SelectiveCommand.new(
        secrets: SelectiveSecretsCommand.new(
          get_by_ids: SecretsGetRequest.new(
            ids: ids
          )
        )
      )
    )
  end

  def create(key: String, note: String, organization_id: String, project_ids: Array[String], value: String)
    @command_runner.run(
      SelectiveCommand.new(
        secrets: SelectiveSecretsCommand.new(
          create: SecretCreateRequest.new(
            key: key,
            note: note,
            organization_id: organization_id,
            project_ids: project_ids,
            value: value,
          )
        )
      )
    )
  end

  def list(organization_id: String)
    @command_runner.run(
      SelectiveCommand.new(
        secrets: SelectiveSecretsCommand.new(
          list: SecretIdentifiersRequest.new(
            organization_id: organization_id
          )
        )
      )
    )
  end

  def update(id: String, key: String, note: String, organization_id: String, project_ids: Array[String], value: String)
    @command_runner.run(
      SelectiveCommand.new(
        secrets: SelectiveSecretsCommand.new(
          update: SecretPutRequest.new(
            id: id,
            key: key,
            note: note,
            organization_id: organization_id,
            project_ids: project_ids,
            value: value
          )
        )
      )
    )
  end

  def delete_secret(ids: Array[String])
    @command_runner.run(
      SelectiveCommand.new(
        secrets: SelectiveSecretsCommand.new(
          delete: SecretsDeleteRequest.new(
            ids: ids
          )
        )
      )
    )
  end
end
