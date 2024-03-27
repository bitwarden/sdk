import {
  Convert,
  ProjectResponse,
  ProjectsDeleteResponse,
  ProjectsResponse,
  SecretIdentifiersResponse,
  SecretResponse,
  SecretsDeleteResponse,
} from "./schemas";

export interface Fido2VaultItem {
  cipherId: string;
  name: string;
}

// TODO: Temporary until I figure out how to decrypt EncString in the SDK
export interface Fido2CredentialView {
  credentialId: string,
  keyType: string,
  keyAlgorithm: string,
  keyCurve: string,
  keyValue: string,
  rpId: string,
  userHandle?: string,
  userName?: string,
  counter: string,
  rpName?: string,
  userDisplayName?: string,
  discoverable: string,
  creationDate: string,
}

export interface FindCredentialsParams {
  ids: Uint8Array[];
  rp_id: string;
}

export interface Fido2CredentialStore {
  findCredentials(params: FindCredentialsParams): Promise<Fido2VaultItem[]>;
  saveCredential(params: Fido2VaultItem): Promise<void>;
}

export interface Fido2NewCredentialParams {
  credential_name: string;
  user_name: string;
}

export interface Fido2ConfirmNewCredentialResult {
  vault_item: Fido2VaultItem;
}

export interface Fido2UserInterface {
  confirmNewCredential(params: Fido2NewCredentialParams): Promise<Fido2ConfirmNewCredentialResult>;
  pickCredential(params: unknown): Promise<Fido2VaultItem>;
  checkUserVerification(): Promise<boolean>;
  checkUserPresence(): Promise<boolean>;
  isPresenceEnabled(): boolean;
  isVerificationEnabled(): boolean | undefined;
}

export interface Fido2ClientCreateCredentialRequest {
  options: string;
  origin: string;
}

export interface Fido2CreatedPublicKeyCredential {
  id: string,
  rawId: Uint8Array,
  type: 'public-key',
  response: {
      clientDataJSON: Uint8Array,
      authenticatorData: Uint8Array,
      publicKey: Uint8Array,
      publicKeyAlgorithm: number,
      attestationObject: Uint8Array,
      transports: string[]
  },
  authenticatorAttachment: string,
  clientExtensionResults: {
      credProps: {
          rk: boolean
      }
  }
}

interface BitwardenSDKClient {
  run_command(js_input: string): Promise<any>;
  client_create_credential(
    webauthn_request: Fido2ClientCreateCredentialRequest,
    user_interface: Fido2UserInterface,
    credential_store: Fido2CredentialStore,
  ): Promise<Fido2CreatedPublicKeyCredential>;
}

function handleResponse<T>(response: { success: boolean; errorMessage?: string; data?: T }): T {
  if (!response.success) {
    throw new Error(response.errorMessage);
  }
  return response.data as T;
}

export class BitwardenClient {
  client: BitwardenSDKClient;

  constructor(client: BitwardenSDKClient) {
    this.client = client;
  }

  async fingerprint(fingerprintMaterial: string, publicKey: string): Promise<string> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        fingerprint: {
          fingerprintMaterial: fingerprintMaterial,
          publicKey: publicKey,
        },
      }),
    );

    return Convert.toResponseForFingerprintResponse(response).data.fingerprint;
  }

  async accessTokenLogin(accessToken: string): Promise<void> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        accessTokenLogin: {
          accessToken,
        },
      }),
    );

    handleResponse(Convert.toResponseForAccessTokenLoginResponse(response));
  }

  secrets(): SecretsClient {
    return new SecretsClient(this.client);
  }

  projects(): ProjectsClient {
    return new ProjectsClient(this.client);
  }
}

export class SecretsClient {
  client: BitwardenSDKClient;

  constructor(client: BitwardenSDKClient) {
    this.client = client;
  }

  async get(id: string): Promise<SecretResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        secrets: {
          get: { id },
        },
      }),
    );

    return handleResponse(Convert.toResponseForSecretResponse(response));
  }

  async create(
    key: string,
    value: string,
    note: string,
    projectIds: string[],
    organizationId: string,
  ): Promise<SecretResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        secrets: {
          create: { key, value, note, projectIds, organizationId },
        },
      }),
    );

    return handleResponse(Convert.toResponseForSecretResponse(response));
  }

  async list(organizationId: string): Promise<SecretIdentifiersResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        secrets: {
          list: { organizationId },
        },
      }),
    );

    return handleResponse(Convert.toResponseForSecretIdentifiersResponse(response));
  }

  async update(
    id: string,
    key: string,
    value: string,
    note: string,
    projectIds: string[],
    organizationId: string,
  ): Promise<SecretResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        secrets: {
          update: { id, key, value, note, projectIds, organizationId },
        },
      }),
    );

    return handleResponse(Convert.toResponseForSecretResponse(response));
  }

  async delete(ids: string[]): Promise<SecretsDeleteResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        secrets: {
          delete: { ids },
        },
      }),
    );

    return handleResponse(Convert.toResponseForSecretsDeleteResponse(response));
  }
}

export class ProjectsClient {
  client: BitwardenSDKClient;

  constructor(client: BitwardenSDKClient) {
    this.client = client;
  }

  async get(id: string): Promise<ProjectResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        projects: {
          get: { id },
        },
      }),
    );

    return handleResponse(Convert.toResponseForProjectResponse(response));
  }

  async create(name: string, organizationId: string): Promise<ProjectResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        projects: {
          create: { name, organizationId },
        },
      }),
    );

    return handleResponse(Convert.toResponseForProjectResponse(response));
  }

  async list(organizationId: string): Promise<ProjectsResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        projects: {
          list: { organizationId },
        },
      }),
    );

    return handleResponse(Convert.toResponseForProjectsResponse(response));
  }

  async update(id: string, name: string, organizationId: string): Promise<ProjectResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        projects: {
          update: { id, name, organizationId },
        },
      }),
    );

    return handleResponse(Convert.toResponseForProjectResponse(response));
  }

  async delete(ids: string[]): Promise<ProjectsDeleteResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        projects: {
          delete: { ids },
        },
      }),
    );

    return handleResponse(Convert.toResponseForProjectsDeleteResponse(response));
  }
}
