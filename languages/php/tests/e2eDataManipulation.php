<?php

function env(string $key)
{
  $env = getenv($key);
  if ($env === false) {
    throw new Exception("Environment variable $key not set");
  }
  return $env;
}

function withRunId(string $s)
{
  $run_id = env('RUN_ID');
  return "{$s}-{$run_id}";
}

function filterProjectToThisRun($projects)
{
  $run_id = env('RUN_ID');
  return array_filter($projects, function ($project) use ($run_id) {
    # project name ends with run_id
    return substr($project->name, -strlen($run_id)) === $run_id;
  });
}

function filterSecretsToThisRun($secrets)
{
  $run_id = env('RUN_ID');
  return array_filter($secrets, function ($secret) use ($run_id) {
    # secret name ends with run_id
    return substr($secret->key, -strlen($run_id)) === $run_id;
  });
}

function projectWithRunId($project)
{
  $run_id = env('RUN_ID');
  $project->name = withRunId($project->name);
  return $project;
}

function secretWithRunId($secret)
{
  $run_id = env('RUN_ID');
  $secret->key = withRunId($secret->key);
  $secret->project_name = withRunId($secret->project_name);
  return $secret;
}

function secretWithProjectId($secret, $projects)
{
  $project = array_first($projects, function ($project) use ($secret) {
    return $project->name === $secret->project_name;
  });
  $secret->projectId = $project->id;
  $secret->organizationId = env("ORGANIZATION_ID");
  return $secret;
}

function array_first($array, $callback)
{
  foreach ($array as $key => $value) {
    if ($callback($value)) {
      return $value;
    }
  }
  return null;
}
