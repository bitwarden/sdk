require 'bitwarden-sdk-secrets'
require 'rspec/expectations'
require_relative 'e2e_data_manipulation'
require 'pry'

organization_id = env('ORGANIZATION_ID')
language_tests_path = File.join(File.dirname(__FILE__), '..', 'language_tests')
expected_data_file_name = 'e2e_data.json'
expected_data_file = env("TEST_DATA_FILE")

RSpec::Matchers.define :equal_secret do |expected|
  match do |actual|
    # Note: actual secrets are from the sdk, expected are parsed from the test data.
    # actual SDK responses have the incorrect casing for organizationId and projectId.
    actual['key'] == expected['key'] && actual['value'] == expected['value'] && actual['note'] == expected['note'] && actual['organizationId'] == organization_id && actual['projectId'] == expected['project_id']
  end
end

RSpec::Matchers.define :equal_secret_identifier do |expected|
  match do |actual|
    actual['key'] == expected['key'] && actual['organizationId'] == organization_id
  end
end

RSpec::Matchers.define :equal_project do |expected|
  match do |actual|
    actual['name'] == expected['name']
  end
end

bitwarden_settings = BitwardenSDKSecrets::BitwardenSettings.new(env('API_URL'), env('IDENTITY_URL'))

describe 'Ruby Read E2E' do
  let(:expected_data) { JSON.parse(File.read(expected_data_file)) }
  let(:expected_projects) { expected_data['projects'].map { |p| project_with_run_id p } }
  let(:expected_secrets) { expected_data['secrets'].map { |s| secret_with_project_id(secret_with_run_id(s), projects) } }
  let(:projects) { @client.projects.list(organization_id) }
  let(:secrets) { @client.secrets.list(organization_id) }

  before(:all) do
    # Set up client
    @state_file = File.join(language_tests_path, 'state.json')
    @client = BitwardenSDKSecrets::BitwardenClient.new(bitwarden_settings)
    @client.auth.login_access_token(env('ACCESS_TOKEN'), @state_file)
  end

  it 'should successfully list projects' do
    expect(projects).to be_an_instance_of(Array)
  end

  it 'should successfully list secrets' do
    expect(secrets).to be_an_instance_of(Array)
  end

  it 'should list the correct projects' do
    expected_names = expected_projects.map { |p| p['name'] }
    projects.each do |project|
      expect(expected_names).to include(project['name'])
    end
  end

  it 'should list the correct secrets' do
    secrets.each do |secret|
      expected_secret = expected_secrets.find { |s| s['key'] == secret['key'] }
      expect(expected_secret).to_not be_nil
      expect(secret).to equal_secret_identifier(expected_secret)
    end
  end

  it 'should successfully get a project' do
    project = projects.first
    response = @client.projects.get(project['id'])
    expect(response).to equal_project(project)
  end

  it 'should successfully get a secret' do
    secret = secrets.first
    response = @client.secrets.get(secret['id'])
    expected_secret = expected_secrets.find { |s| s['key'] == secret['key'] }
    expect(response).to equal_secret(expected_secret)
  end

  after(:all) do
    File.delete(@state_file) if File.exist?(@state_file)
  end
end

describe 'Ruby Secrets Write E2E' do
  let(:expected_data) { JSON.parse(File.read(expected_data_file)) }
  let(:expected_projects) { expected_data['mutable_projects'].map { |p| project_with_run_id p } }
  let(:expected_secrets) { expected_data['mutable_secrets'].map { |s| secret_with_project_id(secret_with_run_id(s), projects) } }
  let(:projects) { @client.projects.list(organization_id) }
  let(:write_project_name) { with_run_id('for_write_tests') }
  let(:write_project) { projects.find { |p| p['name'] == write_project_name } }
  let(:secrets) { @client.secrets.list(organization_id) }

  before(:all) do
    # Set up client
    @state_file = File.join(language_tests_path, 'mutable_state.json')
    @client = BitwardenSDKSecrets::BitwardenClient.new(bitwarden_settings)
    @client.auth.login_access_token(env('MUTABLE_ACCESS_TOKEN'), @state_file)
  end

  it 'should successfully create a secret' do
    secret = {
      'key' => with_run_id('create_secret_key'),
      'value' => 'create_secret_value',
      'note' => 'create_secret_note',
      'project_name' => write_project_name,
      'project_id' => write_project['id']
    }
    created = @client.secrets.create(organization_id, secret['key'], secret['value'], secret['note'], [secret['project_id']])
    expect(created).to equal_secret(secret)
  end

  it 'should delete a secret' do
    to_delete = secrets.find { |s| s['key'] == with_run_id('to_delete') }
    expect(to_delete).to_not be_nil
    deleted = @client.secrets.delete([to_delete['id']])
    expect(deleted).to be_an_instance_of(Array)
    expect(deleted.length).to eq(1)
    expect(deleted[0]['id']).to eq(to_delete['id'])
  end

  it 'should update a secret' do
    to_update = secrets.find { |s| s['key'] == with_run_id('to_update') }
    expect(to_update).to_not be_nil
    updated_secret = {
      'key' => with_run_id('updated_key'),
      'value' => 'updated_value',
      'note' => 'updated_note',
      'project_id' => write_project['id']
    }
    updated = @client.secrets.update(organization_id, to_update['id'], updated_secret['key'], updated_secret['value'], updated_secret['note'], [write_project['id']])
    expect(updated).to equal_secret(updated_secret)
  end

  after(:all) do
    File.delete(@state_file) if File.exist?(@state_file)
  end

end

describe 'Ruby Projects Write E2E' do
  let(:expected_data) { JSON.parse(File.read(expected_data_file)) }
  let(:expected_projects) { expected_data['mutable_projects'].map { |p| project_with_run_id p } }
  let(:expected_secrets) { expected_data['mutable_secrets'].map { |s| secret_with_project_id(secret_with_run_id(s), projects) } }
  let(:projects) { @client.projects.list(organization_id) }
  let(:write_project_name) { with_run_id('for_write_tests') }
  let(:write_project) { projects.find { |p| p['name'] == write_project_name } }

  before(:all) do
    # Set up client
    @state_file = File.join(language_tests_path, 'mutable_state.json')
    @client = BitwardenSDKSecrets::BitwardenClient.new(bitwarden_settings)
    @client.auth.login_access_token(env('MUTABLE_ACCESS_TOKEN'), @state_file)
  end

  it 'should successfully create a project' do
    to_create = {
      'name' => with_run_id('created_project')
    }
    created = @client.projects.create(organization_id, to_create['name'])
    expect(created).to equal_project(to_create)
  end

  it 'should successfully update a project' do
    to_update = projects.find { |p| p['name'] == with_run_id('to_update') }
    expect(to_update).to_not be_nil
    updated_project = {
      'name' => with_run_id('updated_project')
    }
    updated = @client.projects.update(organization_id, to_update['id'], updated_project['name'])
    expect(updated).to equal_project(updated_project)
  end

  it 'should delete a project' do
    to_delete = projects.find { |p| p['name'] == with_run_id('to_delete') }
    expect(to_delete).to_not be_nil
    deleted = @client.projects.delete([to_delete['id']])
    expect(deleted).to be_an_instance_of(Array)
    expect(deleted.length).to eq(1)
    expect(deleted[0]['id']).to eq(to_delete['id'])
  end

  after(:all) do
    File.delete(@state_file) if File.exist?(@state_file)
  end
end
