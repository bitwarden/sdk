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
    $bitwarden_client->auth->login_access_token($access_token, $state_file);
} catch (Exception $e) {
    print("Error: " . $e->getMessage() . "\n");
    exit(1);
}

// create project
print("Projects:\n");
$res = $bitwarden_client->projects->create($organization_id, 'php project');
$project_id = $res->id;
print("\tcreate: '" . $project_id . "'\n\n");

// get project
$res = $bitwarden_client->projects->get($project_id);
print("\tget: '" . $res->name . "'\n\n");

// list projects
$res = $bitwarden_client->projects->list($organization_id);
print("\tlist:\n");
foreach ($res->data as $project) {
    print("\t\tID: '" . $project->id . "', Name: '" . $project->name . "'\n");
}
print("\n");

// update project
$res = $bitwarden_client->projects->update($organization_id, $project_id, 'php test awesome');
print("\tupdate: '" . $res->name . "'\n\n");

// sync secrets
print("Secrets:\n");
print("\tSyncing secrets...\n");
$res = $bitwarden_client->secrets->sync($organization_id,null);
$now = new DateTime();
$now_string = $now->format('Y-m-d\TH:i:s.u\Z');
print("\t\tSync has changes: " . ($res->hasChanges ? 'true' : 'false') . "\n\n");

print("\tSyncing again to ensure no changes since last sync...\n");
$res = $bitwarden_client->secrets->sync($organization_id, $now_string);
print("\t\tSync has changes: " . ($res->hasChanges ? 'true' : 'false') . "\n\n");

// create secret
$res = $bitwarden_client->secrets->create($organization_id, "New Key", "New value", "New note", [$project_id]);
$secret_id = $res->id;
print("\tcreate: '" . $secret_id . "'\n\n");

// get secret
$res = $bitwarden_client->secrets->get($secret_id);
print("\tget: '" . $res->key . "'\n\n");

// get multiple secrets by ids
$res = $bitwarden_client->secrets->get_by_ids([$secret_id]);
print("\tget_by_ids:\n");
foreach ($res->data as $secret) {
    print("\t\tID: '" . $secret->id . "', Key: '" . $secret->key . "'\n");
}
print("\n");

// list secrets
$res = $bitwarden_client->secrets->list($organization_id);
print("\tlist:\n");
foreach ($res->data as $secret) {
    print("\t\tID: '" . $secret->id . "', Key: '" . $secret->key . "'\n");
}
print("\n");

// update secret
$res = $bitwarden_client->secrets->update($organization_id, $secret_id, "Updated key", "Updated value", "Updated note", [$project_id]);
print("\tupdate: '" . $res->key . "'\n\n");

// delete secret
print("Cleaning up secrets and projects:\n");
$res = $bitwarden_client->secrets->delete([$secret_id]);
print("\tdelete:\n");
foreach ($res->data as $secret) {
    print("\t\tdeleted secret: '" . $secret->id . "'\n");
}
print("\n");

// delete project
$res = $bitwarden_client->projects->delete([$project_id]);
print("\tdelete:\n");
foreach ($res->data as $project) {
    print("\t\tdeleted project: '" . $project->id . "'\n");
}
print("\n");
