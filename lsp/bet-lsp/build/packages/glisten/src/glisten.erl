-module(glisten).
-compile([no_auto_import, nowarn_unused_vars, nowarn_unused_function, nowarn_nomatch]).

-export([send/2, handler/2, with_close/2, with_pool_size/2, serve/2, serve_ssl/4]).
-export_type([start_error/0, message/1, connection/1, handler/2]).

-type start_error() :: listener_closed |
    listener_timeout |
    acceptor_timeout |
    {acceptor_failed, gleam@erlang@process:exit_reason()} |
    {acceptor_crashed, gleam@dynamic:dynamic_()} |
    {system_error, glisten@socket:socket_reason()}.

-type message(HUP) :: {packet, bitstring()} | {user, HUP}.

-type connection(HUQ) :: {connection,
        {ok, {{integer(), integer(), integer(), integer()}, integer()}} |
            {error, nil},
        glisten@socket:socket(),
        glisten@transport:transport(),
        gleam@erlang@process:subject(glisten@internal@handler:message(HUQ))}.

-opaque handler(HUR, HUS) :: {handler,
        fun((connection(HUR)) -> {HUS,
            gleam@option:option(gleam@erlang@process:selector(HUR))}),
        fun((message(HUR), HUS, connection(HUR)) -> gleam@otp@actor:next(message(HUR), HUS)),
        gleam@option:option(fun((HUS) -> nil)),
        integer()}.

-spec send(connection(any()), gleam@bytes_builder:bytes_builder()) -> {ok, nil} |
    {error, glisten@socket:socket_reason()}.
send(Conn, Msg) ->
    glisten@transport:send(
        erlang:element(4, Conn),
        erlang:element(3, Conn),
        Msg
    ).

-spec convert_on_init(
    fun((connection(HVP)) -> {HVR,
        gleam@option:option(gleam@erlang@process:selector(HVP))})
) -> fun((glisten@internal@handler:connection(HVP)) -> {HVR,
    gleam@option:option(gleam@erlang@process:selector(HVP))}).
convert_on_init(On_init) ->
    fun(Conn) ->
        Connection = {connection,
            erlang:element(2, Conn),
            erlang:element(3, Conn),
            erlang:element(4, Conn),
            erlang:element(5, Conn)},
        On_init(Connection)
    end.

-spec handler(
    fun((connection(HVX)) -> {HVZ,
        gleam@option:option(gleam@erlang@process:selector(HVX))}),
    fun((message(HVX), HVZ, connection(HVX)) -> gleam@otp@actor:next(message(HVX), HVZ))
) -> handler(HVX, HVZ).
handler(On_init, Loop) ->
    {handler, On_init, Loop, none, 10}.

-spec map_user_selector(gleam@erlang@process:selector(message(HVE))) -> gleam@erlang@process:selector(glisten@internal@handler:loop_message(HVE)).
map_user_selector(Selector) ->
    gleam_erlang_ffi:map_selector(Selector, fun(Value) -> case Value of
                {packet, Msg} ->
                    {packet, Msg};

                {user, Msg@1} ->
                    {custom, Msg@1}
            end end).

-spec convert_loop(
    fun((message(HVJ), HVK, connection(HVJ)) -> gleam@otp@actor:next(message(HVJ), HVK))
) -> fun((glisten@internal@handler:loop_message(HVJ), HVK, glisten@internal@handler:connection(HVJ)) -> gleam@otp@actor:next(glisten@internal@handler:loop_message(HVJ), HVK)).
convert_loop(Loop) ->
    fun(Msg, Data, Conn) ->
        Conn@1 = {connection,
            erlang:element(2, Conn),
            erlang:element(3, Conn),
            erlang:element(4, Conn),
            erlang:element(5, Conn)},
        case Msg of
            {packet, Msg@1} ->
                case Loop({packet, Msg@1}, Data, Conn@1) of
                    {continue, Data@1, Selector} ->
                        {continue,
                            Data@1,
                            gleam@option:map(Selector, fun map_user_selector/1)};

                    {stop, Reason} ->
                        {stop, Reason}
                end;

            {custom, Msg@2} ->
                case Loop({user, Msg@2}, Data, Conn@1) of
                    {continue, Data@2, Selector@1} ->
                        {continue,
                            Data@2,
                            gleam@option:map(
                                Selector@1,
                                fun map_user_selector/1
                            )};

                    {stop, Reason@1} ->
                        {stop, Reason@1}
                end
        end
    end.

-spec with_close(handler(HWG, HWH), fun((HWH) -> nil)) -> handler(HWG, HWH).
with_close(Handler, On_close) ->
    erlang:setelement(4, Handler, {some, On_close}).

-spec with_pool_size(handler(HWM, HWN), integer()) -> handler(HWM, HWN).
with_pool_size(Handler, Size) ->
    erlang:setelement(5, Handler, Size).

-spec serve(handler(any(), any()), integer()) -> {ok,
        gleam@erlang@process:subject(gleam@otp@supervisor:message())} |
    {error, start_error()}.
serve(Handler, Port) ->
    _pipe = Port,
    _pipe@1 = glisten@tcp:listen(_pipe, []),
    _pipe@2 = gleam@result:map_error(_pipe@1, fun(Err) -> case Err of
                closed ->
                    listener_closed;

                timeout ->
                    listener_timeout;

                Err@1 ->
                    {system_error, Err@1}
            end end),
    gleam@result:then(
        _pipe@2,
        fun(Socket) ->
            _pipe@3 = {pool,
                Socket,
                convert_loop(erlang:element(3, Handler)),
                erlang:element(5, Handler),
                convert_on_init(erlang:element(2, Handler)),
                erlang:element(4, Handler),
                tcp},
            _pipe@4 = glisten@internal@acceptor:start_pool(_pipe@3),
            gleam@result:map_error(_pipe@4, fun(Err@2) -> case Err@2 of
                        init_timeout ->
                            acceptor_timeout;

                        {init_failed, Reason} ->
                            {acceptor_failed, Reason};

                        {init_crashed, Reason@1} ->
                            {acceptor_crashed, Reason@1}
                    end end)
        end
    ).

-spec serve_ssl(handler(any(), any()), integer(), binary(), binary()) -> {ok,
        gleam@erlang@process:subject(gleam@otp@supervisor:message())} |
    {error, start_error()}.
serve_ssl(Handler, Port, Certfile, Keyfile) ->
    _assert_subject = glisten_ssl_ffi:start_ssl(),
    {ok, _} = case _assert_subject of
        {ok, _} -> _assert_subject;
        _assert_fail ->
            erlang:error(#{gleam_error => let_assert,
                        message => <<"Assertion pattern match failed"/utf8>>,
                        value => _assert_fail,
                        module => <<"glisten"/utf8>>,
                        function => <<"serve_ssl"/utf8>>,
                        line => 206})
    end,
    _pipe = Port,
    _pipe@1 = glisten@ssl:listen(
        _pipe,
        [{certfile, Certfile}, {keyfile, Keyfile}]
    ),
    _pipe@2 = gleam@result:map_error(_pipe@1, fun(Err) -> case Err of
                closed ->
                    listener_closed;

                timeout ->
                    listener_timeout;

                Err@1 ->
                    {system_error, Err@1}
            end end),
    gleam@result:then(
        _pipe@2,
        fun(Socket) ->
            _pipe@3 = {pool,
                Socket,
                convert_loop(erlang:element(3, Handler)),
                erlang:element(5, Handler),
                convert_on_init(erlang:element(2, Handler)),
                erlang:element(4, Handler),
                ssl},
            _pipe@4 = glisten@internal@acceptor:start_pool(_pipe@3),
            gleam@result:map_error(_pipe@4, fun(Err@2) -> case Err@2 of
                        init_timeout ->
                            acceptor_timeout;

                        {init_failed, Reason} ->
                            {acceptor_failed, Reason};

                        {init_crashed, Reason@1} ->
                            {acceptor_crashed, Reason@1}
                    end end)
        end
    ).
