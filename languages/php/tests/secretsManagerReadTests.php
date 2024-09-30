<?php

use PHPUnit\Framework\TestCase;

require_once 'bootstrap.php';
require_once 'e2eDataManipulation.php';


class SecretsManagerReadTests extends TestCase
{
  static $stateFile;
  static $client;
  static $expectedData;
  static $expectedProjects;
  static $expectedSecrets;
  static $projects;
  static $secrets;

  static function setUpBeforeClass(): void
  {
    self::$stateFile = __DIR__ . '/state.json';
    $bitwardenSettings = new \Bitwarden\Sdk\BitwardenSettings(env('API_URL'), env('IDENTITY_URL'));

    self::$client = new \Bitwarden\Sdk\BitwardenClient($bitwardenSettings);
    self::$client->auth->login_access_token(env('ACCESS_TOKEN'), self::$stateFile);

    # Load data
    self::$expectedData = json_decode(file_get_contents(env('TEST_DATA_FILE')));
    self::$expectedProjects = array_map(function ($p) {
      return projectWithRunId($p);
    }, self::$expectedData->projects);

    # Read projects and secrets from Bitwarden
    $allProjects = self::$client->projects->list(env('ORGANIZATION_ID'))->data;
    $allSecrets = self::$client->secrets->list(env('ORGANIZATION_ID'))->data;
    self::$projects = filterProjectToThisRun($allProjects);
    self::$secrets = filterSecretsToThisRun($allSecrets);

    # Populate expected secrets
    self::$expectedSecrets = array_map(function ($s) {
      return secretWithProjectId(secretWithRunId($s), self::$projects);
    }, self::$expectedData->secrets);
  }

  function testListProjects()
  {
    $this->assertEquals(count(self::$projects), count(self::$expectedProjects));
    $expectedNames = array_map(function ($project) {
      return $project->name;
    }, self::$expectedProjects);
    foreach (self::$projects as $project) {
      $this->assertTrue(in_array($project->name, $expectedNames));
    }
  }

  function testListSecrets()
  {
    $this->assertEquals(count(self::$secrets), count(self::$expectedSecrets));
    foreach (self::$secrets as $secret) {
      $filteredSecrets = array_filter(self::$expectedSecrets, function ($s) use ($secret) {
        return $s->key === $secret->key;
      });
      $this->assertEquals(count($filteredSecrets), 1);
      $this->assertEquals($secret->organizationId, env('ORGANIZATION_ID'));
    }
  }

  function testGetProject()
  {
    $project = array_first(self::$projects, function ($project) {
      return true;
    });
    $response = self::$client->projects->get($project->id);
    $this->assertProjectsEqual($project, $response);
  }

  function testGetSecret()
  {
    $secret = array_first(self::$secrets, function ($secret) {
      return true;
    });
    $response = self::$client->secrets->get($secret->id);
    $this->assertEquals($secret->key, $response->key);
    $this->assertEquals($secret->id, $response->id);
    $this->assertEquals($secret->organizationId, $response->organizationId);
  }

  function testNoUpdateSync()
  {
    $sync = self::$client->secrets->sync(env('ORGANIZATION_ID'), date('Y-m-d\TH:i:s.u\Z'));
    $this->assertEquals($sync->hasChanges, false);
  }

  private function assertSecretsEqual($expected, $actual)
  {
    $this->assertEquals($expected->key, $actual->key);
    $this->assertEquals($expected->value, $actual->value);
    $this->assertEquals($expected->note, $actual->note);
    $this->assertEquals($expected->organizationId, $actual->organizationId);
    $this->assertEquals($expected->projectId, $actual->projectId);
  }

  private function assertProjectsEqual($expected, $actual)
  {
    $this->assertEquals($expected->name, $actual->name);
  }

  static function tearDownAfterClass(): void
  {
    unlink(self::$stateFile);
  }
}
