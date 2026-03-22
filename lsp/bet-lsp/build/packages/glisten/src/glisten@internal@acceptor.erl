-module(glisten@internal@acceptor).
-compile([no_auto_import, nowarn_unused_vars, nowarn_unused_function, nowarn_nomatch]).

-export([start/1, start_pool/1]).
-export_type([acceptor_message/0, acceptor_error/0, acceptor_state/0, pool/2]).

-type acceptor_message() :: {accept_connection, glisten@socket:listen_socket()}.

-type acceptor_error() :: accept_error | handler_error | control_error.

-type acceptor_state() :: {acceptor_state,
        gleam@erlang@process:subject(acceptor_message()),
        gleam@option:option(glisten@socket:socket()),
        glisten@transport:transport()}.

-type pool(HPZ, HQA) :: {pool,
        glisten@socket:listen_socket(),
        fun((glisten@internal@handler:loop_message(HPZ), HQA, glisten@internal@handler:connection(HPZ)) -> gleam@otp@actor:next(glisten@internal@handler:loop_message(HPZ), HQA)),
        integer(),
        fun((glisten@internal@handler:connection(HPZ)) -> {HQA,
            gleam@option:option(gleam@erlang@process:selector(HPZ))}),
        gleam@option:option(fun((HQA) -> nil)),
        glisten@transport:transport()}.

-spec start(pool(any(), any())) -> {ok,
        gleam@erlang@process:subject(acceptor_message())} |
    {error, gleam@otp@actor:start_error()}.
start(Pool) ->
    gleam@otp@actor:start_spec(
        {spec,
            fun() ->
                Subject = gleam@erlang@process:new_subject(),
                Selector = begin
                    _pipe = gleam_erlang_ffi:new_selector(),
                    gleam@erlang@process:selecting(
                        _pipe,
                        Subject,
                        fun gleam@function:identity/1
                    )
                end,
                gleam@erlang@process:send(
                    Subject,
                    {accept_connection, erlang:element(2, Pool)}
                ),
                {ready,
                    {acceptor_state, Subject, none, erlang:element(7, Pool)},
                    Selector}
            end,
            1000,
            fun(Msg, State) ->
                {acceptor_state, Sender, _, _} = State,
                case Msg of
                    {accept_connection, Listener} ->
                        Res = (gleam@result:then(
                            begin
                                _pipe@1 = glisten@transport:accept(
                                    erlang:element(4, State),
                                    Listener
                                ),
                                gleam@result:replace_error(
                                    _pipe@1,
                                    accept_error
                                )
                            end,
                            fun(Sock) ->
                                gleam@result:then(
                                    begin
                                        _pipe@2 = {handler,
                                            Sock,
                                            erlang:element(3, Pool),
                                            erlang:element(5, Pool),
                                            erlang:element(6, Pool),
                                            erlang:element(7, Pool)},
                                        _pipe@3 = glisten@internal@handler:start(
                                            _pipe@2
                                        ),
                                        gleam@result:replace_error(
                                            _pipe@3,
                                            handler_error
                                        )
                                    end,
                                    fun(Start) -> _pipe@4 = Sock,
                                        _pipe@5 = glisten@transport:controlling_process(
                                            erlang:element(4, State),
                                            _pipe@4,
                                            gleam@erlang@process:subject_owner(
                                                Start
                                            )
                                        ),
                                        _pipe@6 = gleam@result:replace_error(
                                            _pipe@5,
                                            control_error
                                        ),
                                        gleam@result:map(
                                            _pipe@6,
                                            fun(_) ->
                                                gleam@erlang@process:send(
                                                    Start,
                                                    {internal, ready}
                                                )
                                            end
                                        ) end
                                )
                            end
                        )),
                        case Res of
                            {error, Reason} ->
                                glisten@internal@logger:error(
                                    {<<"Failed to accept/start handler"/utf8>>,
                                        Reason}
                                ),
                                {stop,
                                    {abnormal,
                                        <<"Failed to accept/start handler"/utf8>>}};

                            _ ->
                                gleam@otp@actor:send(
                                    Sender,
                                    {accept_connection, Listener}
                                ),
                                gleam@otp@actor:continue(State)
                        end
                end
            end}
    ).

-spec start_pool(pool(any(), any())) -> {ok,
        gleam@erlang@process:subject(gleam@otp@supervisor:message())} |
    {error, gleam@otp@actor:start_error()}.
start_pool(Pool) ->
    gleam@otp@supervisor:start_spec(
        {spec,
            nil,
            100,
            1,
            fun(Children) ->
                _pipe = gleam@iterator:range(0, erlang:element(4, Pool)),
                gleam@iterator:fold(
                    _pipe,
                    Children,
                    fun(Children@1, _) ->
                        gleam@otp@supervisor:add(
                            Children@1,
                            gleam@otp@supervisor:worker(
                                fun(_) -> start(Pool) end
                            )
                        )
                    end
                )
            end}
    ).
