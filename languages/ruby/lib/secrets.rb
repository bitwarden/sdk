# frozen_string_literal: true

class SecretsClient
  def initialize(command_runner)
    @command_runner = command_runner
  end

  def get(id)
    command = create_get_command(id)
    parse_response(command)
  end

  def get_by_ids(ids)
    command = create_get_by_ids_command(ids)
    parse_response(command)
  end

  def create(key, note, organization_id, project_ids, value)
    command = create_create_command(key, note, organization_id, project_ids, value)
    parse_response(command)
  end

  def list(organization_id)
    command = create_list_command(organization_id)
    parse_response(command)
  end

  def update(id, key, note, organization_id, project_ids, value)
    command = create_update_command(id, key, note, organization_id, project_ids, value)
    parse_response(command)
  end

  def delete_secret(ids)
    command = create_delete_command(ids)
    parse_response(command)
  end

  private

  def create_get_command(id)
    SelectiveCommand.new(secrets: SelectiveSecretsCommand.new(get: SecretGetRequest.new(id: id)))
  end

  def create_get_by_ids_command(ids)
    SelectiveCommand.new(secrets: SelectiveSecretsCommand.new(get_by_ids: SecretsGetRequest.new(ids: ids)))
  end

  def create_create_command(key, note, organization_id, project_ids, value)
    SelectiveCommand.new(
      secrets: SelectiveSecretsCommand.new(
        create: SecretCreateRequest.new(
          key: key,
          note: note,
          organization_id: organization_id,
          project_ids: project_ids,
          value: value
        )
      )
    )
  end

  def create_list_command(organization_id)
    SelectiveCommand.new(secrets: SelectiveSecretsCommand.new(list: SecretIdentifiersRequest.new(organization_id: organization_id)))
  end

  def create_update_command(id, key, note, organization_id, project_ids, value)
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
  end

  def create_delete_command(ids)
    SelectiveCommand.new(secrets: SelectiveSecretsCommand.new(delete: SecretsDeleteRequest.new(ids: ids)))
  end

  def parse_response(command)
    response = ResponseForProjectResponse.from_json!(@command_runner.run(command))
    response.to_dynamic
  end
end
