<?php

require_once 'vendor/autoload.php';

$access_token = '<you access token here>';
$bitwarden_sdk = new \Bitwarden\Sdk\BitwardenSDK();
$res = $bitwarden_sdk->authorize($access_token);

// get project
$res = $bitwarden_sdk->projectsClient->get("00056058-cc70-4cd2-baea-b0810134a729");
// list projects
$res = $bitwarden_sdk->projectsClient->list('5688da1f-cc25-41d7-bb9f-b0740144ef1d');
// create project
$res = $bitwarden_sdk->projectsClient->create('php project', '5688da1f-cc25-41d7-bb9f-b0740144ef1d');
// update project
$res = $bitwarden_sdk->projectsClient->put('920fe206-ab3b-429d-a4b7-b0ac00e17acf', 'php project awesome', '5688da1f-cc25-41d7-bb9f-b0740144ef1d');
// delete project
$res = $bitwarden_sdk->projectsClient->delete(['920fe206-ab3b-429d-a4b7-b0ac00e17acf']);

// get secret
$res = $bitwarden_sdk->secretsClient->get("75d3a7ff-30ed-433a-91aa-b099016e4833");
// list secrets
$res = $bitwarden_sdk->secretsClient->list("5688da1f-cc25-41d7-bb9f-b0740144ef1d");
// create secret
$res = $bitwarden_sdk->secretsClient->create("New Key", "hello world", "5688da1f-cc25-41d7-bb9f-b0740144ef1d", ["b23818dd-827b-4a22-b97a-b07e010ae9d4"], "123");
// update secret
$res = $bitwarden_sdk->secretsClient->update("901d102d-af7d-46a1-99f5-b0a6017e2f07", "hello world 2", "hello", "5688da1f-cc25-41d7-bb9f-b0740144ef1d", ["b23818dd-827b-4a22-b97a-b07e010ae9d4"], "123");
// delete secret
$res = $bitwarden_sdk->secretsClient->delete(["380b5c30-d8fc-472d-a514-b0ac00f17071"]);
