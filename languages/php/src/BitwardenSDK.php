<?php

namespace Bitwarden\Sdk;

use Bitwarden\Sdk\Schemas\AccessTokenLoginRequest;
use Bitwarden\Sdk\schemas\ClientSettings;
use Bitwarden\Sdk\Schemas\Command;
use FFI;


class BitwardenSDK
{
    private BitwardenLib $bitwarden_lib;

    private ClientSettings $clientSettings;

    private ProjectsClient $projectsClient;

    private SecretsClient $secretsClient;

    private CommandRunner $commandRunner;

    private FFI\CData $handle;

    public function __construct()
    {
        $this->clientSettings = new ClientSettings();
        $this->clientSettings->apiUrl = getenv('API_URL') ?: 'https://api.bitwarden.com';
        $this->clientSettings->identityUrl = getenv('IDENTITY_URL') ?: 'https://identity.bitwarden.com';
        $this->clientSettings->userAgent = getenv('USER_AGENT') ?: 'SDK';
        $this->clientSettings->deviceType = getenv('DEVICE_TYPE') ?: 'SDK';

        $this->bitwarden_lib = new BitwardenLib();
        $this->handle = $this->bitwarden_lib->init($this->clientSettings);

        $this->commandRunner = new CommandRunner($this->bitwarden_lib, $this->handle);
        $this->projectsClient = new ProjectsClient($this->commandRunner);
        $this->secretsClient = new SecretsClient($this->commandRunner);
    }

    public function authorize(string $access_token)
    {
        $access_token_request = new AccessTokenLoginRequest();
        $access_token_request->accessToken = $access_token;
        $command = new Command();
        $command->access_token_request = $access_token_request;
        $this->commandRunner = new CommandRunner($this->bitwarden_lib, $this->handle);
        $this->commandRunner->run($command);
    }

    public function free_mem()
    {
        $this->bitwarden_lib->free_mem();
    }
}
