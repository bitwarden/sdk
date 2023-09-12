import json
from typing import Any, List, Optional
from uuid import UUID
import bitwarden_py
from .schemas import ClientSettings, Command, PasswordLoginRequest, PasswordLoginResponse, ResponseForPasswordLoginResponse, ResponseForSecretIdentifiersResponse, ResponseForSecretResponse, ResponseForSecretsDeleteResponse, ResponseForSyncResponse, ResponseForUserAPIKeyResponse, SecretCreateRequest, SecretGetRequest, SecretIdentifiersRequest, SecretIdentifiersResponse, SecretPutRequest, SecretResponse, SecretVerificationRequest, SecretsCommand, SecretsDeleteRequest, SecretsDeleteResponse, SyncRequest, SyncResponse, UserAPIKeyResponse, AccessTokenLoginRequest, AccessTokenLoginResponse, ResponseForAccessTokenLoginResponse

class BitwardenClient:
    def __init__(self, settings: ClientSettings = None):
        if settings is None:
            self.inner = bitwarden_py.BitwardenClient(None)
        else:
            settings_json = json.dumps(settings.to_dict())
            self.inner = bitwarden_py.BitwardenClient(settings_json)

    def password_login(self, email: str, password: str) -> ResponseForPasswordLoginResponse:
        result = self._run_command(
            Command(password_login=PasswordLoginRequest(email, password))
        )
        return ResponseForPasswordLoginResponse.from_dict(result)

    def access_token_login(self, access_token: str) -> AccessTokenLoginResponse:
        result = self._run_command(
            Command(access_token_login=AccessTokenLoginRequest(access_token))
        )
        return ResponseForAccessTokenLoginResponse.from_dict(result)

    def get_user_api_key(self, secret: str, is_otp: bool = False) -> ResponseForUserAPIKeyResponse:
        result = self._run_command(
            Command(get_user_api_key=SecretVerificationRequest(
                secret if not is_otp else None, secret if is_otp else None))
        )
        return ResponseForUserAPIKeyResponse.from_dict(result)

    def sync(self, exclude_subdomains: bool = False) -> ResponseForSyncResponse:
        result = self._run_command(
            Command(sync=SyncRequest(exclude_subdomains))
        )
        return ResponseForSyncResponse.from_dict(result)

    def secrets(self):
        return SecretsClient(self)

    def _run_command(self, command: Command) -> Any:
        response_json = self.inner.run_command(json.dumps(command.to_dict()))
        return json.loads(response_json)

class SecretsClient:
    def __init__(self, client: BitwardenClient):
        self.client = client

    def get(self, id: str) -> ResponseForSecretResponse:
        result = self.client._run_command(
            Command(secrets=SecretsCommand(get=SecretGetRequest(id)))
        )
        return ResponseForSecretResponse.from_dict(result)

    def create(self, key: str,
               note: str,
               organization_id: str,
               value: str,
               project_ids: Optional[List[UUID]] = None
               ) -> ResponseForSecretResponse:
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

    def update(self, id: str,
               key: str,
               note: str,
               organization_id: str,
               value: str) -> ResponseForSecretResponse:
        result = self.client._run_command(
            Command(secrets=SecretsCommand(update=SecretPutRequest(
                id, key, note, organization_id, value)))
        )
        return ResponseForSecretResponse.from_dict(result)

    def delete(self, ids: List[str]) -> ResponseForSecretsDeleteResponse:
        result = self.client._run_command(
            Command(secrets=SecretsCommand(delete=SecretsDeleteRequest(ids)))
        )
        return ResponseForSecretsDeleteResponse.from_dict(result)
