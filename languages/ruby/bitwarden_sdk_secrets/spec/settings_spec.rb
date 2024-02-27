require 'schemas'
require 'extended_schemas/schemas'

describe ClientSettings do
  it "test" do
    client_settings = ClientSettings.new(
      api_url: nil,
      identity_url: nil,
      user_agent: 'Bitwarden RUBY-SDK',
      device_type: nil
    )

    expect(client_settings.to_dynamic.compact.to_json).to eq('{"userAgent":"Bitwarden RUBY-SDK"}')
  end
end
