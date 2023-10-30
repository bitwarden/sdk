<?php

namespace Bitwarden;

use Bitwarden\Sdk\BitwardenSDK;

$access_token = '0.1de15dbd-1086-45c0-8af3-b0a6018b4bb3.GvOp1KDw0dPPLTKD9nkHyDielyilac:LwQPIiZ02j8XR/Wz06fA0A==';
$bitwarden_sdk = new BitwardenSDK();
$bitwarden_sdk->authorize($access_token);
