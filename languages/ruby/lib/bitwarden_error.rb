# frozen_string_literal: true

class BitwardenError < StandardError
  def initialize(message = "SDK Error Occurred")
    super(message)
  end
end

