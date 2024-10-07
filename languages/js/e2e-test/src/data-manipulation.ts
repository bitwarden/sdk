import { ProjectsResponse, SecretIdentifiersResponse } from "@bitwarden/sdk-client";

export function env(key: string) {
  const value = process.env[key];
  if (!value) {
    throw new Error(`Missing environment variable: ${key}`);
  }
  return value;
}

export function withRunId(s: string) {
  return `${s}-${env("RUN_ID")}`;
}

export function filterProjectsToThisRun(projects: ProjectsResponse) {
  if (!projects?.data) {
    throw new Error("No projects found");
  }
  return projects.data.filter((project) => project.name.endsWith(env("RUN_ID")));
}

export function filterSecretsToThisRun(secrets: SecretIdentifiersResponse) {
  if (!secrets?.data) {
    throw new Error("No secrets found");
  }
  return secrets.data.filter((secret) => secret.key.endsWith(env("RUN_ID")));
}
