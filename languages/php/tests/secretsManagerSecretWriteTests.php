<?php

use PHPUnit\Framework\TestCase;

require_once 'bootstrap.php';
require_once 'e2eDataManipulation.php';

class SecretsManagerSecretWriteTests extends TestCase
{
  static $stateFile;
  static $client;
  static $expectedData;
  static $expectedProjects;
  static $expectedSecrets;
  static $projects;
  static $secrets;
  static $writeProject;

  static function setUpBeforeClass(): void
  {
    self::$stateFile = __DIR__ . '/mutable_state.json';
    $bitwardenSettings = new \Bitwarden\Sdk\BitwardenSettings(env('API_URL'), env('IDENTITY_URL'));

    self::$client = new \Bitwarden\Sdk\BitwardenClient($bitwardenSettings);
    self::$client->auth->login_access_token(env('MUTABLE_ACCESS_TOKEN'), self::$stateFile);

    # Read projects and secrets from Bitwarden
    $allProjects = self::$client->projects->list(env('ORGANIZATION_ID'))->data;
    $allSecrets = self::$client->secrets->list(env('ORGANIZATION_ID'))->data;
    self::$projects = filterProjectToThisRun($allProjects);
    self::$secrets = filterSecretsToThisRun($allSecrets);
    self::$writeProject = array_first(self::$projects, function ($project) {
      return $project->name === withRunId('for_write_tests');
    });
  }

  function testCreateSecret()
  {
    $toCreate = new stdClass();
    $toCreate->key = withRunId('created');
    $toCreate->value = 'create_secret_value';
    $toCreate->note = 'create_secret_note';
    $toCreate->project_name = self::$writeProject->name;
    $toCreate->project_id = self::$writeProject->id;

    $res = self::$client->secrets->create(env('ORGANIZATION_ID'), $toCreate->key, $toCreate->value, $toCreate->note, [$toCreate->project_id]);
    var_dump($res);
    var_dump($toCreate);
    $this->assertEquals($res->key, $toCreate->key);
    $this->assertEquals($res->value, $toCreate->value);
    $this->assertEquals($res->note, $toCreate->note);
    $this->assertEquals($res->organizationId, env('ORGANIZATION_ID'));
    $this->assertEquals($res->projectId, $toCreate->project_id);
  }

  function testUpdateSecret()
  {
    $updated = new stdClass();
    $updated->key = withRunId('updated');
    $updated->value = 'updated_value';
    $updated->note = 'updated_note';
    $updated->project_id = self::$writeProject->id;

    $toUpdate = array_first(self::$secrets, function ($secret) {
      return $secret->key === withRunId('to_update');
    });
    $this->assertNotNull($toUpdate);

    $res = self::$client->secrets->update(env('ORGANIZATION_ID'), $toUpdate->id, $updated->key, $updated->value, $updated->note, [$updated->project_id]);
    $this->assertEquals($res->key, $updated->key);
    $this->assertEquals($res->value, $updated->value);
    $this->assertEquals($res->note, $updated->note);
    $this->assertEquals($res->organizationId, env('ORGANIZATION_ID'));
    $this->assertEquals($res->projectId, $updated->project_id);
  }

  function testDeleteSecret()
  {
    $toDelete = array_first(self::$secrets, function ($secret) {
      return $secret->key === withRunId('to_delete');
    });

    $res = self::$client->secrets->delete([$toDelete->id]);
    $this->assertEquals(array_map(function ($s) {
      return $s->id;
    }, $res->data), [$toDelete->id]);
  }

  private function assertSecretsEqual($expected, $actual)
  {
    $this->assertEquals($expected->key, $actual->key);
    $this->assertEquals($expected->value, $actual->value);
    $this->assertEquals($expected->note, $actual->note);
    $this->assertEquals($expected->organizationId, $actual->organizationId);
    $this->assertEquals($expected->projectId, $actual->projectId);
  }

  static function tearDownAfterClass(): void
  {
    unlink(self::$stateFile);
  }
}
