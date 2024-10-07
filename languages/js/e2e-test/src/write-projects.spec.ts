import { BitwardenClient, SecretIdentifierResponse, ProjectResponse } from "@bitwarden/sdk-client";
import { env } from "./data-manipulation";
import { filterProjectsToThisRun, filterSecretsToThisRun, withRunId } from "./data-manipulation";
import { TestSecret, TestProject, getTestData } from "./test-data";
import { initClients, tearDown } from "./test.setup";
import { forceDefined } from "./type-guards";

describe("write projects", () => {
  let mutableClient: BitwardenClient;
  let projects: ProjectResponse[];

  beforeAll(async () => {
    const clients = await initClients();
    mutableClient = clients.mutableClient;

    // get projects
    const projectsResponse = await mutableClient.projects().list(env("ORGANIZATION_ID"));
    projects = filterProjectsToThisRun(projectsResponse);
  });

  afterAll(async () => {
    tearDown();
  });

  it("creates projects", async () => {
    const toCreate: TestProject = {
      name: withRunId("new project"),
    };

    const result = await mutableClient.projects().create(toCreate.name, env("ORGANIZATION_ID"));

    expect(result).toEqualProject(toCreate);
  });

  it("updates projects", async () => {
    const toUpdate = forceDefined(projects.find((p) => p.name === withRunId("to_update")));
    const newName = withRunId("updated project");

    const result = await mutableClient
      .projects()
      .update(toUpdate.id, newName, env("ORGANIZATION_ID"));

    expect(result).toEqualProject({ name: newName });
  });

  it("deletes projects", async () => {
    const toDelete = forceDefined(projects.find((p) => p.name === withRunId("to_delete")));

    const result = await mutableClient.projects().delete([toDelete.id]);

    const projectsResponse = await mutableClient.projects().list(env("ORGANIZATION_ID"));
    const updatedProjects = filterProjectsToThisRun(projectsResponse);

    expect(updatedProjects.map((p) => p.id)).not.toContain(toDelete.id);
    expect(result.data.map((r) => r.id)).toEqual([toDelete.id]);
  });
});
