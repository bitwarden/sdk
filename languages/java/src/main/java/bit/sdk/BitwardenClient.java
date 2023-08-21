package bit.sdk;

import bit.sdk.schema.APIKeyLoginRequest;
import bit.sdk.schema.AccessTokenLoginRequest;
import bit.sdk.schema.ClientSettings;
import bit.sdk.schema.Command;
import bit.sdk.schema.Converter;
import bit.sdk.schema.FingerprintRequest;
import bit.sdk.schema.PasswordLoginRequest;
import bit.sdk.schema.ResponseForAPIKeyLoginResponse;
import bit.sdk.schema.ResponseForFingerprintResponse;
import bit.sdk.schema.ResponseForPasswordLoginResponse;
import bit.sdk.schema.ResponseForSecretIdentifiersResponse;
import bit.sdk.schema.ResponseForSecretResponse;
import bit.sdk.schema.ResponseForSecretsDeleteResponse;
import bit.sdk.schema.ResponseForSyncResponse;
import bit.sdk.schema.ResponseForUserAPIKeyResponse;
import bit.sdk.schema.SecretCreateRequest;
import bit.sdk.schema.SecretGetRequest;
import bit.sdk.schema.SecretIdentifiersRequest;
import bit.sdk.schema.SecretPutRequest;
import bit.sdk.schema.SecretVerificationRequest;
import bit.sdk.schema.SecretsCommand;
import bit.sdk.schema.SecretsDeleteRequest;
import bit.sdk.schema.SyncRequest;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.sun.jna.Native;
import com.sun.jna.Pointer;
import java.util.UUID;
import java.util.function.Function;

public class BitwardenClient implements AutoCloseable {

    private static <T, R> Function<T, R> throwingFunctionWrapper(ThrowingFunction<T, R, Exception> throwingFunction) {

        return i -> {
            try {
                return throwingFunction.accept(i);
            } catch (Exception ex) {
                throw new RuntimeException(ex);
            }
        };
    }

    private Pointer client;

    private BitwardenLibrary library;

    private boolean isClientOpen;

    public BitwardenClient(ClientSettings clientSettings) throws JsonProcessingException {
        System.setProperty("jna.library.path", System.getProperty("user.dir") + "/libs");
        library = Native.load("bitwarden_c", BitwardenLibrary.class);
        client = library.init(Converter.ClientSettingsToJsonString(clientSettings));
        isClientOpen = true;
    }

    public ResponseForPasswordLoginResponse passwordLogin(String email, String password) {
        Command command = new Command();
        PasswordLoginRequest passwordLoginRequest = new PasswordLoginRequest();
        passwordLoginRequest.setEmail(email);
        passwordLoginRequest.setPassword(password);
        command.setPasswordLogin(passwordLoginRequest);
        return runCommand(command, throwingFunctionWrapper(Converter::ResponseForPasswordLoginResponseFromJsonString));
    }

    public ResponseForAPIKeyLoginResponse apiKeyLogin(String clientId, String clientSecret, String password) {
        Command command = new Command();
        APIKeyLoginRequest apiKeyLoginRequest = new APIKeyLoginRequest();
        apiKeyLoginRequest.setClientID(clientId);
        apiKeyLoginRequest.setClientSecret(clientSecret);
        apiKeyLoginRequest.setPassword(password);
        command.setAPIKeyLogin(apiKeyLoginRequest);
        return runCommand(command, throwingFunctionWrapper(Converter::ResponseForAPIKeyLoginResponseFromJsonString));
    }

    public ResponseForAPIKeyLoginResponse accessTokenLogin(String accessToken) {
        Command command = new Command();
        AccessTokenLoginRequest accessTokenLoginRequest = new AccessTokenLoginRequest();
        accessTokenLoginRequest.setAccessToken(accessToken);
        command.setAccessTokenLogin(accessTokenLoginRequest);
        return runCommand(command, throwingFunctionWrapper(Converter::ResponseForAPIKeyLoginResponseFromJsonString));
    }

    public ResponseForUserAPIKeyResponse userApiKey(String password) {
        Command command = new Command();
        SecretVerificationRequest secretVerificationRequest = new SecretVerificationRequest();
        secretVerificationRequest.setMasterPassword(password);
        command.setGetUserAPIKey(secretVerificationRequest);
        return runCommand(command, throwingFunctionWrapper(Converter::ResponseForUserAPIKeyResponseFromJsonString));
    }

    public ResponseForFingerprintResponse fingerprint(String fingerprintMaterial, String publicKey) {
        Command command = new Command();
        FingerprintRequest fingerprintRequest = new FingerprintRequest();
        fingerprintRequest.setFingerprintMaterial(fingerprintMaterial);
        fingerprintRequest.setPublicKey(publicKey);
        command.setFingerprint(fingerprintRequest);
        return runCommand(command, throwingFunctionWrapper(Converter::ResponseForFingerprintResponseFromJsonString));
    }

    public ResponseForSyncResponse sync(Boolean excludeSubdomains) {
        Command command = new Command();
        SyncRequest syncRequest = new SyncRequest();
        syncRequest.setExcludeSubdomains(excludeSubdomains);
        return runCommand(command, throwingFunctionWrapper(Converter::ResponseForSyncResponseFromJsonString));
    }

    public ResponseForSecretResponse getSecret(UUID id) {
        Command command = new Command();
        SecretsCommand secretsCommand = new SecretsCommand();
        SecretGetRequest secretGetRequest = new SecretGetRequest();
        secretGetRequest.setID(id);
        secretsCommand.setGet(secretGetRequest);
        command.setSecrets(secretsCommand);
        return runCommand(command, throwingFunctionWrapper(Converter::ResponseForSecretResponseFromJsonString));
    }

    public ResponseForSecretResponse createSecret(String key, String value, String note, UUID organizationId,
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
        return runCommand(command, throwingFunctionWrapper(Converter::ResponseForSecretResponseFromJsonString));
    }

    public ResponseForSecretResponse updateSecret(UUID id, String key, String value, String note, UUID organizationId,
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
        return runCommand(command, throwingFunctionWrapper(Converter::ResponseForSecretResponseFromJsonString));
    }

    public ResponseForSecretsDeleteResponse deleteSecrets(UUID[] ids) {
        Command command = new Command();
        SecretsCommand secretsCommand = new SecretsCommand();
        SecretsDeleteRequest secretsDeleteRequest = new SecretsDeleteRequest();
        secretsDeleteRequest.setIDS(ids);
        secretsCommand.setDelete(secretsDeleteRequest);
        command.setSecrets(secretsCommand);
        return runCommand(command, throwingFunctionWrapper(Converter::ResponseForSecretsDeleteResponseFromJsonString));
    }

    public ResponseForSecretIdentifiersResponse listSecrets(UUID organizationId) {
        Command command = new Command();
        SecretsCommand secretsCommand = new SecretsCommand();
        SecretIdentifiersRequest secretIdentifiersRequest = new SecretIdentifiersRequest();
        secretIdentifiersRequest.setOrganizationID(organizationId);
        secretsCommand.setList(secretIdentifiersRequest);
        command.setSecrets(secretsCommand);
        return runCommand(command,
            throwingFunctionWrapper(Converter::ResponseForSecretIdentifiersResponseFromJsonString));
    }

//    public ResponseForProjectResponse getProject(UUID id) {
//        Command command = new Command();
//        ProjectsCommand projectsCommand = new ProjectsCommand();
//        ProjectGetRequest projectGetRequest = new ProjectGetRequest();
//        projectGetRequest.setID(id);
//        projectsCommand.setGet(projectGetRequest);
//        command.setProjects(projectsCommand);
//        return runCommand(command, throwingFunctionWrapper(Converter::ResponseForProjectResponseFromJsonString));
//    }
//
//    public ResponseForProjectResponse createProject(UUID organizationId, String name) {
//        Command command = new Command();
//        ProjectsCommand projectsCommand = new ProjectsCommand();
//        ProjectCreateRequest projectCreateRequest = new ProjectCreateRequest();
//        projectCreateRequest.setOrganizationID(organizationId);
//        projectCreateRequest.setName(name);
//        projectsCommand.setCreate(projectCreateRequest);
//        command.setProjects(projectsCommand);
//        return runCommand(command, throwingFunctionWrapper(Converter::ResponseForProjectResponseFromJsonString));
//    }
//
//    public ResponseForProjectResponse updateProject(UUID id, UUID organizationId, String name) {
//        Command command = new Command();
//        ProjectsCommand projectsCommand = new ProjectsCommand();
//        ProjectPutRequest projectPutRequest = new ProjectPutRequest();
//        projectPutRequest.setID(id);
//        projectPutRequest.setOrganizationID(organizationId);
//        projectPutRequest.setName(name);
//        projectsCommand.setUpdate(projectPutRequest);
//        command.setProjects(projectsCommand);
//        return runCommand(command, throwingFunctionWrapper(Converter::ResponseForProjectResponseFromJsonString));
//    }
//
//    public ResponseForProjectsDeleteResponse deleteProjects(UUID[] ids) {
//        Command command = new Command();
//        ProjectsCommand projectsCommand = new ProjectsCommand();
//        ProjectsDeleteRequest projectsDeleteRequest = new ProjectsDeleteRequest();
//        projectsDeleteRequest.setIDS(ids);
//        projectsCommand.setDelete(projectsDeleteRequest);
//        command.setProjects(projectsCommand);
//        return runCommand(command, throwingFunctionWrapper(Converter::ResponseForProjectsDeleteResponseFromJsonString));
//    }
//
//    public ResponseForProjectsResponse listProjects(UUID organizationId) {
//        Command command = new Command();
//        ProjectsCommand projectsCommand = new ProjectsCommand();
//        ProjectsListRequest projectsListRequeste = new ProjectsListRequest();
//        projectsListRequeste.setOrganizationID(organizationId);
//        projectsCommand.setList(projectsListRequeste);
//        command.setProjects(projectsCommand);
//        return runCommand(command, throwingFunctionWrapper(Converter::ResponseForProjectsResponseFromJsonString));
//    }
//
//    public String mobileKdf(String email, String password, Kdf kdf) {
//        Command command = new Command();
//        MobileCommand mobileCommand = new MobileCommand();
//        MobileKdfCommand mobileKdfCommand = new MobileKdfCommand();
//        PasswordHashRequest passwordHashRequest = new PasswordHashRequest();
//        passwordHashRequest.setEmail(email);
//        passwordHashRequest.setPassword(password);
//        passwordHashRequest.setKdfParams(kdf);
//        mobileKdfCommand.setHashPassword(passwordHashRequest);
//        mobileCommand.setKdf(mobileKdfCommand);
//        command.setMobile(mobileCommand);
//        return runCommand(command, s -> s);
//    }
//
//    public String mobileCrypto(String email, String password, Kdf kdf, String userKey, String privateKey,
//        Map<String, String> organizationalKeys) {
//        Command command = new Command();
//        MobileCommand mobileCommand = new MobileCommand();
//        MobileCryptoCommand mobileCryptoCommand = new MobileCryptoCommand();
//        InitCryptoRequest initCryptoRequest = new InitCryptoRequest();
//        initCryptoRequest.setEmail(email);
//        initCryptoRequest.setPassword(password);
//        initCryptoRequest.setKdfParams(kdf);
//        initCryptoRequest.setUserKey(userKey);
//        initCryptoRequest.setPrivateKey(privateKey);
//        initCryptoRequest.setOrganizationKeys(organizationalKeys);
//        mobileCryptoCommand.setInitCrypto(initCryptoRequest);
//        mobileCommand.setCrypto(mobileCryptoCommand);
//        command.setMobile(mobileCommand);
//        return runCommand(command, s -> s);
//    }
//
//    public ResponseForFolderEncryptResponse mobileFolderEncrypt(UUID id) {
//        Command command = new Command();
//        MobileCommand mobileCommand = new MobileCommand();
//        MobileVaultCommand mobileVaultCommand = new MobileVaultCommand();
//        MobileFoldersCommand mobileFoldersCommand = new MobileFoldersCommand();
//        FolderEncryptRequest folderEncryptRequest = new FolderEncryptRequest();
//        FolderView folderView = new FolderView();
//        folderView.setID(id);
//        folderEncryptRequest.setFolder(folderView);
//        mobileFoldersCommand.setEncrypt(folderEncryptRequest);
//        mobileVaultCommand.setFolders(mobileFoldersCommand);
//        mobileCommand.setVault(mobileVaultCommand);
//        command.setMobile(mobileCommand);
//        return runCommand(command, throwingFunctionWrapper(Converter::ResponseForFolderEncryptResponseFromJsonString));
//    }
//
//    public ResponseForFolderDecryptResponse mobileFolderDecrypt(UUID id) {
//        Command command = new Command();
//        MobileCommand mobileCommand = new MobileCommand();
//        MobileVaultCommand mobileVaultCommand = new MobileVaultCommand();
//        MobileFoldersCommand mobileFoldersCommand = new MobileFoldersCommand();
//        FolderDecryptRequest folderDecryptRequest = new FolderDecryptRequest();
//        Folder folder = new Folder();
//        folder.setID(id);
//        folderDecryptRequest.setFolder(folder);
//        mobileFoldersCommand.setDecrypt(folderDecryptRequest);
//        mobileVaultCommand.setFolders(mobileFoldersCommand);
//        mobileCommand.setVault(mobileVaultCommand);
//        command.setMobile(mobileCommand);
//        return runCommand(command, throwingFunctionWrapper(Converter::ResponseForFolderDecryptResponseFromJsonString));
//    }
//
//    public ResponseForFolderDecryptListResponse mobileFolderDecryptList(UUID[] ids) {
//        Command command = new Command();
//        MobileCommand mobileCommand = new MobileCommand();
//        MobileVaultCommand mobileVaultCommand = new MobileVaultCommand();
//        MobileFoldersCommand mobileFoldersCommand = new MobileFoldersCommand();
//        FolderDecryptListRequest folderDecryptListRequest = new FolderDecryptListRequest();
//
//        Folder[] folders = (Folder[]) Arrays.stream(ids)
//            .map(id -> {
//                Folder folder = new Folder();
//                folder.setID(id);
//                return folder;
//            })
//            .collect(Collectors.toList()).toArray();
//        folderDecryptListRequest.setFolders(folders);
//        mobileFoldersCommand.setDecryptList(folderDecryptListRequest);
//        mobileVaultCommand.setFolders(mobileFoldersCommand);
//        mobileCommand.setVault(mobileVaultCommand);
//        command.setMobile(mobileCommand);
//        return runCommand(command,
//            throwingFunctionWrapper(Converter::ResponseForFolderDecryptListResponseFromJsonString));
//    }

    private <T> T runCommand(Command command, Function<String, T> deserializer) {
        String response = null;
        try {
            response = library.run_command(Converter.CommandToJsonString(command), client);
        } catch (JsonProcessingException e) {
            throw new RuntimeException(e);
        }
        return deserializer.apply(response);
    }

    @Override
    public void close() {
        if (isClientOpen) {
            library.free_mem(client);
            isClientOpen = false;
        }
    }
}
