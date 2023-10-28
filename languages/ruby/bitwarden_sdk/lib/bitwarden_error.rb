# frozen_string_literal: true

module BitwardenSDK
  class BitwardenError < StandardError
    def initialize(message = "SDK Error Occurred")
      super(message)
    end
  end
end
