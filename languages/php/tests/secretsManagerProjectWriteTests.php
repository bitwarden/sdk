<?php

use PHPUnit\Framework\TestCase;

require_once 'bootstrap.php';
require_once 'e2eDataManipulation.php';

class SecretsManagerProjectWriteTests extends TestCase
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
    self::$stateFile = __DIR__ . '/mutable_state.json';
    $bitwardenSettings = new \Bitwarden\Sdk\BitwardenSettings(env('API_URL'), env('IDENTITY_URL'));

    self::$client = new \Bitwarden\Sdk\BitwardenClient($bitwardenSettings);
    self::$client->auth->login_access_token(env('MUTABLE_ACCESS_TOKEN'), self::$stateFile);

    # Read projects and secrets from Bitwarden
    $allProjects = self::$client->projects->list(env('ORGANIZATION_ID'))->data;
    self::$projects = filterProjectToThisRun($allProjects);
  }

  function testCreateProject()
  {
    $toCreate = new stdClass();
    $toCreate->name = withRunId('created');

    $res = self::$client->projects->create(env('ORGANIZATION_ID'), $toCreate->name);
    $this->assertProjectsEqual($toCreate, $res);
  }

  function testUpdateProject()
  {
    $updated = new stdClass();
    $updated->name = withRunId('updated');

    $toUpdate = array_first(self::$projects, function ($project) {
      return $project->name === withRunId('to_update');
    });

    $res = self::$client->projects->update(env('ORGANIZATION_ID'), $toUpdate->id, $updated->name);
    $this->assertProjectsEqual($res, $updated);
  }

  function testDeleteProject()
  {
    $toDelete = array_first(self::$projects, function ($project) {
      return $project->name === withRunId('to_delete');
    });

    $res = self::$client->projects->delete([$toDelete->id]);
    $this->assertEquals(array_map(function ($s) {
      return $s->id;
    }, $res->data), [$toDelete->id]);
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
