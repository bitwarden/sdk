# frozen_string_literal: true

require 'json'

class SecretsClient
  def initialize(command_runner)
    @command_runner = command_runner
  end

  def get(id)
    command = create_command(get: SecretGetRequest.new(id: id))
    response = run_command(command)

    secrets_response = ResponseForSecretResponse.from_json!(response).to_dynamic

    raise BitwardenError, secrets_response['errorMessage'] if secrets_response['errorMessage']

    secrets_response['data']
  end

  def get_by_ids(ids)
    command = create_command(get_by_ids: SecretsGetRequest.new(ids: ids))
    response = run_command(command)

    secrets_response = ResponseForSecretIdentifiersResponse.from_json!(response).to_dynamic

    raise BitwardenError, secrets_response['errorMessage'] if secrets_response['errorMessage']

    secrets_response['data']['data']
  end

  def create(key, note, organization_id, project_ids, value)
    command = create_command(
      create: SecretCreateRequest.new(
        key: key, note: note, organization_id: organization_id, project_ids: project_ids, value: value
      )
    )
    response = run_command(command)

    secrets_response = ResponseForSecretResponse.from_json!(response).to_dynamic

    raise BitwardenError, 'Error creating secret' if secrets_response['errorMessage']

    secrets_response['data']
  end

  def list(organization_id)
    command = create_command(list: SecretIdentifiersRequest.new(organization_id: organization_id))
    response = run_command(command)

    secrets_response = ResponseForSecretIdentifiersResponse.from_json!(response).to_dynamic

    raise BitwardenError, 'Error getting list of secrets' if secrets_response['errorMessage']

    secrets_response['data']['data']
  end

  def update(id, key, note, organization_id, project_ids, value)
    command = create_command(
      update: SecretPutRequest.new(
        id: id, key: key, note: note, organization_id: organization_id, project_ids: project_ids, value: value
      )
    )
    response = run_command(command)

    secrets_response = ResponseForSecretResponse.from_json!(response).to_dynamic

    raise BitwardenError, secrets_response['errorMessage'] if secrets_response['errorMessage']

    secrets_response['data']
  end

  def delete_secret(ids)
    command = create_command(delete: SecretsDeleteRequest.new(ids: ids))
    response = run_command(command)

    secrets_response = ResponseForSecretsDeleteResponse.from_json!(response).to_dynamic

    raise BitwardenError, secrets_response['errorMessage'] if secrets_response['errorMessage']

    secrets_response['data']['data']
  end

  private

  def create_command(commands)
    SelectiveCommand.new(secrets: SelectiveSecretsCommand.new(commands))
  end

  def run_command(command)
    response = @command_runner.run(command)
    raise BitwardenError, 'Error getting response' if response.nil?

    response
  end
end
