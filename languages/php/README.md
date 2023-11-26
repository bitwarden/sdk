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
You can initialize ClientSettings and its setting before passing it to the BitwardenClient.

```php
$client_settings = new \Bitwarden\Sdk\Schemas\ClientSettings()
$client_settings->apiUrl = getenv('API_URL') ?: 'https://api.bitwarden.com';
$client_settings->identityUrl = getenv('IDENTITY_URL') ?: 'https://identity.bitwarden.com';
$client_settings->userAgent = getenv('USER_AGENT') ?: 'SDK';
$client_settings->deviceType = getenv('DEVICE_TYPE') ?: 'SDK';
```

Authorization can be performed using access token like so:

```php
$access_token = '<you access token here>';
$bitwarden_client = new \Bitwarden\Sdk\BitwardenClient($client_settings);
$result = $bitwarden_client->access_token_login($access_token);
```

After successful authorization you can interact with client to manage your projects and secrets.
```php
$organization_id = "<your organization id here>";

$client_settings = new \Bitwarden\Sdk\Schemas\ClientSettings();

$bitwarden_client = new \Bitwarden\Sdk\BitwardenClient($client_settings);
$res = $bitwarden_client->access_token_login($access_token);

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
