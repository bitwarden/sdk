import { LoggingLevel } from "./bitwarden_client/logging_level";
import { DeviceType } from "./bitwarden_client/schemas";

import("./bitwarden_client").then(async (module) => {
  const client = new module.BitwardenClient({
    apiUrl: "http://localhost:8081/api",
    identityUrl: "http://localhost:8081/identity",
    deviceType: DeviceType.SDK,
    userAgent: "Bitwarden JS SDK",
  }, LoggingLevel.Debug);
  const result = await client.login("test@bitwarden.com", "asdfasdf");
  console.log(`auth result success: ${result.success}`);

  const apikeyResponse = await client.getUserApiKey("asdfasdf");
  console.log(`user API key: ${apikeyResponse.data.apiKey}`);

  const sync = await client.sync();
  console.log("Sync result", sync);

  const org_id = sync.data.profile.organizations[0].id;

  const secret = await client.secrets().create("TEST_KEY", "This is a test secret", org_id, "Secret1234!");
  console.log("New secret: ", secret.data);

  await client.secrets().delete([secret.data.id]);
});
