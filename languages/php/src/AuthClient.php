<?php

namespace Bitwarden\Sdk;

use Bitwarden\Sdk\Schemas\AccessTokenLoginRequest;
use Bitwarden\Sdk\Schemas\Command;
use Exception;

class AuthClient
{
    private CommandRunner $commandRunner;

    public function __construct(CommandRunner $commandRunner)
    {
        $this->commandRunner = $commandRunner;
    }

    /**
     * @throws Exception
     */
    public function login_access_token(string $access_token, ?string $state_file): void
    {
        $access_token_request = new AccessTokenLoginRequest($access_token, $state_file);
        $command = new Command(passwordLogin: null, apiKeyLogin: null, loginAccessToken: $access_token_request,
            getUserApiKey: null, fingerprint: null, sync: null, secrets: null, projects: null, generators: null);
        $result = $this->commandRunner->run($command);
        if (!isset($result->authenticated)) {
            throw new Exception("Authorization error");
        }

        if (!$result->authenticated) {
            throw new Exception("Unauthorized");
        }
    }
}
