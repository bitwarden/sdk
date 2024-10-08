import { ProjectResponse } from "@bitwarden/sdk-client";
import { TestProject } from "../src/test-data";
import { env } from "../src/data-manipulation";
import { diff } from "jest-diff";

/** Matches the expected project with the actual one received from the SDK */
export const toEqualProject: jest.CustomMatcher = function (
  received: ProjectResponse,
  expected: TestProject,
) {
  return {
    pass: received.name === expected.name && received.organizationId === env("ORGANIZATION_ID"),
    message: () =>
      diff(expected, received, { expand: true }) ??
      `Projects are not equal.\n received: ${JSON.stringify(received)}\n expected: ${JSON.stringify(expected)}`,
  };
};
