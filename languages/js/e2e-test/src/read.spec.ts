import {
  BitwardenClient,
  ProjectResponse,
  SecretIdentifierResponse,
  SecretIdentifiersRequest,
} from "@bitwarden/sdk-client";
import { initClients, tearDown } from "./test.setup";
import { env, filterProjectsToThisRun, filterSecretsToThisRun } from "./data-manipulation";
import { getTestData, type TestProject, type TestSecret } from "./test-data";
import { isNotDefined } from "./type-guards";

describe("read secrets", () => {
  let client: BitwardenClient;
  let secrets: SecretIdentifierResponse[];
  let projects: ProjectResponse[];
  let expectedSecrets: TestSecret[];
  let expectedProjects: TestProject[];

  beforeAll(async () => {
    const clients = await initClients();
    client = clients.client;

    // get projects
    const projectsResponse = await client.projects().list(env("ORGANIZATION_ID"));
    projects = filterProjectsToThisRun(projectsResponse);

    // get secrets
    const secretsResponse = await client.secrets().list(env("ORGANIZATION_ID"));
    secrets = filterSecretsToThisRun(secretsResponse);

    // read expected data
    const testData = getTestData({ projects: projectsResponse, mutable: false });
    expectedSecrets = testData.secrets;
    expectedProjects = testData.projects;
  });

  afterAll(async () => {
    tearDown();
  });

  it("reads secrets", () => {
    expect(secrets.length).toEqual(expectedSecrets.length);

    secrets.forEach((secret) => {
      const expectedSecret = expectedSecrets.find((s) => s.key === secret.key) as TestSecret;
      if (isNotDefined(expectedSecret)) {
        fail(`Secret not found: ${secret.key}`);
      }
      expect(secret).toEqualSecretIdentifier(expectedSecret);
    });
  });

  it("reads projects", () => {
    expect(projects.length).toEqual(expectedProjects.length);

    projects.forEach((project) => {
      const expectedProject = expectedProjects.find((p) => p.name === project.name) as TestProject;
      if (isNotDefined(expectedProject)) {
        fail(`Project not found: ${project.name}`);
      }
      expect(project).toEqualProject(expectedProject);
    });
  });

  it("gets a secret", async () => {
    secrets.forEach(async (secret) => {
      const secretResponse = await client.secrets().get(secret.id);
      const expectedSecret = expectedSecrets.find((s) => s.key === secret.key) as TestSecret;
      if (isNotDefined(expectedSecret)) {
        fail(`Secret not found: ${secret.key}`);
      }
      expect(secretResponse).toEqualSecret(expectedSecret);
    });
  });

  it("gets a project", async () => {
    projects.forEach(async (project) => {
      const projectResponse = await client.projects().get(project.id);
      const expectedProject = expectedProjects.find((p) => p.name === project.name) as TestProject;
      if (isNotDefined(expectedProject)) {
        fail(`Project not found: ${project.name}`);
      }
      expect(projectResponse).toEqualProject(expectedProject);
    });
  });
});
