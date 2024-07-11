<?php

require_once 'vendor/autoload.php';

$access_token = getenv('ACCESS_TOKEN');
$organization_id = getenv('ORGANIZATION_ID');

// Configuring the URLS is optional, set them to null to use the default values
$api_url = getenv('API_URL');
$identity_url = getenv('IDENTITY_URL');

$client_settings = new \Bitwarden\Sdk\BitwardenSettings($api_url, $identity_url);

$bitwarden_client = new \Bitwarden\Sdk\BitwardenClient($client_settings);
$bitwarden_client->login_access_token($access_token);

// create project
$res = $bitwarden_client->projects->create('php project', $organization_id);
$project_id = $res->id;

// get project
$res = $bitwarden_client->projects->get($project_id);

// list projects
$res = $bitwarden_client->projects->list($organization_id);

// update project
$res = $bitwarden_client->projects->put($project_id, 'php test awesome', $organization_id);

// create secret
$res = $bitwarden_client->secrets->create("New Key", "hello world", $organization_id, [$project_id], "123");
$secret_id = $res->id;

// get secret
$res = $bitwarden_client->secrets->get($secret_id);

// list secrets
$res = $bitwarden_client->secrets->list($organization_id);

// update secret
$res = $bitwarden_client->secrets->update($secret_id, "hello world 2", "hello", $organization_id, [$project_id], "123");

// delete secret
$res = $bitwarden_client->secrets->delete([$secret_id]);

// delete project
$res = $bitwarden_client->projects->delete([$project_id]);
