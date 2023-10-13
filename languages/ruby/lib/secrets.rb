# frozen_string_literal: true

class SecretsClient
  def initialize(command_runner)
    @command_runner = command_runner
  end

  def get(id)
    command = create_command(get: SecretGetRequest.new(id: id))
    response = parse_response(command)
    return response['data'] unless response['data'] == false || response.nil?

    raise BitwardenError 'Error getting secret' if response.nil?

    raise BitwardenError, response['errorMessage']
  end

  def get_by_ids(ids)
    command = create_command(get_by_ids: SecretsGetRequest.new(ids: ids))
    response = parse_response(command)
    return response['data'] unless response['data'] == false || response.nil?

    raise BitwardenError 'Error getting secrets' if response.nil?

    raise BitwardenError, response['errorMessage']
  end

  def create(key, note, organization_id, project_ids, value)
    command = create_command(
      create: SecretCreateRequest(
        key: key, note: note, organization_id: organization_id, project_ids: project_ids, value: value
      )
    )
    response = parse_response(command)
    return response['data'] unless response['data'] == false || response.nil?

    raise BitwardenError 'Error creating secret' if response.nil?

    raise BitwardenError, response['errorMessage']
  end

  def list(organization_id)
    command = create_command(list: SecretIdentifiersRequest.new(organization_id: organization_id))
    response = parse_response(command)
    return response['data'] unless response['data'] == false || response.nil?

    raise BitwardenError 'Error getting secrets list' if response.nil?

    raise BitwardenError, response['errorMessage']
  end

  def update(id, key, note, organization_id, project_ids, value)
    command = create_command(
      update: SecretPutRequest(
        id: id, key: key, note: note, organization_id: organization_id, project_ids: project_ids, value: value
      )
    )
    response = parse_response(command)
    return response['data'] unless response['data'] == false || response.nil?

    raise BitwardenError, 'Error updating secret' if response.nil?

    raise BitwardenError, response['errorMessage']
  end

  def delete_secret(ids)
    command = create_command(delete: SecretsDeleteRequest(ids: ids))
    response = parse_response(command)
    return response['data'] unless response['data'] == false || response.nil?

    raise BitwardenError, 'Error deleting secret' if response.nil?

    raise BitwardenError, response['errorMessage']
  end

  private

  def create_command(commands)
    SelectiveCommand.new(secrets: SelectiveSecretsCommand.new(commands))
  end

  def parse_response(command)
    response = @command_runner.run(command)
    raise BitwardenError, 'Error getting response' if response.nil?

    ResponseForSecretResponse.from_json!(response).to_dynamic
  end
end
