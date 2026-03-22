%% SPDX-License-Identifier: MIT OR Apache-2.0
%% Erlang I/O functions for Betlang LSP

-module(bet_lsp_io).
-export([read_line/0, read_bytes/1, write/1, parse_int/1]).

%% Read a line from stdin
-spec read_line() -> {ok, binary()} | {error, binary()}.
read_line() ->
    case io:get_line(standard_io, "") of
        eof -> {error, <<"EOF">>};
        {error, Reason} -> {error, list_to_binary(io_lib:format("~p", [Reason]))};
        Line when is_list(Line) -> {ok, list_to_binary(Line)};
        Line when is_binary(Line) -> {ok, Line}
    end.

%% Read exactly N bytes from stdin
-spec read_bytes(non_neg_integer()) -> {ok, binary()} | {error, binary()}.
read_bytes(N) when N >= 0 ->
    case io:get_chars(standard_io, "", N) of
        eof -> {error, <<"EOF">>};
        {error, Reason} -> {error, list_to_binary(io_lib:format("~p", [Reason]))};
        Data when is_list(Data) -> {ok, list_to_binary(Data)};
        Data when is_binary(Data) -> {ok, Data}
    end.

%% Write to stdout
-spec write(binary()) -> nil.
write(Data) ->
    io:put_chars(standard_io, Data),
    nil.

%% Parse integer from string
-spec parse_int(binary()) -> {ok, integer()} | {error, nil}.
parse_int(Bin) ->
    try
        {ok, binary_to_integer(Bin)}
    catch
        _:_ -> {error, nil}
    end.
