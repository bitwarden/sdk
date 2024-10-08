import { ProjectsResponse } from "@bitwarden/sdk-client";
import { env, withRunId } from "./data-manipulation";
import { readFileSync } from "fs";

type E2eData = {
  projects: TestProject[];
  mutable_projects: TestProject[];
  secrets: TestSecret[];
  mutable_secrets: TestSecret[];
};

export type TestProject = {
  name: string;
};

export type TestSecret = {
  key: string;
  value: string;
  note: string;
  project_name: string;
  project_id: string | undefined;
};

export function getTestData({
  mutable,
  projects,
}: {
  mutable: boolean;
  projects: ProjectsResponse | undefined;
}): E2eData {
  const testDataFile = env("TEST_DATA_FILE");
  const data = JSON.parse(readFileSync(testDataFile, "utf8")) as E2eData;

  data.projects.forEach((project) => {
    project.name = withRunId(project.name);
  });
  data.mutable_projects.forEach((project) => {
    project.name = withRunId(project.name);
  });
  data.secrets.forEach((secret) => {
    secret.key = withRunId(secret.key);
    secret.project_name = withRunId(secret.project_name);
    if (!mutable && projects) {
      secret.project_id = findProjectId(secret, projects);
    }
  });
  data.mutable_secrets.forEach((secret) => {
    secret.key = withRunId(secret.key);
    if (mutable && projects) {
      secret.project_id = findProjectId(secret, projects);
    }
  });

  return data;
}

function findProjectId(secret: TestSecret, projects: ProjectsResponse): string | undefined {
  if (!secret.project_name) {
    return undefined;
  }

  if (!projects?.data) {
    throw new Error("No projects found");
  }

  const project = projects.data.find((p) => p.name === secret.project_name);
  if (!project) {
    throw new Error(`Project not found: ${secret.project_name}`);
  }
  return project.id;
}
