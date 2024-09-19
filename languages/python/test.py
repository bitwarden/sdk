#!/usr/bin/env python3
import unittest
import sys
import os
import json
import uuid

from bitwarden_sdk import (
    BitwardenClient,
    DeviceType,
    SecretResponse,
    ProjectResponse,
    client_settings_from_dict,
)

language_tests_path = os.path.join(os.path.dirname(__file__), "..", "language-tests")
run_id = os.getenv("RUN_ID")
if run_id is None:
    raise ValueError("RUN_ID is required")

organization_id = uuid.UUID(os.getenv("ORGANIZATION_ID"))
if organization_id is None:
    raise ValueError("ORGANIZATION_ID is required")


def with_run_id(val: str) -> str:
    return f"{val}-{run_id}"


def project_with_run_id(val: [dict]) -> [dict]:
    return [dict(val, name=with_run_id(val["name"])) for val in val]


def secret_with_run_id(val: [dict]) -> [dict]:
    return [
        dict(
            val,
            key=with_run_id(val["key"]),
            project_name=with_run_id(val["project_name"]),
        )
        for val in val
    ]


def secret_with_project_id(val: [dict], projects: [ProjectResponse]) -> [dict]:
    return [
        dict(
            val,
            project_id=next(
                (
                    project.id
                    for project in projects
                    if project.name == val["project_name"]
                ),
                None,
            ),
        )
        for val in val
    ]


def secrets_equal(a: dict, b: SecretResponse, projects: [ProjectResponse]) -> bool:
    return (
        a["key"] == b.key
        and a["project_id"] == b.project_id
        and b.organization_id == organization_id
        and a["value"] == b.value
        and a["note"] == b.note
    )


def projects_equal(a: dict, b: ProjectResponse) -> bool:
    return a["name"] == b.name


class PythonLanguageTests(object):
    @classmethod
    def setUpClass(cls):
        # create clients
        settings = client_settings_from_dict(
            {
                "apiUrl": os.getenv("API_URL", "https://vault.qa.bitwarden.pw/api"),
                "deviceType": DeviceType.SDK,
                "identityUrl": os.getenv(
                    "IDENTITY_URL", "https://vault.qa.bitwarden.pw/api"
                ),
                "userAgent": "Python",
            }
        )
        cls.client = BitwardenClient(settings)
        cls.mutable_client = BitwardenClient(settings)

        # authenticate
        print(cls)
        cls.state_path = os.path.join(language_tests_path, "state.json")
        cls.client.auth().login_access_token(os.getenv("ACCESS_TOKEN"), cls.state_path)
        cls.mutable_client.auth().login_access_token(
            os.getenv("MUTABLE_ACCESS_TOKEN"), cls.state_path
        )

        # Query for projects
        cls.projects_response = cls.client.projects().list(organization_id)
        cls.projects = getattr(cls.projects_response.data, "data", None)
        cls.mutable_projects_response = cls.mutable_client.projects().list(
            organization_id
        )
        cls.mutable_projects = getattr(cls.mutable_projects_response.data, "data", None)

    @classmethod
    def tearDownClass(cls):
        os.remove(cls.state_path)


class ReadTests(PythonLanguageTests, unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        super().setUpClass()
        # Read expected projects and secrets
        with open(os.path.join(language_tests_path, "e2e_data.json")) as f:
            data = json.load(f)
            cls.expected_projects = project_with_run_id(data["projects"])
            cls.expected_secrets = secret_with_project_id(
                secret_with_run_id(data["secrets"]), cls.projects
            )

        # Query for secrets
        cls.list_response = cls.client.secrets().list(organization_id)
        cls.secrets_response = cls.client.secrets().get_by_ids(
            [secret.id for secret in cls.list_response.data.data]
        )
        cls.secrets = getattr(cls.secrets_response.data, "data", None)
        cls.list = getattr(cls.list_response.data, "data", None)

    def test_list_response_is_success(self):
        self.assertTrue(self.list_response.success)

    def test_secrets_response_is_success(self):
        self.assertTrue(self.secrets_response.success)

    def test_list_data_is_not_none(self):
        self.assertIsNotNone(self.list)

    def test_secrets_data_is_not_none(self):
        self.assertIsNotNone(self.secrets)

    def test_secrets_have_correct_data(self):
        for secret in self.secrets:
            expected_secret = next(
                (s for s in self.expected_secrets if s["key"] == secret.key),
                None,
            )
            self.assertIsNotNone(expected_secret)
            self.assertTrue(secrets_equal(expected_secret, secret, self.projects))

    def test_projects_have_correct_data(self):
        expected_names = [p["name"] for p in self.expected_projects]
        for project in self.projects:
            self.assertIn(project.name, expected_names)

    def test_secret_get_equal_to_get_by_id(self):
        for secret in self.secrets:
            self.assertEqual(
                secret,
                self.client.secrets().get(secret.id).data,
            )

    def test_sync(self):
        response = self.client.secrets().sync(organization_id, None)
        self.assertTrue(response.success)
        for secret in response.data.secrets:
            expected_secret = next(
                (s for s in self.expected_secrets if s["key"] == secret.key),
                None,
            )
            self.assertIsNotNone(expected_secret)
            self.assertTrue(secrets_equal(expected_secret, secret, self.projects))

    def test_project_get_equal_to_list(self):
        for project in self.projects:
            self.assertEqual(
                project,
                self.client.projects().get(project.id).data,
            )


class SecretWriteTests(PythonLanguageTests, unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        super().setUpClass()
        cls.write_project = next(
            (
                p
                for p in cls.mutable_projects
                if p.name == with_run_id("for_write_tests")
            ),
            None,
        )

    def find_secret(self, key: str) -> SecretResponse:
        return next(
            (
                s
                for s in self.mutable_client.secrets().list(organization_id).data.data
                if s.key == key
            ),
            None,
        )

    def test_create_secret(self):
        secret = {
            "key": with_run_id("create_secret_key"),
            "value": "create_secret_value",
            "note": "create_secret_note",
            "project_name": with_run_id("for_write_tests"),
            "project_id": self.write_project.id,
        }
        response = self.mutable_client.secrets().create(
            organization_id,
            secret["key"],
            secret["value"],
            secret["note"],
            [self.write_project.id],
        )
        self.assertTrue(response.success)
        self.assertTrue(secrets_equal(secret, response.data, self.projects))

        # delete
        response = self.mutable_client.secrets().delete([response.data.id])

    def test_delete_secret(self):
        to_delete_secret = self.find_secret(with_run_id("to_delete"))
        self.assertIsNotNone(to_delete_secret)
        response = self.mutable_client.secrets().delete([to_delete_secret.id])

    def test_update_secret(self):
        to_update_secret = self.find_secret(with_run_id("to_update"))
        self.assertIsNotNone(to_update_secret)
        updated_secret = {
            "key": with_run_id("updated_key"),
            "value": "updated_value",
            "note": "updated_note",
            "project_name": with_run_id("for_write_tests"),
            "project_id": self.write_project.id,
        }

        response = self.mutable_client.secrets().update(
            organization_id,
            to_update_secret.id,
            updated_secret["key"],
            updated_secret["value"],
            updated_secret["note"],
            [self.write_project.id],
        )
        self.assertTrue(response.success)
        self.assertTrue(secrets_equal(updated_secret, response.data, self.projects))


class ProjectsWriteTests(PythonLanguageTests, unittest.TestCase):
    def find_project(self, name: str) -> ProjectResponse:
        return next(
            (
                p
                for p in self.mutable_client.projects().list(organization_id).data.data
                if p.name == name
            ),
            None,
        )

    def test_create_project(self):
        project = {
            "name": with_run_id("created_project"),
            "organization_id": organization_id,
        }
        response = self.mutable_client.projects().create(
            organization_id, project["name"]
        )
        self.assertTrue(response.success)
        self.assertTrue(projects_equal(project, response.data))

        # delete
        response = self.mutable_client.projects().delete([response.data.id])

    def test_delete_project(self):
        to_delete_project = self.find_project(with_run_id("to_delete"))
        self.assertIsNotNone(to_delete_project)
        response = self.mutable_client.projects().delete([to_delete_project.id])
        self.assertTrue(response.success)

    def test_update_project(self):
        to_update_project = self.find_project(with_run_id("to_update"))
        self.assertIsNotNone(to_update_project)
        updated_project = {
            "name": with_run_id("updated_project"),
            "organization_id": organization_id,
        }
        response = self.mutable_client.projects().update(
            organization_id, to_update_project.id, updated_project["name"]
        )
        self.assertTrue(response.success)
        self.assertTrue(projects_equal(updated_project, response.data))


if __name__ == "__main__":
    unittest.main()
