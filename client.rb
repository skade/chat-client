require 'ffi'

module ChatClient
  extend FFI::Library
  ffi_lib 'target/debug/libsync_chat_client.so'

  attach_function :start_client, [ :string, :string ], :pointer
  attach_function :read_line,    [ :pointer ], :string
  attach_function :free_line,    [ :string ], :void
  attach_function :write_msg,    [ :pointer, :string ], :void
  attach_function :free_client,  [ :pointer ], :void
end

client = ChatClient.start_client("127.0.0.1:8080", "skade")

ChatClient.write_msg(client, "test")
line = ChatClient.read_line(client)
puts line
ChatClient.free_client(client)
