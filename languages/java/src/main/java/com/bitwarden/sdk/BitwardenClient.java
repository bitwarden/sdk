package com.bitwarden.sdk;

import com.bitwarden.sdk.schema.*;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.sun.jna.Native;
import com.sun.jna.Pointer;

import java.util.function.Function;

public class BitwardenClient implements AutoCloseable {

    private final Pointer client;

    private final BitwardenLibrary library;

    private final CommandRunner commandRunner;

    private boolean isClientOpen;

    private final ProjectsClient projects;

    private final SecretsClient secrets;

    private final AuthClient auth;

    public BitwardenClient(BitwardenSettings bitwardenSettings) {
        ClientSettings clientSettings = new ClientSettings();
        clientSettings.setAPIURL(bitwardenSettings.getApiUrl());
        clientSettings.setIdentityURL(bitwardenSettings.getIdentityUrl());
        clientSettings.setDeviceType(DeviceType.SDK);
        clientSettings.setUserAgent("Bitwarden JAVA-SDK");

        library = Native.load("bitwarden_c", BitwardenLibrary.class);

        try {
            client = library.init(Converter.ClientSettingsToJsonString(clientSettings));
        } catch (JsonProcessingException e) {
            throw new BitwardenClientException("Error while processing client settings");
        }

        commandRunner = new CommandRunner(library, client);
        projects = new ProjectsClient(commandRunner);
        secrets = new SecretsClient(commandRunner);
        auth = new AuthClient(commandRunner);
        isClientOpen = true;
    }

    static <T, R> Function<T, R> throwingFunctionWrapper(ThrowingFunction<T, R, Exception> throwingFunction) {
        return i -> {
            try {
                return throwingFunction.accept(i);
            } catch (Exception ex) {
                throw new BitwardenClientException("Response deserialization failed", ex);
            }
        };
    }

    public ProjectsClient projects() {
        return projects;
    }

    public SecretsClient secrets() {
        return secrets;
    }

    public AuthClient auth() {
        return auth;
    }

    @Override
    public void close() {
        if (isClientOpen) {
            library.free_mem(client);
            isClientOpen = false;
        }
    }
}
