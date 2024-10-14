import json
from typing import Any, List, Optional
from uuid import UUID
import bitwarden_py

from .schemas import (ClientSettings, Command, ResponseForSecretIdentifiersResponse, ResponseForSecretResponse,
                      ResponseForSecretsResponse, ResponseForSecretsDeleteResponse, SecretCreateRequest,
                      SecretGetRequest, SecretsGetRequest, SecretIdentifiersRequest, SecretPutRequest,
                      SecretsCommand, SecretsDeleteRequest, SecretsSyncRequest, AccessTokenLoginRequest,
                      ResponseForSecretsSyncResponse, ResponseForAccessTokenLoginResponse,
                      ResponseForProjectResponse, ProjectsCommand, ProjectCreateRequest, ProjectGetRequest,
                      ProjectPutRequest, ProjectsListRequest, ResponseForProjectsResponse,
                      ResponseForProjectsDeleteResponse, ProjectsDeleteRequest)


class BitwardenClient:
    def __init__(self, settings: ClientSettings = None):
        if settings is None:
            self.inner = bitwarden_py.BitwardenClient(None)
        else:
            settings_json = json.dumps(settings.to_dict())
            self.inner = bitwarden_py.BitwardenClient(settings_json)

    def auth(self):
        return AuthClient(self)

    def secrets(self):
        return SecretsClient(self)

    def projects(self):
        return ProjectsClient(self)

    def _run_command(self, command: Command) -> Any:
        response_json = self.inner.run_command(json.dumps(command.to_dict()))
        response = json.loads(response_json)

        if response["success"] == False:
            raise Exception(response["errorMessage"])

        return response


class AuthClient:
    def __init__(self, client: BitwardenClient):
        self.client = client

    def login_access_token(self, access_token: str,
                           state_file: str = None) -> ResponseForAccessTokenLoginResponse:
        result = self.client._run_command(
            Command(login_access_token=AccessTokenLoginRequest(access_token, state_file))
        )
        return ResponseForAccessTokenLoginResponse.from_dict(result)


class SecretsClient:
    def __init__(self, client: BitwardenClient):
        self.client = client

    def get(self, id: str) -> ResponseForSecretResponse:
        result = self.client._run_command(
            Command(secrets=SecretsCommand(get=SecretGetRequest(id)))
        )
        return ResponseForSecretResponse.from_dict(result)

    def get_by_ids(self, ids: List[UUID]) -> ResponseForSecretsResponse:
        result = self.client._run_command(
            Command(secrets=SecretsCommand(
                get_by_ids=SecretsGetRequest(ids))
            ))
        return ResponseForSecretsResponse.from_dict(result)

    def create(
            self,
            organization_id: UUID,
            key: str,
            value: str,
            note: Optional[str],
            project_ids: Optional[List[UUID]] = None,
    ) -> ResponseForSecretResponse:
        if note is None:
            # secrets api does not accept empty notes
            note = ""
        result = self.client._run_command(
            Command(secrets=SecretsCommand(
                create=SecretCreateRequest(key, note, organization_id, value, project_ids)))
        )
        return ResponseForSecretResponse.from_dict(result)

    def list(self, organization_id: str) -> ResponseForSecretIdentifiersResponse:
        result = self.client._run_command(
            Command(secrets=SecretsCommand(
                list=SecretIdentifiersRequest(organization_id)))
        )
        return ResponseForSecretIdentifiersResponse.from_dict(result)

    def update(
            self,
            organization_id: str,
            id: str,
            key: str,
            value: str,
            note: Optional[str],
            project_ids: Optional[List[UUID]] = None,
    ) -> ResponseForSecretResponse:
        if note is None:
            # secrets api does not accept empty notes
            note = ""
        result = self.client._run_command(
            Command(secrets=SecretsCommand(update=SecretPutRequest(
                id, key, note, organization_id, value, project_ids)))
        )
        return ResponseForSecretResponse.from_dict(result)

    def delete(self, ids: List[str]) -> ResponseForSecretsDeleteResponse:
        result = self.client._run_command(
            Command(secrets=SecretsCommand(delete=SecretsDeleteRequest(ids)))
        )
        return ResponseForSecretsDeleteResponse.from_dict(result)

    def sync(self, organization_id: str, last_synced_date: Optional[str]) -> ResponseForSecretsSyncResponse:
        result = self.client._run_command(
            Command(secrets=SecretsCommand(sync=SecretsSyncRequest(organization_id, last_synced_date)))
        )
        return ResponseForSecretsSyncResponse.from_dict(result)


class ProjectsClient:
    def __init__(self, client: BitwardenClient):
        self.client = client

    def get(self, id: str) -> ResponseForProjectResponse:
        result = self.client._run_command(
            Command(projects=ProjectsCommand(get=ProjectGetRequest(id)))
        )
        return ResponseForProjectResponse.from_dict(result)

    def create(self,
               organization_id: str,
               name: str,
               ) -> ResponseForProjectResponse:
        result = self.client._run_command(
            Command(projects=ProjectsCommand(
                create=ProjectCreateRequest(name, organization_id)))
        )
        return ResponseForProjectResponse.from_dict(result)

    def list(self, organization_id: str) -> ResponseForProjectsResponse:
        result = self.client._run_command(
            Command(projects=ProjectsCommand(
                list=ProjectsListRequest(organization_id)))
        )
        return ResponseForProjectsResponse.from_dict(result)

    def update(
            self,
            organization_id: str,
            id: str,
            name: str,
    ) -> ResponseForProjectResponse:
        result = self.client._run_command(
            Command(projects=ProjectsCommand(update=ProjectPutRequest(
                id, name, organization_id)))
        )
        return ResponseForProjectResponse.from_dict(result)

    def delete(self, ids: List[str]) -> ResponseForProjectsDeleteResponse:
        result = self.client._run_command(
            Command(projects=ProjectsCommand(delete=ProjectsDeleteRequest(ids)))
        )
        return ResponseForProjectsDeleteResponse.from_dict(result)
