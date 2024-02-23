defmodule ExrustTest do
  use ExUnit.Case
  doctest Exrust

  test "greets the world" do
    assert Exrust.hello() == :world
  end
end
