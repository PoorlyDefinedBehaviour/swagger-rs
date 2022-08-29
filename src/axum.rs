use std::collections::HashMap;

use crate::{ast, rust_type_to_openapi_type, Component, ParameterSchema};
use syn::{token::Struct, FnArg, Type};

/// Returns true for params of the Query type.
///
/// async fn handler(Query(params): Query<T>) {}
pub fn is_query_param(arg: &FnArg) -> bool {
  ast::pattern_type_without_path(arg)
    .map(|ty| ty == "Query")
    .unwrap_or(false)
    || ast::arg_base_type_without_path(arg) == "Query"
}

pub fn arg_to_parameter_schema(
  components: &HashMap<String, Vec<Component>>,
  arg: &FnArg,
) -> Vec<ParameterSchema> {
  let typ = ast::inner_type_without_path(arg);
  match components.get(&typ) {
    None => vec![ParameterSchema::Enum {
      r#type: String::from("string"),
      default: String::from("unknown"),
      r#enum: vec![String::from("unknown")],
    }],
    Some(components) => components
      .iter()
      .map(|component| ParameterSchema::Primitive {
        r#type: match rust_type_to_openapi_type(&component.r#type) {
          crate::Type::Primitive { r#type } => r#type,
          _ => unreachable!(),
        },
      })
      .collect(),
  }
}

/*
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
enum ParameterSchema {
  Enum {
    r#type: integer,
    default: nu,
    r#enum: Vec<String>,
  },
  Array {
    r#type: String,
    items: Type,
  },
  Integer {
    r#type: String,
    format: String,
  },
}
*/
