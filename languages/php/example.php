<?php

namespace Bitwarden;

use Bitwarden\Sdk\BitwardenSDK;

$access_token = '<your access token goes here>';
$bitwarden_sdk = new BitwardenSDK();
$bitwarden_sdk->authorize($access_token);
