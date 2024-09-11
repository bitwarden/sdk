<?php

namespace Bitwarden\Sdk;

use Bitwarden\Sdk\Schemas\ClientSettings;
use Bitwarden\Sdk\Schemas\DeviceType;
use JsonException;

class BitwardenClient
{
    private BitwardenLib $bitwarden_lib;

    private ClientSettings $clientSettings;

    public ProjectsClient $projects;

    public SecretsClient $secrets;

    public AuthClient $auth;

    private CommandRunner $commandRunner;

    /**
     * @throws JsonException
     */
    public function __construct(BitwardenSettings $bitwardenSettings)
    {
        $this->clientSettings = new ClientSettings(apiUrl: $bitwardenSettings->get_api_url(),
            deviceType: DeviceType::$SDK, identityUrl: $bitwardenSettings->get_identity_url(),
            userAgent: "Bitwarden PHP-SDK");

        $this->bitwarden_lib = new BitwardenLib();
        $this->bitwarden_lib->init($this->clientSettings);

        $this->commandRunner = new CommandRunner($this->bitwarden_lib);
        $this->projects = new ProjectsClient($this->commandRunner);
        $this->secrets = new SecretsClient($this->commandRunner);
        $this->auth = new AuthClient($this->commandRunner);
    }

    public function __destruct()
    {
        $this->bitwarden_lib->free_mem();
    }
}
