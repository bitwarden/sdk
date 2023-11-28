<?php

namespace Bitwarden\Sdk;

class BitwardenSettings
{
    private ?string $api_url;

    private ?string $identity_url;

    public function __construct($api_url = null, $identity_url = null, $device_type = null)
    {
        $this->api_url = $api_url;
        $this->identity_url = $identity_url;
    }

    public function get_api_url(): ?string
    {
        return $this->api_url;
    }

    public function get_identity_url(): ?string
    {
        return $this->identity_url;
    }
}
