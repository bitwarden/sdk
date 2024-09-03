<?php

require_once 'vendor/autoload.php';

$access_token = getenv('ACCESS_TOKEN');
$state_file = getenv('STATE_FILE');
$organization_id = getenv('ORGANIZATION_ID');

// Configuring the URLS is optional, set them to null to use the default values
$api_url = getenv('API_URL');
$identity_url = getenv('IDENTITY_URL');

$client_settings = new \Bitwarden\Sdk\BitwardenSettings($api_url, $identity_url);

$bitwarden_client = new \Bitwarden\Sdk\BitwardenClient($client_settings);
try {
    $bitwarden_client->login_access_token($access_token, $state_file);
} catch (Exception $e) {
    print("Error: " . $e->getMessage() . "\n");
    exit(1);
}

// create project
print("Projects:\n");
$res = $bitwarden_client->projects->create('php project', $organization_id);
$project_id = $res->id;
print("\tcreate: '" . $project_id . "'\n");

// get project
$res = $bitwarden_client->projects->get($project_id);
print("\tget: '" . $res->name . "'\n");

// list projects
$res = $bitwarden_client->projects->list($organization_id);
print("\tlist:\n");
foreach ($res->data as $project) {
    print("\t\tID: '" . $project->id . "', Name: '" . $project->name . "'\n");
}

// update project
$res = $bitwarden_client->projects->put($project_id, 'php test awesome', $organization_id);
print("\tupdate: '" . $res->name . "'\n\n");

// create secret
print("Secrets:\n");
$res = $bitwarden_client->secrets->create("New Key", "hello world", $organization_id, [$project_id], "123");
$secret_id = $res->id;
print("\tcreate: '" . $secret_id . "'\n");

// get secret
$res = $bitwarden_client->secrets->get($secret_id);
print("\tget: '" . $res->key . "'\n");

// list secrets
$res = $bitwarden_client->secrets->list($organization_id);
print("\tlist:\n");
foreach ($res->data as $secret) {
    print("\t\tID: '" . $secret->id . "', Name: '" . $secret->key . "'\n");
}

// update secret
$res = $bitwarden_client->secrets->update($secret_id, "hello world 2", "hello", $organization_id, [$project_id], "123");
print("\tupdate: '" . $res->key . "'\n\n");

// delete secret
print("Cleaning up secrets and projects:\n");
$res = $bitwarden_client->secrets->delete([$secret_id]);
print("\tdelete:\n");
foreach ($res->data as $secret) {
    print("\t\tdeleted secret: '" . $secret->id . "'\n\n");
}

// delete project
$res = $bitwarden_client->projects->delete([$project_id]);
print("\tdelete:\n");
foreach ($res->data as $project) {
    print("\t\tdeleted project: '" . $project->id . "'\n\n");
}
