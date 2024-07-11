<?php

namespace Bitwarden\Sdk;

use Bitwarden\Sdk\Schemas\AccessTokenLoginRequest;
use Bitwarden\Sdk\schemas\ClientSettings;
use Bitwarden\Sdk\Schemas\Command;
use FFI;
use Swaggest\JsonDiff\Exception;


class BitwardenClient
{
    private BitwardenLib $bitwarden_lib;

    private ClientSettings $clientSettings;

    public ProjectsClient $projects;

    public SecretsClient $secrets;

    private CommandRunner $commandRunner;

    private FFI\CData $handle;

    public function __construct(BitwardenSettings $bitwardenSettings)
    {
        $this->clientSettings = new ClientSettings();
        $this->clientSettings->apiUrl = $bitwardenSettings->get_api_url();
        $this->clientSettings->identityUrl = $bitwardenSettings->get_identity_url();
        $this->clientSettings->userAgent = "Bitwarden PHP-SDK";

        $this->bitwarden_lib = new BitwardenLib();
        $this->handle = $this->bitwarden_lib->init($this->clientSettings);

        $this->commandRunner = new CommandRunner($this->bitwarden_lib, $this->handle);
        $this->projects = new ProjectsClient($this->commandRunner);
        $this->secrets = new SecretsClient($this->commandRunner);
    }

    /**
     * @throws \Exception
     */
    public function login_access_token(string $access_token)
    {
        $access_token_request = new AccessTokenLoginRequest();
        $access_token_request->accessToken = $access_token;
        $command = new Command();
        $command->accessTokenLogin = $access_token_request->jsonSerialize();
        $result = $this->commandRunner->run($command);
        if (!isset($result->authenticated)) {
            throw new \Exception("Authorization error");
        }

        if ($result->authenticated == False) {
            throw new \Exception("Unauthorized");
        }
    }

    public function __destruct()
    {
        $this->bitwarden_lib->free_mem();
    }
}
