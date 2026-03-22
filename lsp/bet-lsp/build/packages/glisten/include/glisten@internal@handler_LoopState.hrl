-record(loop_state, {
    client_ip :: {ok, {{integer(), integer(), integer(), integer()}, integer()}} |
        {error, nil},
    socket :: glisten@socket:socket(),
    sender :: gleam@erlang@process:subject(glisten@internal@handler:message(any())),
    transport :: glisten@transport:transport(),
    data :: any()
}).
