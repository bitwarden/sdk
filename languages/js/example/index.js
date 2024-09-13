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

  await client.auth().loginAccessToken(process.env.ACCESS_TOKEN);
  const organization_id = process.env.ORGANIZATION_ID;

  // Project functions

  const project = await client.projects().create(organization_id, "project-name");
  console.log(project);

  const project_get = await client.projects().get(project.id);
  console.log(project_get);

  const projects = await client.projects().list(organization_id);
  console.log(projects.data);

  const updated_project = await client.projects().update(organization_id, project.id, "project-name-updated");
  console.log(updated_project);

  // Secret functions

  const secret = await client
    .secrets()
    .create(organization_id, "secret-key", "secret-value", "secret-note", [project.id]);
  console.log(secret);

  const secret_get = await client.secrets().get(secret.id);
  console.log(secret_get);

  const secrets = await client.secrets().list(organization_id);
  console.log(secrets.data);

  const secrets_by_ids = await client.secrets().getByIds([secret.id]);
  console.log(secrets_by_ids.data);

  const updated_secret = await client.secrets().update(organization_id, secret.id, "secret-key-updated", "secret-value-updated", "secret-note-updated", [project.id]);
  console.log(updated_secret);

  const now = new Date();
  const secret_sync = await client.secrets().sync(organization_id, now);
  console.log(secret_sync.hasChanges);

  // Delete functions

  await client.secrets().delete([secret.id]);
  await client.projects().delete([project.id]);
}

main();
