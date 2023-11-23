import * as rust from "../../binding";
import { LogLevel } from "../../binding";
import {
  ClientSettings,
  Convert,
  ResponseForAPIKeyLoginResponse,
  ResponseForSecretIdentifiersResponse,
  ResponseForSecretResponse,
  ResponseForSecretsDeleteResponse,
  ResponseForSecretsResponse,
} from "./schemas";

export class BitwardenClient {
  client: rust.BitwardenClient;

  constructor(settings?: ClientSettings, loggingLevel?: LogLevel) {
    const settingsJson = settings == null ? null : Convert.clientSettingsToJson(settings);
    this.client = new rust.BitwardenClient(settingsJson, loggingLevel ?? LogLevel.Info);
  }

  async loginWithAccessToken(accessToken: string): Promise<ResponseForAPIKeyLoginResponse> {
    const commandInput = Convert.commandToJson({
      accessTokenLogin: {
        accessToken: accessToken,
      },
    });
    const response = await this.client.runCommand(commandInput);

    return Convert.toResponseForAPIKeyLoginResponse(response);
  }

  /*
  async sync(excludeSubdomains = false): Promise<ResponseForSyncResponse> {
    const response = await this.client.runCommand(
      Convert.commandToJson({
        sync: {
          excludeSubdomains,
        },
      })
    );

    return Convert.toResponseForSyncResponse(response);
  }
  */

  secrets(): SecretsClient {
    return new SecretsClient(this.client);
  }
}

export class SecretsClient {
  client: rust.BitwardenClient;

  constructor(client: rust.BitwardenClient) {
    this.client = client;
  }

  async get(id: string): Promise<ResponseForSecretResponse> {
    const response = await this.client.runCommand(
      Convert.commandToJson({
        secrets: {
          get: { id },
        },
      }),
    );

    return Convert.toResponseForSecretResponse(response);
  }

  async getByIds(ids: string[]): Promise<ResponseForSecretsResponse> {
    const response = await this.client.runCommand(
      Convert.commandToJson({
        secrets: {
          getByIds: { ids },
        },
      }),
    );

    return Convert.toResponseForSecretsResponse(response);
  }

  async create(
    key: string,
    note: string,
    organizationId: string,
    value: string,
  ): Promise<ResponseForSecretResponse> {
    const response = await this.client.runCommand(
      Convert.commandToJson({
        secrets: {
          create: { key, note, organizationId, value },
        },
      }),
    );

    return Convert.toResponseForSecretResponse(response);
  }

  async list(organizationId: string): Promise<ResponseForSecretIdentifiersResponse> {
    const response = await this.client.runCommand(
      Convert.commandToJson({
        secrets: {
          list: { organizationId },
        },
      }),
    );

    return Convert.toResponseForSecretIdentifiersResponse(response);
  }

  async update(
    id: string,
    key: string,
    note: string,
    organizationId: string,
    value: string,
  ): Promise<ResponseForSecretResponse> {
    const response = await this.client.runCommand(
      Convert.commandToJson({
        secrets: {
          update: { id, key, note, organizationId, value },
        },
      }),
    );

    return Convert.toResponseForSecretResponse(response);
  }

  async delete(ids: string[]): Promise<ResponseForSecretsDeleteResponse> {
    const response = await this.client.runCommand(
      Convert.commandToJson({
        secrets: {
          delete: { ids },
        },
      }),
    );

    return Convert.toResponseForSecretsDeleteResponse(response);
  }
}
