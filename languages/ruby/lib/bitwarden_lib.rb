# frozen_string_literal: true
require "ffi"

module BitwardenLib
  extend FFI::Library

  ffi_lib case RUBY_PLATFORM
          when /darwin/
            File.join(__dir__, "../../../target/debug/libbitwarden_c.dylib")
          when /linux/
            File.join(__dir__, "../../../target/debug/libbitwarden_c.so")
          when /mswin|mingw/
            File.join(__dir__, "../../../target/bitwarden_c.dll")
          else
            raise "Unsupported platform: #{RUBY_PLATFORM}"
          end

  attach_function :init, [:string], :pointer
  attach_function :run_command, %i[string pointer], :string
  attach_function :free_mem, [:pointer], :void
end
