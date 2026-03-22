// SPDX-License-Identifier: MIT OR Apache-2.0
//// LSP Protocol types and constants

/// Text document sync kinds
pub const sync_none = 0
pub const sync_full = 1
pub const sync_incremental = 2

/// Completion item kinds
pub const completion_text = 1
pub const completion_method = 2
pub const completion_function = 3
pub const completion_constructor = 4
pub const completion_field = 5
pub const completion_variable = 6
pub const completion_class = 7
pub const completion_interface = 8
pub const completion_module = 9
pub const completion_property = 10
pub const completion_unit = 11
pub const completion_value = 12
pub const completion_enum = 13
pub const completion_keyword = 14
pub const completion_snippet = 15
pub const completion_color = 16
pub const completion_file = 17
pub const completion_reference = 18
pub const completion_folder = 19
pub const completion_enum_member = 20
pub const completion_constant = 21
pub const completion_struct = 22
pub const completion_event = 23
pub const completion_operator = 24
pub const completion_type_parameter = 25

/// Symbol kinds
pub const symbol_file = 1
pub const symbol_module = 2
pub const symbol_namespace = 3
pub const symbol_package = 4
pub const symbol_class = 5
pub const symbol_method = 6
pub const symbol_property = 7
pub const symbol_field = 8
pub const symbol_constructor = 9
pub const symbol_enum = 10
pub const symbol_interface = 11
pub const symbol_function = 12
pub const symbol_variable = 13
pub const symbol_constant = 14
pub const symbol_string = 15
pub const symbol_number = 16
pub const symbol_boolean = 17
pub const symbol_array = 18
pub const symbol_object = 19
pub const symbol_key = 20
pub const symbol_null = 21
pub const symbol_enum_member = 22
pub const symbol_struct = 23
pub const symbol_event = 24
pub const symbol_operator = 25
pub const symbol_type_parameter = 26

/// Diagnostic severity
pub const severity_error = 1
pub const severity_warning = 2
pub const severity_information = 3
pub const severity_hint = 4

/// Diagnostic tags
pub const tag_unnecessary = 1
pub const tag_deprecated = 2

/// Message types for showMessage
pub const message_error = 1
pub const message_warning = 2
pub const message_info = 3
pub const message_log = 4
