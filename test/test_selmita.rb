# frozen_string_literal: true

require "test_helper"

class TestSelmita < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::Selmita::VERSION
  end

  def test_it_does_something_useful
    input = Selmita::HTML.new("html")
    assert_equal("wow!", input.rewrite)
  end
end
