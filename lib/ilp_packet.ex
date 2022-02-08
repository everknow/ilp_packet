defmodule IlpPacket do
  @moduledoc false

  use Rustler, otp_app: :ilp_packet, crate: "packet"

  def decode(_a), do: :erlang.nif_error(:nif_not_loaded)

  def encode_prepare(_a), do: :erlang.nif_error(:nif_not_loaded)

  def encode_fulfill(_a), do: :erlang.nif_error(:nif_not_loaded)

  def encode_reject(_a), do: :erlang.nif_error(:nif_not_loaded)
end
