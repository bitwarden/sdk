package bit.sdk;

import bit.sdk.schema.AccessTokenLoginRequest;
import bit.sdk.schema.ClientSettings;
import bit.sdk.schema.Command;
import bit.sdk.schema.Converter;
import bit.sdk.schema.ResponseForAPIKeyLoginResponse;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.sun.jna.Native;
import com.sun.jna.Pointer;
import java.util.function.Function;

public class BitwardenClient implements AutoCloseable {

    static <T, R> Function<T, R> throwingFunctionWrapper(ThrowingFunction<T, R, Exception> throwingFunction) {

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

    private CommandRunner commandRunner;

    private boolean isClientOpen;

    private Projects projects;

    private Secrets secrets;

    public BitwardenClient(ClientSettings clientSettings) throws JsonProcessingException {
        library = Native.load("bitwarden_c", BitwardenLibrary.class);
        client = library.init(Converter.ClientSettingsToJsonString(clientSettings));
        commandRunner = new CommandRunner(library, client);
        projects = new Projects(commandRunner);
        secrets = new Secrets(commandRunner);
        isClientOpen = true;
    }

    public ResponseForAPIKeyLoginResponse accessTokenLogin(String accessToken) {
        Command command = new Command();
        AccessTokenLoginRequest accessTokenLoginRequest = new AccessTokenLoginRequest();
        accessTokenLoginRequest.setAccessToken(accessToken);
        command.setAccessTokenLogin(accessTokenLoginRequest);
        return commandRunner.runCommand(command,
            throwingFunctionWrapper(Converter::ResponseForAPIKeyLoginResponseFromJsonString));
    }

    public Projects projects() {
        return projects;
    }

    public Secrets secrets() {
        return secrets;
    }

    @Override
    public void close() {
        if (isClientOpen) {
            library.free_mem(client);
            isClientOpen = false;
        }
    }
}
