-module(glisten@transport).
-compile([no_auto_import, nowarn_unused_vars, nowarn_unused_function, nowarn_nomatch]).

-export([controlling_process/3, listen/3, accept_timeout/3, accept/2, handshake/2, receive_timeout/4, 'receive'/3, send/3, close/2, shutdown/2, set_opts/3, negotiated_protocol/2, peername/2, socket_info/1]).
-export_type([transport/0]).

-type transport() :: tcp | ssl.

-spec controlling_process(
    transport(),
    glisten@socket:socket(),
    gleam@erlang@process:pid_()
) -> {ok, nil} | {error, gleam@erlang@atom:atom_()}.
controlling_process(Transport, Socket, Pid) ->
    case Transport of
        tcp ->
            glisten_tcp_ffi:controlling_process(Socket, Pid);

        ssl ->
            glisten_ssl_ffi:controlling_process(Socket, Pid)
    end.

-spec listen(transport(), integer(), list(glisten@socket@options:tcp_option())) -> {ok,
        glisten@socket:listen_socket()} |
    {error, glisten@socket:socket_reason()}.
listen(Transport, Port, Opts) ->
    case Transport of
        tcp ->
            glisten@tcp:listen(Port, Opts);

        ssl ->
            glisten@ssl:listen(Port, Opts)
    end.

-spec accept_timeout(transport(), glisten@socket:listen_socket(), integer()) -> {ok,
        glisten@socket:socket()} |
    {error, glisten@socket:socket_reason()}.
accept_timeout(Transport, Socket, Timeout) ->
    case Transport of
        tcp ->
            gen_tcp:accept(Socket, Timeout);

        ssl ->
            ssl:transport_accept(Socket, Timeout)
    end.

-spec accept(transport(), glisten@socket:listen_socket()) -> {ok,
        glisten@socket:socket()} |
    {error, glisten@socket:socket_reason()}.
accept(Transport, Socket) ->
    case Transport of
        tcp ->
            gen_tcp:accept(Socket);

        ssl ->
            ssl:transport_accept(Socket)
    end.

-spec handshake(transport(), glisten@socket:socket()) -> {ok,
        glisten@socket:socket()} |
    {error, nil}.
handshake(Transport, Socket) ->
    case Transport of
        tcp ->
            glisten@tcp:handshake(Socket);

        ssl ->
            ssl:handshake(Socket)
    end.

-spec receive_timeout(
    transport(),
    glisten@socket:socket(),
    integer(),
    integer()
) -> {ok, bitstring()} | {error, glisten@socket:socket_reason()}.
receive_timeout(Transport, Socket, Amount, Timeout) ->
    case Transport of
        tcp ->
            gen_tcp:recv(Socket, Amount, Timeout);

        ssl ->
            ssl:recv(Socket, Amount, Timeout)
    end.

-spec 'receive'(transport(), glisten@socket:socket(), integer()) -> {ok,
        bitstring()} |
    {error, glisten@socket:socket_reason()}.
'receive'(Transport, Socket, Amount) ->
    case Transport of
        tcp ->
            gen_tcp:recv(Socket, Amount);

        ssl ->
            ssl:recv(Socket, Amount)
    end.

-spec send(
    transport(),
    glisten@socket:socket(),
    gleam@bytes_builder:bytes_builder()
) -> {ok, nil} | {error, glisten@socket:socket_reason()}.
send(Transport, Socket, Data) ->
    case Transport of
        tcp ->
            glisten_tcp_ffi:send(Socket, Data);

        ssl ->
            glisten_ssl_ffi:send(Socket, Data)
    end.

-spec close(transport(), glisten@socket:socket()) -> {ok, nil} |
    {error, glisten@socket:socket_reason()}.
close(Transport, Socket) ->
    case Transport of
        tcp ->
            glisten_tcp_ffi:close(Socket);

        ssl ->
            glisten_ssl_ffi:close(Socket)
    end.

-spec shutdown(transport(), glisten@socket:socket()) -> {ok, nil} |
    {error, glisten@socket:socket_reason()}.
shutdown(Transport, Socket) ->
    case Transport of
        tcp ->
            glisten@tcp:shutdown(Socket);

        ssl ->
            glisten@ssl:shutdown(Socket)
    end.

-spec set_opts(
    transport(),
    glisten@socket:socket(),
    list(glisten@socket@options:tcp_option())
) -> {ok, nil} | {error, nil}.
set_opts(Transport, Socket, Opts) ->
    case Transport of
        tcp ->
            glisten@tcp:set_opts(Socket, Opts);

        ssl ->
            glisten@ssl:set_opts(Socket, Opts)
    end.

-spec negotiated_protocol(transport(), glisten@socket:socket()) -> {ok,
        binary()} |
    {error, binary()}.
negotiated_protocol(Transport, Socket) ->
    case Transport of
        tcp ->
            {error, <<"Can't negotiate protocol on tcp"/utf8>>};

        ssl ->
            glisten_ssl_ffi:negotiated_protocol(Socket)
    end.

-spec peername(transport(), glisten@socket:socket()) -> {ok,
        {{integer(), integer(), integer(), integer()}, integer()}} |
    {error, nil}.
peername(Transport, Socket) ->
    case Transport of
        tcp ->
            inet:peername(Socket);

        ssl ->
            ssl:peername(Socket)
    end.

-spec socket_info(glisten@socket:socket()) -> gleam@dict:dict(gleam@erlang@atom:atom_(), gleam@dynamic:dynamic_()).
socket_info(Socket) ->
    socket:info(Socket).
