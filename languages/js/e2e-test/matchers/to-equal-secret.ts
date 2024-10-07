import { SecretIdentifierResponse, SecretResponse } from "@bitwarden/sdk-client";
import { TestSecret } from "../src/test-data";
import { env } from "../src/data-manipulation";
import { diff } from "jest-diff";

/** Matches the expected secret with the actual one received from the SDK */
export const toEqualSecret: jest.CustomMatcher = function (
  received: SecretResponse,
  expected: TestSecret,
) {
  return {
    pass:
      received.key === expected.key &&
      received.value === expected.value &&
      received.note === expected.note &&
      received.organizationId === env("ORGANIZATION_ID") &&
      received.projectId === expected.project_id,
    message: () =>
      diff(expected, received, { expand: true }) ??
      `Secrets are not equal.\n received: ${JSON.stringify(received)}\n expected: ${JSON.stringify(expected)}`,
  };
};

export const toEqualSecretIdentifier: jest.CustomMatcher = function (
  received: SecretIdentifierResponse,
  expected: TestSecret,
) {
  return {
    pass: received.key === expected.key && received.organizationId === env("ORGANIZATION_ID"),
    message: () =>
      diff(expected, received, { expand: true }) ??
      `Secrets are not equal.\n received: ${JSON.stringify(received)}\n expected: ${JSON.stringify(expected)}`,
  };
};
