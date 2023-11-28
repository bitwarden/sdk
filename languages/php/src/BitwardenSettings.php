<?php

namespace Bitwarden\Sdk;

class BitwardenSettings
{
    private string $api_url;

    private string $identity_url;

    private string $user_agent = "Bitwarden PHP SDK";

//    private string $device_type;

    public function __construct($api_url = null, $identity_url = null, $device_type = null)
    {
        if (is_null($api_url))
        {
            $this->api_url = 'https://api.bitwarden.com';
        }

        if (is_null($identity_url))
        {
            $this->identity_url = 'https://identity.bitwarden.com';
        }

//        $this->device_type = $device_type ? isset($device_type) : "";
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
