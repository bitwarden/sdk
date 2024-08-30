package com.bitwarden.sdk;

import com.bitwarden.sdk.schema.*;

import java.util.UUID;

public class SecretsClient {

    private final CommandRunner commandRunner;

    SecretsClient(CommandRunner commandRunner) {
        this.commandRunner = commandRunner;
    }

    public SecretResponse get(UUID id) {
        Command command = new Command();
        SecretsCommand secretsCommand = new SecretsCommand();
        SecretGetRequest secretGetRequest = new SecretGetRequest();
        secretGetRequest.setID(id);
        secretsCommand.setGet(secretGetRequest);
        command.setSecrets(secretsCommand);

        ResponseForSecretResponse response = commandRunner.runCommand(command,
            BitwardenClient.throwingFunctionWrapper(Converter::ResponseForSecretResponseFromJsonString));

        if (response == null || !response.getSuccess()) {
            throw new BitwardenClientException(response != null ? response.getErrorMessage() : "Secret not found");
        }

        return response.getData();
    }

    public SecretResponse create(String key, String value, String note, UUID organizationId, UUID[] projectIds) {
        Command command = new Command();
        SecretsCommand secretsCommand = new SecretsCommand();
        SecretCreateRequest secretCreateRequest = new SecretCreateRequest();
        secretCreateRequest.setKey(key);
        secretCreateRequest.setValue(value);
        secretCreateRequest.setNote(note);
        secretCreateRequest.setOrganizationID(organizationId);
        secretCreateRequest.setProjectIDS(projectIds);
        secretsCommand.setCreate(secretCreateRequest);
        command.setSecrets(secretsCommand);

        ResponseForSecretResponse response = commandRunner.runCommand(command,
            BitwardenClient.throwingFunctionWrapper(Converter::ResponseForSecretResponseFromJsonString));

        if (response == null || !response.getSuccess()) {
            throw new BitwardenClientException(response != null ? response.getErrorMessage() : "Secret create failed");
        }

        return response.getData();
    }

    public SecretResponse update(UUID id, String key, String value, String note, UUID organizationId,
                                 UUID[] projectIds) {
        Command command = new Command();
        SecretsCommand secretsCommand = new SecretsCommand();
        SecretPutRequest secretPutRequest = new SecretPutRequest();
        secretPutRequest.setID(id);
        secretPutRequest.setKey(key);
        secretPutRequest.setValue(value);
        secretPutRequest.setNote(note);
        secretPutRequest.setOrganizationID(organizationId);
        secretPutRequest.setProjectIDS(projectIds);
        secretsCommand.setUpdate(secretPutRequest);
        command.setSecrets(secretsCommand);

        ResponseForSecretResponse response = commandRunner.runCommand(command,
            BitwardenClient.throwingFunctionWrapper(Converter::ResponseForSecretResponseFromJsonString));

        if (response == null || !response.getSuccess()) {
            throw new BitwardenClientException(response != null ? response.getErrorMessage() : "Secret update failed");
        }

        return response.getData();
    }

    public SecretsDeleteResponse delete(UUID[] ids) {
        Command command = new Command();
        SecretsCommand secretsCommand = new SecretsCommand();
        SecretsDeleteRequest secretsDeleteRequest = new SecretsDeleteRequest();
        secretsDeleteRequest.setIDS(ids);
        secretsCommand.setDelete(secretsDeleteRequest);
        command.setSecrets(secretsCommand);

        ResponseForSecretsDeleteResponse response = commandRunner.runCommand(command,
            BitwardenClient.throwingFunctionWrapper(Converter::ResponseForSecretsDeleteResponseFromJsonString));

        if (response == null || !response.getSuccess()) {
            throw new BitwardenClientException(response != null ? response.getErrorMessage() : "Secrets delete failed");
        }

        return response.getData();
    }

    public SecretIdentifiersResponse list(UUID organizationId) {
        Command command = new Command();
        SecretsCommand secretsCommand = new SecretsCommand();
        SecretIdentifiersRequest secretIdentifiersRequest = new SecretIdentifiersRequest();
        secretIdentifiersRequest.setOrganizationID(organizationId);
        secretsCommand.setList(secretIdentifiersRequest);
        command.setSecrets(secretsCommand);

        ResponseForSecretIdentifiersResponse response = commandRunner.runCommand(command,
            BitwardenClient.throwingFunctionWrapper(Converter::ResponseForSecretIdentifiersResponseFromJsonString));

        if (response == null || !response.getSuccess()) {
            throw new BitwardenClientException(response != null ?
                response.getErrorMessage() : "No secrets for given organization");
        }

        return response.getData();
    }

    public SecretsResponse getByIds(UUID[] ids) {
        Command command = new Command();
        SecretsCommand secretsCommand = new SecretsCommand();
        SecretsGetRequest secretsGetRequest = new SecretsGetRequest();
        secretsGetRequest.setIDS(ids);
        secretsCommand.setGetByIDS(secretsGetRequest);
        command.setSecrets(secretsCommand);

        ResponseForSecretsResponse response = commandRunner.runCommand(command,
            BitwardenClient.throwingFunctionWrapper(Converter::ResponseForSecretsResponseFromJsonString));

        if (response == null || !response.getSuccess()) {
            throw new BitwardenClientException(response != null ? response.getErrorMessage() : "Secret not found");
        }

        return response.getData();
    }
}
