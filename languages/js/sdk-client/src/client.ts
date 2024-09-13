import {
  Convert,
  PasswordGeneratorRequest,
  ProjectResponse,
  ProjectsDeleteResponse,
  ProjectsResponse,
  SecretIdentifiersResponse,
  SecretResponse,
  SecretsDeleteResponse,
  SecretsSyncResponse,
  SecretsResponse,
} from "./schemas";

interface BitwardenSDKClient {
  run_command(js_input: string): Promise<any>;
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

  auth(): AuthClient {
    return new AuthClient(this.client);
  }

  secrets(): SecretsClient {
    return new SecretsClient(this.client);
  }

  projects(): ProjectsClient {
    return new ProjectsClient(this.client);
  }

  generators(): GeneratorsClient {
    return new GeneratorsClient(this.client);
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
    organizationId: string,
    key: string,
    value: string,
    note: string,
    projectIds: string[],
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
    organizationId: string,
    id: string,
    key: string,
    value: string,
    note: string,
    projectIds: string[],
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

  async sync(organizationId: string, lastSyncedDate: Date): Promise<SecretsSyncResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        secrets: {
          sync: { organizationId, lastSyncedDate },
        },
      }),
    );

    return handleResponse(Convert.toResponseForSecretsSyncResponse(response));
  }

  async getByIds(ids: string[]): Promise<SecretsResponse> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        secrets: {
          getByIds: { ids },
        },
      }),
    );

    return handleResponse(Convert.toResponseForSecretsResponse(response));
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

  async create(organizationId: string, name: string): Promise<ProjectResponse> {
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

  async update(organizationId: string, id: string, name: string): Promise<ProjectResponse> {
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

export class GeneratorsClient {
  client: BitwardenSDKClient;

  constructor(client: BitwardenSDKClient) {
    this.client = client;
  }

  async password(req: PasswordGeneratorRequest): Promise<string> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        generators: {
          generatePassword: req,
        },
      }),
    );

    return handleResponse(Convert.toResponseForString(response));
  }
}

export class AuthClient {
  client: BitwardenSDKClient;

  constructor(client: BitwardenSDKClient) {
    this.client = client;
  }

  async loginAccessToken(accessToken: string): Promise<void> {
    const response = await this.client.run_command(
      Convert.commandToJson({
        loginAccessToken: {
          accessToken,
        },
      }),
    );
    
    handleResponse(Convert.toResponseForAccessTokenLoginResponse(response));
  }
}
