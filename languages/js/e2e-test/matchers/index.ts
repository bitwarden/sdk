import { TestProject, TestSecret } from "../src/test-data";

export { toEqualSecret, toEqualSecretIdentifier } from "./to-equal-secret";
export { toEqualProject } from "./to-equal-project";

export interface CustomMatchers<R = unknown> {
  toEqualSecret(expected: TestSecret): R;
  toEqualSecretIdentifier(expected: TestSecret): R;
  toEqualProject(expected: TestProject): R;
}
