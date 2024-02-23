defmodule Exrust do
  @moduledoc """
  Documentation for `Exrust`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> Exrust.hello()
      :world

  """

  def main(_args) do
    num = Exrust.Native.add(1, 2)
    IO.puts(num)
  end
end
