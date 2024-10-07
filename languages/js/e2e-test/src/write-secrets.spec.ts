import { BitwardenClient, SecretIdentifierResponse, ProjectResponse } from "@bitwarden/sdk-client";
import { env, withRunId } from "./data-manipulation";
import { filterProjectsToThisRun, filterSecretsToThisRun } from "./data-manipulation";
import { TestSecret, TestProject, getTestData } from "./test-data";
import { initClients, tearDown } from "./test.setup";
import { forceDefined } from "./type-guards";

describe("write secrets", () => {
  let mutableClient: BitwardenClient;
  let secrets: SecretIdentifierResponse[];
  let projects: ProjectResponse[];
  let writeProject: ProjectResponse;

  const writeProjectName = "for_write_tests";

  beforeAll(async () => {
    const clients = await initClients();
    mutableClient = clients.mutableClient;

    // get projects
    const projectsResponse = await mutableClient.projects().list(env("ORGANIZATION_ID"));
    projects = filterProjectsToThisRun(projectsResponse);
    writeProject = forceDefined(projects.find((p) => p.name === withRunId(writeProjectName)));

    // get secrets
    const secretsResponse = await mutableClient.secrets().list(env("ORGANIZATION_ID"));
    secrets = filterSecretsToThisRun(secretsResponse);
  });

  afterAll(async () => {
    tearDown();
  });

  it("creates secrets", async () => {
    const toCreate: TestSecret = {
      key: withRunId("new secret"),
      value: "new secret value",
      note: "new secret note",
      project_name: writeProjectName,
      project_id: writeProject.id,
    };

    const result = await mutableClient
      .secrets()
      .create(
        toCreate.key,
        toCreate.value,
        toCreate.note,
        [toCreate.project_id!],
        env("ORGANIZATION_ID"),
      );

    expect(result).toEqualSecret(toCreate);
  });

  it("updates secrets", async () => {
    const toUpdate = forceDefined(secrets.find((s) => s.key === withRunId("to_update")));
    const newValues: TestSecret = {
      key: withRunId("to_update"),
      value: "new value",
      note: "new note",
      project_name: writeProjectName,
      project_id: writeProject.id,
    };

    const result = await mutableClient
      .secrets()
      .update(
        toUpdate.id,
        newValues.key,
        newValues.value,
        newValues.note,
        [newValues.project_id!],
        env("ORGANIZATION_ID"),
      );

    expect(result).toEqualSecret(newValues);
  });

  it("deletes secrets", async () => {
    const toDelete = forceDefined(secrets.find((s) => s.key === withRunId("to_delete")));

    const result = await mutableClient.secrets().delete([toDelete.id]);

    const secretsResponse = await mutableClient.secrets().list(env("ORGANIZATION_ID"));
    const remainingSecrets = filterSecretsToThisRun(secretsResponse);

    expect(remainingSecrets.find((s) => s.id === toDelete.id)).toBeUndefined();
    expect(result.data.map((r) => r.id)).toEqual([toDelete.id]);
  });
});
