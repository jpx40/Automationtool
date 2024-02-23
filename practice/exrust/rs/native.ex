defmodule Exrust.Native do
  use Rustler, otp_app: :exrust, crate: :nif_exrust


  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
  
end
