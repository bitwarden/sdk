package bit.sdk;

import static bit.sdk.BitwardenClient.throwingFunctionWrapper;

import bit.sdk.schema.Command;
import bit.sdk.schema.Converter;
import bit.sdk.schema.ResponseForSecretIdentifiersResponse;
import bit.sdk.schema.ResponseForSecretResponse;
import bit.sdk.schema.ResponseForSecretsDeleteResponse;
import bit.sdk.schema.SecretCreateRequest;
import bit.sdk.schema.SecretGetRequest;
import bit.sdk.schema.SecretIdentifiersRequest;
import bit.sdk.schema.SecretPutRequest;
import bit.sdk.schema.SecretsCommand;
import bit.sdk.schema.SecretsDeleteRequest;
import java.util.UUID;

public class Secrets {

    private CommandRunner commandRunner;

    Secrets(CommandRunner commandRunner) {
        this.commandRunner = commandRunner;
    }

    public ResponseForSecretResponse get(UUID id) {
        Command command = new Command();
        SecretsCommand secretsCommand = new SecretsCommand();
        SecretGetRequest secretGetRequest = new SecretGetRequest();
        secretGetRequest.setID(id);
        secretsCommand.setGet(secretGetRequest);
        command.setSecrets(secretsCommand);
        return commandRunner.runCommand(command,
            throwingFunctionWrapper(Converter::ResponseForSecretResponseFromJsonString));
    }

    public ResponseForSecretResponse create(String key, String value, String note, UUID organizationId,
        UUID[] projectIds) {
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
        return commandRunner.runCommand(command,
            throwingFunctionWrapper(Converter::ResponseForSecretResponseFromJsonString));
    }

    public ResponseForSecretResponse update(UUID id, String key, String value, String note, UUID organizationId,
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
        return commandRunner.runCommand(command,
            throwingFunctionWrapper(Converter::ResponseForSecretResponseFromJsonString));
    }

    public ResponseForSecretsDeleteResponse delete(UUID[] ids) {
        Command command = new Command();
        SecretsCommand secretsCommand = new SecretsCommand();
        SecretsDeleteRequest secretsDeleteRequest = new SecretsDeleteRequest();
        secretsDeleteRequest.setIDS(ids);
        secretsCommand.setDelete(secretsDeleteRequest);
        command.setSecrets(secretsCommand);
        return commandRunner.runCommand(command,
            throwingFunctionWrapper(Converter::ResponseForSecretsDeleteResponseFromJsonString));
    }

    public ResponseForSecretIdentifiersResponse list(UUID organizationId) {
        Command command = new Command();
        SecretsCommand secretsCommand = new SecretsCommand();
        SecretIdentifiersRequest secretIdentifiersRequest = new SecretIdentifiersRequest();
        secretIdentifiersRequest.setOrganizationID(organizationId);
        secretsCommand.setList(secretIdentifiersRequest);
        command.setSecrets(secretsCommand);
        return commandRunner.runCommand(command,
            throwingFunctionWrapper(Converter::ResponseForSecretIdentifiersResponseFromJsonString));
    }
}
