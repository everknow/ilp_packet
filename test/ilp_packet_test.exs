defmodule IlpPacketTest do
  use ExUnit.Case

  @prepare """
  \x0c\x82\x01\x4b\x00\x00\x00\x00\x00\x00\x00\x6b\x32\x30\x31\x38\x30\x36\
  \x30\x37\x32\x30\x34\x38\x34\x32\x34\x38\x33\x11\x7b\x43\x4f\x1a\x54\xe9\
  \x04\x4f\x4f\x54\x92\x3b\x2c\xff\x9e\x4a\x6d\x42\x0a\xe2\x81\xd5\x02\x5d\
  \x7b\xb0\x40\xc4\xb4\xc0\x4a\x0d\x65\x78\x61\x6d\x70\x6c\x65\x2e\x61\x6c\
  \x69\x63\x65\x82\x01\x01\x6c\x99\xf6\xa9\x69\x47\x30\x28\xef\x46\xe0\x9b\
  \x47\x15\x81\xc9\x15\xb6\xd5\x49\x63\x29\xc1\xe3\xa1\xc2\x74\x8d\x74\x22\
  \xa7\xbd\xcc\x79\x8e\x28\x6c\xab\xe3\x19\x7c\xcc\xfc\x21\x3e\x93\x0b\x8d\
  \xba\x57\xc7\xab\xdf\x2d\x1f\x3b\x25\x11\x68\x9d\xe4\xf0\xef\xf4\x41\xf5\
  \x3d\xa0\xfe\xff\xd2\x32\x49\xa3\x55\xb2\x6c\x3b\xd0\x25\x6d\x51\x22\xe7\
  \xcc\xdf\x15\x9f\xd6\xcb\x08\x3d\xd7\x3c\xb2\x93\x97\x96\x78\x71\xbe\xcd\
  \x04\x89\x04\x92\x11\x9c\x5e\x3e\x6b\x02\x4b\xe3\x5d\xe2\x64\x66\xf6\x0c\
  \x16\xd9\x0a\x21\x05\x4f\xb1\x38\x00\x12\x0c\xfb\x85\xb0\xdf\x76\xe5\x0a\
  \xac\xd6\x85\x26\xfd\x04\x30\x26\xd3\xd0\x20\x10\xc6\x71\x98\x7a\x1f\x65\
  \x01\xb5\x08\x5f\x0d\x7d\x58\x97\x62\x4b\xe5\x86\x2f\x98\xc0\x1d\xf6\x57\
  \x92\x97\x01\x81\xa8\x7d\x0f\x3c\x58\x6a\x0c\xa6\xbd\x89\xdc\x37\x2c\x45\
  \xee\xf5\xb3\x8a\x63\x07\xb1\x6f\x1d\x7d\x31\xe8\xd9\x2e\x59\x82\xc9\xdd\
  \x29\x86\xea\xad\x58\x1f\x21\x2d\x43\xda\x9c\x5c\xb7\xb9\x48\xfc\x18\x91\
  \x4b\xe9\x02\x19\x70\x9d\x0c\x26\xd3\xb5\xf4\xad\x87\x9d\x84\x94\xbb\x3a\
  \xeb\xfe\x61\x2e\xc5\x40\x41\xe4\xa3\x80\xf0\
  """

  @fulfill """
  \x0d\x82\x01\x24\x11\x7b\x43\x4f\x1a\x54\xe9\x04\x4f\x4f\x54\x92\x3b\x2c\
  \xff\x9e\x4a\x6d\x42\x0a\xe2\x81\xd5\x02\x5d\x7b\xb0\x40\xc4\xb4\xc0\x4a\
  \x82\x01\x01\x6c\x99\xf6\xa9\x69\x47\x30\x28\xef\x46\xe0\x9b\x47\x15\x81\
  \xc9\x15\xb6\xd5\x49\x63\x29\xc1\xe3\xa1\xc2\x74\x8d\x74\x22\xa7\xbd\xcc\
  \x79\x8e\x28\x6c\xab\xe3\x19\x7c\xcc\xfc\x21\x3e\x93\x0b\x8d\xba\x57\xc7\
  \xab\xdf\x2d\x1f\x3b\x25\x11\x68\x9d\xe4\xf0\xef\xf4\x41\xf5\x3d\xa0\xfe\
  \xff\xd2\x32\x49\xa3\x55\xb2\x6c\x3b\xd0\x25\x6d\x51\x22\xe7\xcc\xdf\x15\
  \x9f\xd6\xcb\x08\x3d\xd7\x3c\xb2\x93\x97\x96\x78\x71\xbe\xcd\x04\x89\x04\
  \x92\x11\x9c\x5e\x3e\x6b\x02\x4b\xe3\x5d\xe2\x64\x66\xf6\x0c\x16\xd9\x0a\
  \x21\x05\x4f\xb1\x38\x00\x12\x0c\xfb\x85\xb0\xdf\x76\xe5\x0a\xac\xd6\x85\
  \x26\xfd\x04\x30\x26\xd3\xd0\x20\x10\xc6\x71\x98\x7a\x1f\x65\x01\xb5\x08\
  \x5f\x0d\x7d\x58\x97\x62\x4b\xe5\x86\x2f\x98\xc0\x1d\xf6\x57\x92\x97\x01\
  \x81\xa8\x7d\x0f\x3c\x58\x6a\x0c\xa6\xbd\x89\xdc\x37\x2c\x45\xee\xf5\xb3\
  \x8a\x63\x07\xb1\x6f\x1d\x7d\x31\xe8\xd9\x2e\x59\x82\xc9\xdd\x29\x86\xea\
  \xad\x58\x1f\x21\x2d\x43\xda\x9c\x5c\xb7\xb9\x48\xfc\x18\x91\x4b\xe9\x02\
  \x19\x70\x9d\x0c\x26\xd3\xb5\xf4\xad\x87\x9d\x84\x94\xbb\x3a\xeb\xfe\x61\
  \x2e\xc5\x40\x41\xe4\xa3\x80\xf0\
  """

  @reject """
  \x0e\x82\x01\x24\x46\x39\x39\x11\x65\x78\x61\x6d\x70\x6c\x65\x2e\x63\x6f\
  \x6e\x6e\x65\x63\x74\x6f\x72\x0a\x53\x6f\x6d\x65\x20\x65\x72\x72\x6f\x72\
  \x82\x01\x01\x6c\x99\xf6\xa9\x69\x47\x30\x28\xef\x46\xe0\x9b\x47\x15\x81\
  \xc9\x15\xb6\xd5\x49\x63\x29\xc1\xe3\xa1\xc2\x74\x8d\x74\x22\xa7\xbd\xcc\
  \x79\x8e\x28\x6c\xab\xe3\x19\x7c\xcc\xfc\x21\x3e\x93\x0b\x8d\xba\x57\xc7\
  \xab\xdf\x2d\x1f\x3b\x25\x11\x68\x9d\xe4\xf0\xef\xf4\x41\xf5\x3d\xa0\xfe\
  \xff\xd2\x32\x49\xa3\x55\xb2\x6c\x3b\xd0\x25\x6d\x51\x22\xe7\xcc\xdf\x15\
  \x9f\xd6\xcb\x08\x3d\xd7\x3c\xb2\x93\x97\x96\x78\x71\xbe\xcd\x04\x89\x04\
  \x92\x11\x9c\x5e\x3e\x6b\x02\x4b\xe3\x5d\xe2\x64\x66\xf6\x0c\x16\xd9\x0a\
  \x21\x05\x4f\xb1\x38\x00\x12\x0c\xfb\x85\xb0\xdf\x76\xe5\x0a\xac\xd6\x85\
  \x26\xfd\x04\x30\x26\xd3\xd0\x20\x10\xc6\x71\x98\x7a\x1f\x65\x01\xb5\x08\
  \x5f\x0d\x7d\x58\x97\x62\x4b\xe5\x86\x2f\x98\xc0\x1d\xf6\x57\x92\x97\x01\
  \x81\xa8\x7d\x0f\x3c\x58\x6a\x0c\xa6\xbd\x89\xdc\x37\x2c\x45\xee\xf5\xb3\
  \x8a\x63\x07\xb1\x6f\x1d\x7d\x31\xe8\xd9\x2e\x59\x82\xc9\xdd\x29\x86\xea\
  \xad\x58\x1f\x21\x2d\x43\xda\x9c\x5c\xb7\xb9\x48\xfc\x18\x91\x4b\xe9\x02\
  \x19\x70\x9d\x0c\x26\xd3\xb5\xf4\xad\x87\x9d\x84\x94\xbb\x3a\xeb\xfe\x61\
  \x2e\xc5\x40\x41\xe4\xa3\x80\xf0\
  """

  @data """
  \x6c\x99\xf6\xa9\x69\x47\x30\x28\xef\x46\xe0\x9b\x47\x15\x81\xc9\x15\xb6\
  \xd5\x49\x63\x29\xc1\xe3\xa1\xc2\x74\x8d\x74\x22\xa7\xbd\xcc\x79\x8e\x28\
  \x6c\xab\xe3\x19\x7c\xcc\xfc\x21\x3e\x93\x0b\x8d\xba\x57\xc7\xab\xdf\x2d\
  \x1f\x3b\x25\x11\x68\x9d\xe4\xf0\xef\xf4\x41\xf5\x3d\xa0\xfe\xff\xd2\x32\
  \x49\xa3\x55\xb2\x6c\x3b\xd0\x25\x6d\x51\x22\xe7\xcc\xdf\x15\x9f\xd6\xcb\
  \x08\x3d\xd7\x3c\xb2\x93\x97\x96\x78\x71\xbe\xcd\x04\x89\x04\x92\x11\x9c\
  \x5e\x3e\x6b\x02\x4b\xe3\x5d\xe2\x64\x66\xf6\x0c\x16\xd9\x0a\x21\x05\x4f\
  \xb1\x38\x00\x12\x0c\xfb\x85\xb0\xdf\x76\xe5\x0a\xac\xd6\x85\x26\xfd\x04\
  \x30\x26\xd3\xd0\x20\x10\xc6\x71\x98\x7a\x1f\x65\x01\xb5\x08\x5f\x0d\x7d\
  \x58\x97\x62\x4b\xe5\x86\x2f\x98\xc0\x1d\xf6\x57\x92\x97\x01\x81\xa8\x7d\
  \x0f\x3c\x58\x6a\x0c\xa6\xbd\x89\xdc\x37\x2c\x45\xee\xf5\xb3\x8a\x63\x07\
  \xb1\x6f\x1d\x7d\x31\xe8\xd9\x2e\x59\x82\xc9\xdd\x29\x86\xea\xad\x58\x1f\
  \x21\x2d\x43\xda\x9c\x5c\xb7\xb9\x48\xfc\x18\x91\x4b\xe9\x02\x19\x70\x9d\
  \x0c\x26\xd3\xb5\xf4\xad\x87\x9d\x84\x94\xbb\x3a\xeb\xfe\x61\x2e\xc5\x40\
  \x41\xe4\xa3\x80\xf0\
  """

  test "decode/1" do
    prepare = @prepare
    fulfill = @fulfill
    reject = @reject
    data = :binary.bin_to_list(@data)

    assert {:ok, %{"type" => :prepare, "data" => ^data}} = IlpPacket.decode(prepare)
    assert {:ok, %{"type" => :fulfill, "data" => ^data}} = IlpPacket.decode(fulfill)
    assert {:ok, %{"type" => :reject, "data" => ^data}} = IlpPacket.decode(reject)
    assert {:error, "Invalid Packet Unknown packet type: None"} = IlpPacket.decode("")
  end
end
