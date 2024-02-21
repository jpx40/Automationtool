defmodule CentralTest do
  use ExUnit.Case
  doctest Central

  test "greets the world" do
    assert Central.hello() == :world
  end
end
