use serde::Serialize;
use std::{collections::HashMap, fs::File, io::Read};
use syn::{token::Return, Expr, ExprMethodCall, Item, Stmt};
mod item;
mod method_call;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Eq)]
struct Controller {
  pub method: String,
  pub name: String,
  pub request_body: Option<String>,
  pub response_body: Option<String>,
}

fn main() {
  run("/home/bruno/dev/rust/swagger/src/test.rs").unwrap();
}

fn run(path: &str) -> Result<String, Box<dyn std::error::Error>> {
  let mut file = File::open(path).expect("Unable to open file");

  let mut src = String::new();
  file.read_to_string(&mut src).expect("Unable to read file");

  let syntax = syn::parse_file(&src).expect("Unable to parse file");

  let mut routes = HashMap::new();

  let mut fn_declarations = HashMap::new();

  let mut structs = HashMap::new();

  for item in syntax.items.into_iter() {
    match item {
      Item::Struct(struct_) => {
        structs.insert(struct_.ident.to_string(), struct_);
      }
      Item::Fn(func) => {
        fn_declarations.insert(func.sig.ident.to_string(), func.clone());

        for stmt in func.block.stmts.into_iter() {
          // if let Stmt::Expr()
          match stmt {
            Stmt::Item(_) => continue,
            Stmt::Expr(_) => {
              todo!()
            }
            Stmt::Semi(expr, _tokens) => match expr {
              Expr::MethodCall(method_call) => handle_method_call(&mut routes, method_call),
              _ => continue,
            },
            Stmt::Local(local_stmt) => {
              if let Some(init) = local_stmt.init {
                if let Expr::MethodCall(method_call) = *init.1 {
                  handle_method_call(&mut routes, method_call)
                }
              }
            }
          }
        }
      }
      _ => {}
    }
  }

  println!("found routes {:?}", routes);

  println!(
    "found functions: {:?}",
    fn_declarations.keys().collect::<Vec<_>>()
  );

  dbg!(&fn_declarations);

  println!(
    "found structs: {:?}",
    structs
      .values()
      .map(|v| v.ident.to_string())
      .collect::<Vec<_>>()
  );

  let resource = Resource {
    paths: {
      let mut paths = HashMap::new();

      for (path, controller) in routes {
        paths.insert(
          path,
          HashMap::from([(
            controller.method,
            Path {
              summary: Some(String::from("TODO")),
              request_body: RequestBody {
                required: true,
                content: Content {
                  content_type: ContentType {
                    schema: Schema {
                      r#type: String::from("object"),
                      properties: HashMap::from([(
                        String::from("username"),
                        Type {
                          r#type: String::from("string"),
                        },
                      )]),
                    },
                  },
                },
              },
            },
          )]),
        );
      }

      paths
    },
  };

  Ok(serde_json::to_string_pretty(&resource)?)
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Resource {
  pub paths: HashMap<String, HashMap<String, Path>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Path {
  pub summary: Option<String>,
  pub request_body: RequestBody,
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
struct Schema {
  pub r#type: String,
  pub properties: HashMap<String, Type>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Type {
  pub r#type: String,
}
