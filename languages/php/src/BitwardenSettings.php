<?php

namespace Bitwarden\Sdk;

class BitwardenSettings
{
    private string $api_url;

    private string $identity_url;

    private string $user_agent = "Bitwarden PHP SDK";

    private string $device_type;

    public function __construct($api_url = null, $identity_url = null, $device_type = null)
    {
        if (is_null($api_url))
        {
            $this->api_url = getenv('API_URL') ?: 'https://api.bitwarden.com';
            $this->identity_url = getenv('IDENTITY_URL') ?: 'https://identity.bitwarden.com';
        } else {
            $this->api_url = $api_url;
        }

        if (is_null($identity_url))
        {
            $this->identity_url = getenv('IDENTITY_URL') ?: 'https://identity.bitwarden.com';
        } else {
            $this->identity_url = $identity_url;
        }

        $this->device_type = $device_type ? isset($device_type) : "";
    }

    public function get_api_url(): string
    {
        return $this->api_url;
    }

    public function get_identity_url(): string
    {
        return $this->identity_url;
    }

    public function get_user_agent(): string
    {
        return $this->user_agent;
    }

    public function get_device_type(): string
    {
        return $this->device_type;
    }
}
