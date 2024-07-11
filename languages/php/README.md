# Bitwarden Secrets Manager SDK wrapper for PHP

PHP bindings for interacting with the [Bitwarden Secrets Manager]. This is a beta release and might be missing some functionality.
Supported are CRUD operations on project and secret entities.

## Installation

Requirements:
- PHP >= 8.0
- Composer
- Bitwarden C libraries which you can generate using BitwardenSDK and following instructions in its readme (requires Rust). https://github.com/bitwarden/sdk
If you are not using the standalone version of this library, file will be placed in `target/debug` folder if you are using from BitwardenSDK repository.
- Access token for the Bitwarden account


## Usage

To interact with the client first you need to obtain the access token from Bitwarden.
You can then initialize BitwardenSettings passing $api_url and $identity_url if needed. These parameteres are
optional and if they are not defined, BitwardenSettings instance will try to get these values from ENV, and
if they are not defined there as well, it will use defaults: `https://api.bitwarden.com` as api_url and
`https://identity.bitwarden.com` as identity_url. You can also pass device type as argument but that is entirely
optional.

Passing BitwardenSettings instance to BitwardenClient will initialize it. Before using the client you must
be authorized by calling the login_access_token method passing your Bitwarden access token to it.


```php
$access_token = '<your token here>';
$api_url = "<api url>";
$identity_url = "<identity url>";
$bitwarden_settings = new \Bitwarden\Sdk\BitwardenSettings($api_url, $identity_url);

$bitwarden_client = new \Bitwarden\Sdk\BitwardenClient($bitwarden_settings);
$bitwarden_client->login_access_token($access_token);
```

After successful authorization you can interact with client to manage your projects and secrets.
```php
$organization_id = "<your organization id here>";

$bitwarden_client = new \Bitwarden\Sdk\BitwardenClient($bitwarden_settings);
$res = $bitwarden_client->login_access_token($access_token);

// create project
$name = "PHP project"
$res = $bitwarden_client->projects->create($name, $organization_id);
$project_id = $res->id;

// get project
$res = $bitwarden_client->projects->get($project_id);

// list projects
$res = $bitwarden_client->projects->list($organization_id);

// update project
$name = "Updated PHP project"
$res = $bitwarden_client->projects->put($project_id, $name, $organization_id);

// get secret
$res = $bitwarden_client->secrets->get($secret_id);

// list secrets
$res = $bitwarden_client->secrets->list($organization_id);

// delete project
$res = $bitwarden_client->projects->delete([$project_id]);

```

Similarly, you interact with secrets:
```php
$organization_id = "<your organization id here>";

// create secret
$key = "AWS secret key";
$note = "Private account";
$secret = "76asaj,Is_)"
$res = $bitwarden_client->secrets->create($key, $note, $organization_id, [$project_id], $secret);
$secret_id = $res->id;

// get secret
$res = $bitwarden_sdk->secrets->get($secret_id);

// list secrets
$res = $bitwarden_client->secrets->list($organization_id);

// update secret
$note = "Updated account";
$key = "AWS private updated"
$secret = "7uYTE,:Aer"
$res = $bitwarden_client->secrets->update($secret_id, $key, $note, $organization_id, [$project_id], $secret);

// delete secret
$res = $bitwarden_sdk->secrets->delete([$secret_id]);
```


[Bitwarden Secrets Manager]: https://bitwarden.com/products/secrets-manager/
