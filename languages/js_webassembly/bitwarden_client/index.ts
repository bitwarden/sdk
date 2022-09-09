import * as rust from "../pkg/bitwarden_wasm";
import { LoggingLevel } from "./logging_level";
import {
  ClientSettings,
  Convert,
  PasswordLoginResponse,
  ResponseForPasswordLoginResponse,
  ResponseForSecretDeleteResponse,
  ResponseForSecretIdentifiersResponse,
  ResponseForSecretResponse,
  ResponseForSecretsDeleteResponse,
  ResponseForSyncResponse,
  ResponseForUserAPIKeyResponse,
  UserAPIKeyResponse,
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
        PasswordLogin: {
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
        GetUserApiKey: {
          master_password: isOtp ? null : secret,
          otp: isOtp ? secret : null,
        },
      })
    );

    return Convert.toResponseForUserAPIKeyResponse(response);
  }


  async sync(
    exclude_subdomains: boolean = false
  ): Promise<ResponseForSyncResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        Sync: {
          exclude_subdomains
        },
      })
    );

    return Convert.toResponseForSyncResponse(response);
  }

  secrets(): SecretsClient {
    return new SecretsClient(this.client);
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
        Secrets: {
          Get: { id }
        },
      })
    );

    return Convert.toResponseForSecretResponse(response);
  }

  async create(
    key: string,
    note: string,
    organization_id: string,
    value: string,
  ): Promise<ResponseForSecretResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        Secrets: {
          Create: { key, note, organization_id, value }
        },
      })
    );

    return Convert.toResponseForSecretResponse(response);
  }

  async list(
    organization_id: string
  ): Promise<ResponseForSecretIdentifiersResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        Secrets: {
          List: { organization_id }
        },
      })
    );

    return Convert.toResponseForSecretIdentifiersResponse(response);
  }

  async update(
    id: string,
    key: string,
    note: string,
    organization_id: string,
    value: string,
  ): Promise<ResponseForSecretResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        Secrets: {
          Update: { id, key, note, organization_id, value }
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
        Secrets: {
          Delete: { ids }
        },
      })
    );

    return Convert.toResponseForSecretsDeleteResponse(response);
  }

}
