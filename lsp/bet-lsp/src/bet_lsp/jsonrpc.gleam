// SPDX-License-Identifier: MIT OR Apache-2.0
//// JSON-RPC 2.0 implementation for LSP

import gleam/dynamic.{type Dynamic}
import gleam/json.{type Json}
import gleam/option.{type Option, None, Some}
import gleam/result

/// JSON-RPC request
pub type Request {
  Request(
    jsonrpc: String,
    id: Option(RequestId),
    method: String,
    params: Option(Dynamic),
  )
}

/// Request ID can be string or integer
pub type RequestId {
  IntId(Int)
  StringId(String)
}

/// JSON-RPC response
pub type Response {
  Response(
    jsonrpc: String,
    id: Option(RequestId),
    result: Option(Json),
    error: Option(ResponseError),
  )
}

/// JSON-RPC error
pub type ResponseError {
  ResponseError(code: Int, message: String, data: Option(Json))
}

// Error codes
pub const parse_error_code = -32700
pub const invalid_request_code = -32600
pub const method_not_found_code = -32601
pub const invalid_params_code = -32602
pub const internal_error_code = -32603

// LSP-specific error codes
pub const server_not_initialized_code = -32002
pub const request_cancelled_code = -32800

/// Parse a JSON-RPC request from string
pub fn parse(json_str: String) -> Result(Request, String) {
  case json.decode(json_str, request_decoder()) {
    Ok(request) -> Ok(request)
    Error(_) -> Error("Failed to parse JSON-RPC request")
  }
}

/// Decode a request from dynamic JSON
fn request_decoder() -> fn(Dynamic) -> Result(Request, List(dynamic.DecodeError)) {
  fn(dyn: Dynamic) {
    use jsonrpc <- result.try(dynamic.field("jsonrpc", dynamic.string)(dyn))
    use method <- result.try(dynamic.field("method", dynamic.string)(dyn))
    use id <- result.try(dynamic.optional_field("id", id_decoder())(dyn))
    use params <- result.try(dynamic.optional_field("params", dynamic.dynamic)(dyn))

    Ok(Request(jsonrpc: jsonrpc, id: id, method: method, params: params))
  }
}

/// Decode request ID
fn id_decoder() -> fn(Dynamic) -> Result(RequestId, List(dynamic.DecodeError)) {
  fn(dyn: Dynamic) {
    case dynamic.int(dyn) {
      Ok(i) -> Ok(IntId(i))
      Error(_) ->
        case dynamic.string(dyn) {
          Ok(s) -> Ok(StringId(s))
          Error(e) -> Error(e)
        }
    }
  }
}

/// Encode a response to JSON string
pub fn encode_response(response: Response) -> String {
  let id_json = case response.id {
    Some(IntId(i)) -> json.int(i)
    Some(StringId(s)) -> json.string(s)
    None -> json.null()
  }

  let base = [
    #("jsonrpc", json.string("2.0")),
    #("id", id_json),
  ]

  let with_result = case response.result {
    Some(r) -> [#("result", r), ..base]
    None -> base
  }

  let with_error = case response.error {
    Some(err) -> [#("error", encode_error(err)), ..with_result]
    None -> with_result
  }

  json.object(with_error)
  |> json.to_string
}

/// Encode an error object
fn encode_error(err: ResponseError) -> Json {
  let base = [
    #("code", json.int(err.code)),
    #("message", json.string(err.message)),
  ]

  let with_data = case err.data {
    Some(d) -> [#("data", d), ..base]
    None -> base
  }

  json.object(with_data)
}

/// Create a success response
pub fn success(id: Option(RequestId), result: Json) -> String {
  encode_response(Response(
    jsonrpc: "2.0",
    id: id,
    result: Some(result),
    error: None,
  ))
}

/// Create an error response
pub fn error(id: Option(RequestId), code: Int, message: String) -> String {
  encode_response(Response(
    jsonrpc: "2.0",
    id: id,
    result: None,
    error: Some(ResponseError(code: code, message: message, data: None)),
  ))
}

/// Parse error response
pub fn parse_error(msg: String) -> String {
  error(None, parse_error_code, "Parse error: " <> msg)
}

/// Method not found response
pub fn method_not_found(id: Option(RequestId)) -> String {
  error(id, method_not_found_code, "Method not found")
}

/// Invalid params response
pub fn invalid_params(id: Option(RequestId), msg: String) -> String {
  error(id, invalid_params_code, "Invalid params: " <> msg)
}

/// Internal error response
pub fn internal_error(id: Option(RequestId), msg: String) -> String {
  error(id, internal_error_code, "Internal error: " <> msg)
}
