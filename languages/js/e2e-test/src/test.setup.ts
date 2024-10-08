import { BitwardenClient, ClientSettings } from "@bitwarden/sdk-client";
import { BitwardenClient as BitwardenClientWasm } from "@bitwarden/wasm";
import { env } from "./data-manipulation";
import { existsSync, unlink } from "fs";
import { toEqualSecret, toEqualSecretIdentifier, toEqualProject } from "../matchers";

export async function initClients() {
  const clientSettings: ClientSettings = {
    apiUrl: env("API_URL"),
    identityUrl: env("IDENTITY_URL"),
  };
  const client: BitwardenClient = new BitwardenClient(
    new BitwardenClientWasm(JSON.stringify(clientSettings)),
  );
  const mutableClient: BitwardenClient = new BitwardenClient(
    new BitwardenClientWasm(JSON.stringify(clientSettings)),
  );

  // authenticate clients
  await client.accessTokenLogin(env("ACCESS_TOKEN"), "state.json");
  await mutableClient.accessTokenLogin(env("MUTABLE_ACCESS_TOKEN"), "mutable_state.json");

  return {
    client,
    mutableClient,
  };
}

export async function tearDown() {
  // clean up
  if (existsSync("state.json")) {
    unlink("state.json", () => {});
  }

  if (existsSync("mutable_state.json")) {
    unlink("mutable_state.json", () => {});
  }
}

expect.extend({
  toEqualSecret: toEqualSecret,
  toEqualSecretIdentifier: toEqualSecretIdentifier,
  toEqualProject: toEqualProject,
});
