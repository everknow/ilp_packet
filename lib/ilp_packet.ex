defmodule IlpPacket do
  @moduledoc false

  use Rustler, otp_app: :ilp_packet, crate: "ilppacket"

  def decode(_a), do: :erlang.nif_error(:nif_not_loaded)
end
