package com.bitwarden.sdk;

import com.bitwarden.sdk.schema.*;

import java.util.function.Function;

public class AuthClient {

    private final CommandRunner commandRunner;

    AuthClient(CommandRunner commandRunner) {
        this.commandRunner = commandRunner;
    }

    public APIKeyLoginResponse loginAccessToken(String accessToken, String stateFile) {
        Command command = new Command();
        AccessTokenLoginRequest accessTokenLoginRequest = new AccessTokenLoginRequest();
        accessTokenLoginRequest.setAccessToken(accessToken);
        accessTokenLoginRequest.setStateFile(stateFile);

        command.setLoginAccessToken(accessTokenLoginRequest);

        ResponseForAPIKeyLoginResponse response = commandRunner.runCommand(command,
            throwingFunctionWrapper(Converter::ResponseForAPIKeyLoginResponseFromJsonString));

        if (response == null || !response.getSuccess()) {
            throw new BitwardenClientException(response != null ? response.getErrorMessage() : "Login failed");
        }

        return response.getData();
    }

    static <T, R> Function<T, R> throwingFunctionWrapper(ThrowingFunction<T, R, Exception> throwingFunction) {
        return i -> {
            try {
                return throwingFunction.accept(i);
            } catch (Exception ex) {
                throw new BitwardenClientException("Response deserialization failed");
            }
        };
    }
}
