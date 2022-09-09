import { LoggingLevel } from "./bitwarden_client/logging_level";

import("./bitwarden_client").then(async (module) => {
  const client = new module.BitwardenClient(null, LoggingLevel.Debug);
  const result = await client.login("test@bitwarden.com", "asdfasdf");
  console.log(`auth result success: ${result.success}`);

  const apikeyResponse = await client.getUserApiKey("asdfasdf");
  console.log(`user API key: ${apikeyResponse.data.api_key}`);

  const sync = await client.sync();
  console.log("Sync result", sync);

  const org_id = sync.data.profile.organizations[0].id;

  const secret = await client.secrets().create("TEST_KEY", "This is a test secret", org_id, "Secret1234!");
  console.log("New secret: ", secret.data);

  await client.secrets().delete([secret.data.id]);
});
