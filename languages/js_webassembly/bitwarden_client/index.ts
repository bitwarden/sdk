import * as rust from "../pkg";
import { LoggingLevel } from "./logging_level";
import {
  ClientSettings,
  Convert,
  ResponseForPasswordLoginResponse,
  ResponseForSecretIdentifiersResponse,
  ResponseForSecretResponse,
  ResponseForSecretsDeleteResponse,
  ResponseForSyncResponse,
  ResponseForUserAPIKeyResponse,
} from "./schemas";

export class BitwardenClient {
  client: rust.BitwardenClient;

  constructor(settings?: ClientSettings, logging_level?: LoggingLevel) {
    const settings_json = settings == null ? null : Convert.clientSettingsToJson(settings);
    this.client = new rust.BitwardenClient(settings_json, logging_level ?? LoggingLevel.Info);
  }

  async login(email: string, password: string): Promise<ResponseForPasswordLoginResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        passwordLogin: {
          email: email,
          password: password,
        },
      })
    );

    return Convert.toResponseForPasswordLoginResponse(response);
  }

  async getUserApiKey(
    secret: string,
    isOtp: boolean = false
  ): Promise<ResponseForUserAPIKeyResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        getUserApiKey: {
          masterPassword: isOtp ? null : secret,
          otp: isOtp ? secret : null,
        },
      })
    );

    return Convert.toResponseForUserAPIKeyResponse(response);
  }


  async sync(
    excludeSubdomains: boolean = false
  ): Promise<ResponseForSyncResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        sync: {
          excludeSubdomains
        },
      })
    );

    return Convert.toResponseForSyncResponse(response);
  }

  get performance(): PerformanceClient {
    return new PerformanceClient(this.client);
  }

  secrets(): SecretsClient {
    return new SecretsClient(this.client);
  }
}

export class PerformanceClient {
  client: rust.BitwardenClient;

  constructor(client: rust.BitwardenClient) {
    this.client = client;
  }

  async encrypt(key: string, numOperations = 1000): Promise<void> {
    await this.client.run_command(
      Convert.commandToJson({
        performance: {
          encrypt: {
            key,
            numOperations,
          }
        },
      })
    );
  }

  async decrypt(cipherText:string, key: string, numOperations = 1000): Promise<void> {
    await this.client.run_command(
      Convert.commandToJson({
        performance: {
          decrypt: {
            cipherText,
            key,
            numOperations,
          }
        },
      })
    );
  }
}

export class SecretsClient {
  client: rust.BitwardenClient;

  constructor(client: rust.BitwardenClient) {
    this.client = client;
  }

  async get(
    id: string
  ): Promise<ResponseForSecretResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        secrets: {
          get: { id }
        },
      })
    );

    return Convert.toResponseForSecretResponse(response);
  }

  async create(
    key: string,
    note: string,
    organizationId: string,
    value: string,
  ): Promise<ResponseForSecretResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        secrets: {
          create: { key, note, organizationId, value }
        },
      })
    );

    return Convert.toResponseForSecretResponse(response);
  }

  async list(
    organizationId: string
  ): Promise<ResponseForSecretIdentifiersResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        secrets: {
          list: { organizationId }
        },
      })
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
    const response = await this.client.run_command(
      Convert.commandToJson({
        secrets: {
          update: { id, key, note, organizationId, value }
        },
      })
    );

    return Convert.toResponseForSecretResponse(response);
  }

  async delete(
    ids: string[]
  ): Promise<ResponseForSecretsDeleteResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        secrets: {
          delete: { ids }
        },
      })
    );

    return Convert.toResponseForSecretsDeleteResponse(response);
  }

}
