## Bitwarden Secrets Manager SDK

Node-API bindings for interacting with the Bitwarden Secrets Manager. This is a beta release and
might be missing some functionality.

## Getting started

```ts
import { BitwardenClient, ClientSettings, DeviceType, LogLevel } from "@bitwarden/sdk-napi";

// Optional settings
const settings: ClientSettings = {
  apiUrl: "https://api.bitwarden.com",
  identityUrl: "https://identity.bitwarden.com",
  userAgent: "Bitwarden SDK",
  deviceType: DeviceType.SDK,
};

const accessToken = "-- REDACTED --";
const stateFile = "some/path/to/state/file";

const client = new BitwardenClient(settings, LogLevel.Info);

// Authenticating using a machine account access token
await client.auth().loginAccessToken(accessToken, stateFile);

// List secrets
const secrets = await client.secrets().list();

// Get a specific secret
const secret = await client.secrets().get("secret-id");
```
