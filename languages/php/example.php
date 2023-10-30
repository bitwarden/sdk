<?php

require_once 'vendor/autoload.php';

$access_token = '';
$bitwarden_sdk = new \Bitwarden\Sdk\BitwardenSDK();
$bitwarden_sdk->authorize($access_token);
$bitwarden_sdk->projectsClient->get("00056058-cc70-4cd2-baea-b0810134a729");
