import {
  Convert,
  ResponseForPasswordLoginResponse,
  ResponseForSecretIdentifiersResponse,
  ResponseForSecretResponse,
  ResponseForSecretsDeleteResponse,
  ResponseForSyncResponse,
  ResponseForUserAPIKeyResponse,
} from "./schemas";

interface BitwardenSDKClient {
  run_command(js_input: string): Promise<any>;
}

export class BitwardenClient {
  client: BitwardenSDKClient;

  constructor(client: BitwardenSDKClient) {
    this.client = client;
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

  async sync(excludeSubdomains: boolean = false): Promise<ResponseForSyncResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        sync: {
          excludeSubdomains,
        },
      })
    );

    return Convert.toResponseForSyncResponse(response);
  }

  async fingerprint(fingerprintMaterial: string, publicKey: string): Promise<string> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        fingerprint: {
          fingerprintMaterial: fingerprintMaterial,
          publicKey: publicKey,
        }
      })
    )

    return response;
  };
}

export class SecretsClient {
  client: BitwardenSDKClient;

  constructor(client: BitwardenSDKClient) {
    this.client = client;
  }

  async get(id: string): Promise<ResponseForSecretResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        secrets: {
          get: { id },
        },
      })
    );

    return Convert.toResponseForSecretResponse(response);
  }

  async create(
    key: string,
    note: string,
    organizationId: string,
    value: string
  ): Promise<ResponseForSecretResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        secrets: {
          create: { key, note, organizationId, value },
        },
      })
    );

    return Convert.toResponseForSecretResponse(response);
  }

  async list(organizationId: string): Promise<ResponseForSecretIdentifiersResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        secrets: {
          list: { organizationId },
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
    value: string
  ): Promise<ResponseForSecretResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        secrets: {
          update: { id, key, note, organizationId, value },
        },
      })
    );

    return Convert.toResponseForSecretResponse(response);
  }

  async delete(ids: string[]): Promise<ResponseForSecretsDeleteResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        secrets: {
          delete: { ids },
        },
      })
    );

    return Convert.toResponseForSecretsDeleteResponse(response);
  }
}
