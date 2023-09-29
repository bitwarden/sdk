# frozen_string_literal: true

require_relative 'lib/ruby/sdk/version'

Gem::Specification.new do |spec|
  spec.name = 'ruby-bitwarden-sdk'
  spec.version = Ruby::Bitwarden::Sdk::VERSION
  spec.authors = ['Milos Trifunovic']
  spec.email = ['milos.trifunovic@symphony.is']

  spec.summary = 'Bitwarden Secrets Manager SDK.'
  spec.description = 'Ruby wrapper for Bitwarden secrets manager SDK.'
  spec.homepage = 'https://rubygems.org/gems/ruby-bitwarden-sdk'
  spec.required_ruby_version = '>= 2.7.0'

  spec.metadata['homepage_uri'] = spec.homepage
  spec.metadata['source_code_uri'] = 'https://github.com/MaliRobot/ruby-sdk'
  spec.metadata['changelog_uri'] = 'https://github.com/MaliRobot/ruby-sdk'

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir.chdir(__dir__) do
    `git ls-files -z`.split("\x0").reject do |f|
      (File.expand_path(f) == __FILE__) || f.start_with?(*%w[bin/ test/ spec/ features/ .git .circleci appveyor])
    end
  end
  spec.bindir = 'exe'
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ['lib']

  # Uncomment to register a new dependency of your gem
  # spec.add_dependency "example-gem", "~> 1.0"
  spec.add_dependency 'dry-struct', '~> 1.6'
  spec.add_dependency 'dry-types', '~> 1.7'
  spec.add_dependency 'ffi', '~> 1.15'
  spec.add_dependency 'json', '~> 2.6'
  spec.add_dependency 'rake', '~> 13.0'
  spec.add_dependency 'rubocop', '~> 1.21'

end
