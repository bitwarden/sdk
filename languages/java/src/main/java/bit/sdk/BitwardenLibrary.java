package bit.sdk;

import com.sun.jna.Library;
import com.sun.jna.Pointer;

public interface BitwardenLibrary extends Library {

    Pointer init(String clientSettings);

    void free_mem(Pointer client);

    String run_command(String command, Pointer client);
}
