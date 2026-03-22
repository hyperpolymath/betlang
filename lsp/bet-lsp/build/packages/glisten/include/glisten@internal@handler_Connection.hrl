-record(connection, {
    client_ip :: {ok, {{integer(), integer(), integer(), integer()}, integer()}} |
        {error, nil},
    socket :: glisten@socket:socket(),
    transport :: glisten@transport:transport(),
    sender :: gleam@erlang@process:subject(glisten@internal@handler:message(any()))
}).
