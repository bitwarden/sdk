const { BitwardenClient: BitwardenClientWasm, LogLevel } = require("@bitwarden/sdk-wasm");
const sdk = require("@bitwarden/sdk-client");

async function main() {
  const settings = {
    apiUrl: process.env.API_URL,
    identityUrl: process.env.IDENTITY_URL,
  };

  const client = new sdk.BitwardenClient(
    new BitwardenClientWasm(JSON.stringify(settings), LogLevel.Debug),
  );

  await client.loginAccessToken(process.env.ACCESS_TOKEN, process.env.STATE_FILE);
  const organization_id = process.env.ORGANIZATION_ID;

  const project = await client.projects().create(organization_id, "project-name");
  const projects = await client.projects().list(organization_id);
  console.log(projects.data[0]);

  const project_get = await client.projects().get(project.id);
  console.log(project_get);

  const updated_project = await client.projects().update(organization_id, project.id, "updated-project-name");
  console.log(updated_project);

  const secret = await client
    .secrets()
    .create(organization_id, "test-secret", "test-value", "test-note", [project.id]);
  const secrets = await client.secrets().list(organization_id);
  console.log(secrets.data);

  const secret_sync_has_changes = await client.secrets().sync(organization_id, null);
  const now = new Date();
  console.log(secret_sync_has_changes.hasChanges);

  const updated_secret = await client.secrets().update(organization_id, secret.id, "updated-secret", "updated-value", "updated-note", [project.id]);
  console.log(updated_secret);

  const sync_has_changes_after_update = await client.secrets().sync(organization_id, now);
  console.log(sync_has_changes_after_update.hasChanges);

  // for secret.id in secrets.data, delete secret
  for (const secret of secrets.data) {
    await client.secrets().delete([secret.id]);
  }
  await client.projects().delete([project.id]);
}
main();
