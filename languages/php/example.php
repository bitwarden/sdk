<?php

require_once 'vendor/autoload.php';

$access_token = '<you access token here>';
$organization_id = "<your organization id here>";

$client_settings = new \Bitwarden\Sdk\Schemas\ClientSettings();

$bitwarden_client = new \Bitwarden\Sdk\BitwardenClient($client_settings);
$res = $bitwarden_client->access_token_login($access_token);

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
