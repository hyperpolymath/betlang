-record(connection, {
    client_ip :: {ok, {{integer(), integer(), integer(), integer()}, integer()}} |
        {error, nil},
    socket :: glisten@socket:socket(),
    transport :: glisten@transport:transport(),
    subject :: gleam@erlang@process:subject(glisten@internal@handler:message(any()))
}).
