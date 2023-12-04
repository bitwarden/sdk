# frozen_string_literal: true

module BitwardenSDK
  class BitwardenError < StandardError
    def initialize(message = 'Error getting response')
      super(message)
    end
  end
end
