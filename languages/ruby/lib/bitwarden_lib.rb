# frozen_string_literal: true

require 'ffi'

module BitwardenLib
  extend FFI::Library

  ffi_lib case RUBY_PLATFORM
          when /darwin/
            local_file = File.expand_path('libbitwarden_c.dylib', __dir__)
            File.exist?(local_file) ? local_file : File.expand_path('../../../target/debug/libbitwarden_c.dylib', __dir__)
          when /linux/
            local_file = File.expand_path('libbitwarden_c.so', __dir__)
            File.exist?(local_file) ? local_file : File.expand_path('../../../target/debug/libbitwarden_c.so', __dir__)
          when /mswin|mingw/
            local_file = File.expand_path('libbitwarden_c.dll', __dir__)
            File.exist?(local_file) ? local_file : File.expand_path('../../../target/debug/bitwarden_c.dll', __dir__)
          else
            raise "Unsupported platform: #{RUBY_PLATFORM}"
          end

  attach_function :init, [:string], :pointer
  attach_function :run_command, %i[string pointer], :string
  attach_function :free_mem, [:pointer], :void
end
