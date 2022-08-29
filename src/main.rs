use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;

use std::{collections::HashMap, fs::File, io::Read};
use syn::{Expr, ExprMethodCall, Item, ItemFn, ItemStruct, Stmt};

mod ast;
mod axum;
mod item;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Eq)]
pub struct Controller {
  pub method: String,
  pub name: String,
  pub request_body: Option<String>,
  pub response_body: Option<String>,
}

#[derive(Debug)]
pub struct RouteHandler {
  pub method: String,
  pub handler_name: String,
}

#[derive(Debug)]
struct Component {
  pub r#type: String,
  pub required: bool,
  pub field_name: Option<String>,
}

fn main() {
  run("/home/bruno/dev/rust/swagger/src/test.rs").unwrap();
}

lazy_static! {
  static ref OPTION_REGEX: Regex = Regex::new(r#"Option<(.+)>"#).unwrap();
  static ref SERDE_YAML_ENUM_TAG_REGEX: Regex = Regex::new(r#"!.+"#).unwrap();
}

#[derive(Debug)]
struct AstTraverser {
  structs: HashMap<String, ItemStruct>,
  routes: HashMap<String, RouteHandler>,
  used_types: Vec<String>,
  fn_declarations: HashMap<String, ItemFn>,
  components: HashMap<String, Vec<Component>>,
}

impl AstTraverser {
  pub fn new() -> Self {
    Self {
      structs: HashMap::new(),
      fn_declarations: HashMap::new(),
      routes: HashMap::new(),
      used_types: Vec::new(),
      components: HashMap::new(),
    }
  }

  pub fn traverse(&mut self, syntax: syn::File) {
    for item in syntax.items.into_iter() {
      match item {
        Item::Struct(struct_) => {
          self.structs.insert(struct_.ident.to_string(), struct_);
        }
        Item::Fn(func) => {
          let func_name = func.sig.ident.to_string();
          if func_name != "main" {
            self.fn_declarations.insert(func_name, func.clone());
          }

          for stmt in func.block.stmts.into_iter() {
            match stmt {
              Stmt::Item(_) | Stmt::Expr(_) => continue,
              Stmt::Semi(expr, _tokens) => match expr {
                Expr::MethodCall(method_call) => self.handle_method_call(method_call),
                _ => continue,
              },
              Stmt::Local(local_stmt) => {
                if let Some(init) = local_stmt.init {
                  if let Expr::MethodCall(method_call) = *init.1 {
                    self.handle_method_call(method_call)
                  }
                }
              }
            }
          }
        }
        _ => {}
      }
    }
  }

  fn handle_method_call(&mut self, method_call: ExprMethodCall) {
    // Calling Router.route("/path", method(controller))
    if method_call.method != "route" {
      return;
    }

    let route = match &method_call.args[0] {
      Expr::Lit(lit) => match &lit.lit {
        syn::Lit::Str(path) => path.value(),
        _ => return,
      },
      _ => return,
    };

    let (method, controller) = match &method_call.args[1] {
      Expr::Call(call) => {
        let method = match &*call.func {
          Expr::Path(path) => path.path.segments.last().unwrap().ident.to_string(),
          _ => return,
        };

        let controller = match call.args.last().unwrap() {
          Expr::Path(path) => path.path.segments.last().unwrap().ident.to_string(),
          _ => return,
        };

        (method, controller)
      }

      _ => return,
    };

    self.routes.insert(
      route,
      RouteHandler {
        method,
        handler_name: controller,
      },
    );
  }

  pub fn build_type_components(&mut self) {
    for used_type in self.used_types.clone().iter() {
      let struct_ = self.structs.get(used_type).cloned();
      match struct_ {
        // It is a type that wasn't defined in the project.
        None => {
          if let Some(inner_type) = OPTION_REGEX
            .captures(used_type)
            .and_then(|capture| capture.get(1))
          {
            self.components.insert(
              inner_type.as_str().to_owned(),
              vec![Component {
                r#type: inner_type.as_str().to_owned(),
                required: false,
                field_name: None,
              }],
            );
          } else {
            self.components.insert(
              used_type.clone(),
              vec![Component {
                r#type: used_type.clone(),
                required: false,
                field_name: None,
              }],
            );
          }
        }
        Some(struct_) => {
          self.build_type_components_from_struct(&struct_);
        }
      }
    }
  }

  fn build_type_components_from_struct(&mut self, struct_: &ItemStruct) {
    let struct_name = struct_.ident.to_string();

    self.components.insert(struct_name.clone(), vec![]);

    for field in struct_.fields.iter() {
      let field_name = field.ident.clone().unwrap().to_string();

      self.build_type_components_from_type(&struct_name, &field_name, &field.ty);
    }
  }

  fn build_type_components_from_type(
    &mut self,
    struct_name: &str,
    field_name: &str,
    ty: &syn::Type,
  ) {
    match ty {
      syn::Type::Array(_) => todo!(),
      syn::Type::BareFn(_) => todo!(),
      syn::Type::Group(_) => todo!(),
      syn::Type::ImplTrait(_) => todo!(),
      syn::Type::Infer(_) => todo!(),
      syn::Type::Macro(_) => todo!(),
      syn::Type::Never(_) => todo!(),
      syn::Type::Paren(_) => todo!(),
      syn::Type::Ptr(_) => todo!(),
      syn::Type::Reference(_) => todo!(),
      syn::Type::Slice(_) => todo!(),
      syn::Type::TraitObject(_) => todo!(),
      syn::Type::Verbatim(_) => todo!(),
      syn::Type::Path(type_path) => {
        let segment = type_path
          .path
          .segments
          .iter()
          .map(|segment| {
            let ident = segment.ident.to_string();

            let arguments = match &segment.arguments {
              syn::PathArguments::None => vec![],
              syn::PathArguments::AngleBracketed(args) => args
                .args
                .iter()
                .map(|arg| match arg {
                  syn::GenericArgument::Lifetime(_) => todo!(),
                  syn::GenericArgument::Binding(_) => todo!(),
                  syn::GenericArgument::Constraint(_) => todo!(),
                  syn::GenericArgument::Const(_) => todo!(),
                  syn::GenericArgument::Type(typ) => match typ {
                    syn::Type::Path(type_path) => type_path
                      .path
                      .segments
                      .iter()
                      .map(|segment| segment.ident.to_string())
                      .collect::<Vec<_>>()
                      .join("::"),
                    _ => todo!(),
                  },
                })
                .collect(),
              syn::PathArguments::Parenthesized(_) => todo!(),
            };

            if arguments.is_empty() {
              ident
            } else {
              format!("{}<{}>", ident, arguments.join(", "))
            }
          })
          .collect::<Vec<_>>()
          .join("::");

        println!(
          "aaaaaa struct_name={} field_name={}, segment={:?}",
          struct_name, field_name, segment
        );

        match self.structs.get(&segment).cloned() {
          None => {
            println!(
              "aaaaaa segment is primitive struct_name={} segment={}",
              struct_name, segment
            );
            self
              .components
              .get_mut(struct_name)
              .unwrap()
              .push(Component {
                required: {
                  let required = !OPTION_REGEX.is_match(&segment);
                  println!("aaaaaa field is required field_name={}", segment);
                  required
                },
                field_name: Some(field_name.to_string()),
                r#type: OPTION_REGEX
                  .captures(&segment)
                  .and_then(|capture| capture.get(1))
                  .map(|matches| matches.as_str().to_string())
                  .unwrap_or_else(|| segment),
              });
          }
          Some(struct_) => {
            // We found a field that has a struct type:
            //
            // struct S1 {
            //   field_1: i32,
            //   field_2: S2 <-- here
            // }
            //
            // struct S2 {
            //   field_1: String
            // }
            //
            // So we add a definition for field2 and recurse into the struct S2.
            let field_struct_name = struct_.ident.to_string();
            self
              .components
              .get_mut(struct_name)
              .unwrap()
              .push(Component {
                required: !OPTION_REGEX.is_match(&field_struct_name),
                r#type: field_struct_name,
                field_name: Some(field_name.to_string()),
              });
            self.build_type_components_from_struct(&struct_);
          }
        }
      }
      syn::Type::Tuple(tuple_struct) => {
        for elem in tuple_struct.elems.iter() {
          self.build_type_components_from_type(struct_name, field_name, elem);
        }
      }
      _ => todo!(),
    }
  }

  pub fn build_resource(&self) -> Resource {
    Resource {
      openapi: String::from("3.0.3"),
      info: Info {
        title: String::from("My rest API"),
        description: String::from("i don't know"),
        version: env!("CARGO_PKG_VERSION").to_owned(),
      },
      servers: vec![],
      paths: {
        let mut paths = HashMap::new();

        for (route, controller) in self.routes.iter() {
          let controller_fn = self.fn_declarations.get(&controller.handler_name).unwrap();
          paths.insert(
            route.clone(),
            HashMap::from([(
              controller.method.clone(),
              Path {
                summary: Some(String::from("TODO")),
                parameters: {
                  let parameters = Vec::new();

                for parameter in controller_fn
                .sig
                .inputs
                .iter()
                .filter(|parameter| axum::is_query_param(parameter)) {
                  parameters
                }
                controller_fn
                  .sig
                  .inputs
                  .iter()
                  .filter(|parameter| axum::is_query_param(parameter))
                  .map(|parameter| Parameter {
                    name: ast::param_name(parameter),
                    r#in: String::from("query"),
                    description: String::from("TODO"),
                    required: true,
                    explode: false,
                    schema: ParameterSchema::Integer {
                      r#type: String::from("integer"),
                      format: String::from("int64"),
                    },
                  })
                  .collect();
                parameters
                },
                request_body: RequestBody {
                  // TODO: fixme
                  required: true,
                  content: Content {
                    content_type: ContentType {
                      schema: Schema::Ref {
                        r#ref: format!(
                          "#/components/schemas/{}",
                          "TODO",
                          /*controller_fn
                          .parameters
                          .iter()
                          .find(|param| param.is_json_request_body())
                          .map(|param| param.full_path_inner_type())
                          .unwrap()*/
                        ),
                      },
                    },
                  },
                },
                description: String::from("TODO"),
                responses: HashMap::from([(
                  String::from("200"),
                  Response::DescriptionOnly {
                    description: String::from("OK"),
                  },
                )]),
              },
            )]),
          );
        }

        paths
      },
      components: {
        let mut components = HashMap::new();

        for (type_name, fields) in self.components.iter() {
          components.insert(
            type_name.clone(),
            ResourceComponent {
              r#type: if !fields.is_empty() {
                String::from("object")
              } else {
                // TODO: handle other types (e.g. integer)
                String::from("string")
              },
              required: fields
                .iter()
                .filter(|field| field.required)
                .map(|field| field.field_name.clone().unwrap())
                .collect(),
              properties: fields
                .iter()
                .map(|field| {
                  (
                    field.field_name.clone().unwrap(),
                    rust_type_to_openapi_type(&field.r#type),
                  )
                })
                .collect(),
            },
          );
        }

        ComponentsSchemas {
          schemas: components,
        }
      },
    }
  }

  fn debug(&self) {
    println!("found routes {:?}", self.routes);

    println!("found functions: {:#?}", self.fn_declarations);

    println!(
      "found structs: {:?}",
      self
        .structs
        .values()
        .map(|v| v.ident.to_string())
        .collect::<Vec<_>>()
    );

    println!("type components: {:#?}", self.components);
  }
}

fn run(path: &str) -> Result<String, Box<dyn std::error::Error>> {
  let mut file = File::open(path).expect("Unable to open file");

  let mut src = String::new();
  file.read_to_string(&mut src).expect("Unable to read file");

  let syntax = syn::parse_file(&src).expect("Unable to parse file");

  let mut traverser = AstTraverser::new();
  traverser.traverse(syntax);

  traverser.used_types.push(String::from("RequestBody"));
  traverser.build_type_components();
  traverser.debug();

  println!(
    "{}",
    SERDE_YAML_ENUM_TAG_REGEX.replace_all(&serde_yaml::to_string(&traverser.build_resource())?, "")
  );

  todo!()
}

fn rust_type_to_openapi_type(rust_type: &str) -> Type {
  let typ = if rust_type.eq_ignore_ascii_case("string") {
    "string"
  } else if rust_type.eq_ignore_ascii_case("i8") || rust_type.eq_ignore_ascii_case("u8") {
    "byte"
  } else if rust_type.eq_ignore_ascii_case("i16")
    || rust_type.eq_ignore_ascii_case("i32")
    || rust_type.eq_ignore_ascii_case("i64")
    || rust_type.eq_ignore_ascii_case("i128")
    || rust_type.eq_ignore_ascii_case("u16")
    || rust_type.eq_ignore_ascii_case("u32")
    || rust_type.eq_ignore_ascii_case("u64")
    || rust_type.eq_ignore_ascii_case("u128")
  {
    "integer"
  } else if rust_type.eq_ignore_ascii_case("f32") || rust_type.eq_ignore_ascii_case("f64") {
    "number"
  } else if rust_type.eq_ignore_ascii_case("boolean") {
    "boolean"
  } else {
    return Type::Ref {
      r#ref: format!("#/components/schemas/{}", rust_type),
    };
  };

  Type::Primitive {
    r#type: String::from(typ),
  }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Resource {
  pub openapi: String,
  pub info: Info,
  pub servers: Vec<Server>,
  pub paths: HashMap<String, HashMap<String, Path>>,
  pub components: ComponentsSchemas,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Info {
  pub title: String,
  pub description: String,
  pub version: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Server {
  pub url: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ComponentsSchemas {
  pub schemas: HashMap<String, ResourceComponent>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ResourceComponent {
  pub r#type: String,
  pub required: Vec<String>,
  pub properties: HashMap<String, Type>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Path {
  pub summary: Option<String>,
  pub description: String,
  pub parameters: Vec<Parameter>,
  pub request_body: RequestBody,
  pub responses: HashMap<String, Response>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Parameter {
  pub name: String,
  pub r#in: String,
  pub description: String,
  pub required: bool,
  // TODO: what does this even mean?
  pub explode: bool,
  pub schema: ParameterSchema,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
enum ParameterSchema {
  Enum {
    r#type: String,
    default: String,
    r#enum: Vec<String>,
  },
  Array {
    r#type: String,
    items: Type,
  },
  Primitive {
    r#type: String,
  },
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
enum Response {
  DescriptionOnly { description: String },
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RequestBody {
  pub required: bool,
  pub content: Content,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Content {
  #[serde(rename = "application/json")]
  pub content_type: ContentType,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ContentType {
  pub schema: Schema,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
enum Schema {
  Properties {
    r#type: String,
    properties: Option<HashMap<String, Type>>,
  },
  Ref {
    #[serde(rename = "$ref")]
    r#ref: String,
  },
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Type {
  Primitive {
    r#type: String,
  },
  Ref {
    #[serde(rename = "$ref")]
    r#ref: String,
  },
}
