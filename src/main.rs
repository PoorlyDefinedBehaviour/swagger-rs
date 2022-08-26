use std::{collections::HashMap, fs::File, io::Read};

use syn::{Expr, Item, Stmt};
mod item;

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

fn run(path: &str) -> Result<(), Box<dyn std::error::Error>> {
  let mut file = File::open(path).expect("Unable to open file");

  let mut src = String::new();
  file.read_to_string(&mut src).expect("Unable to read file");

  let syntax = syn::parse_file(&src).expect("Unable to parse file");
  dbg!(&syntax.items);

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
          if let Stmt::Local(local_stmt) = stmt {
            if let Some(init) = local_stmt.init {
              if let Expr::MethodCall(method_call) = *init.1 {
                // Calling Router.route("/path", method(controller))
                if method_call.method != "route" {
                  continue;
                }

                let route = match &method_call.args[0] {
                  Expr::Lit(lit) => match &lit.lit {
                    syn::Lit::Str(path) => path.value(),
                    _ => continue,
                  },
                  _ => continue,
                };

                dbg!(&method_call.args);
                let (method, controller) = match &method_call.args[1] {
                  Expr::Call(call) => {
                    let method = match &*call.func {
                      Expr::Path(path) => path.path.segments.last().unwrap().ident.to_string(),
                      _ => continue,
                    };

                    let controller = match call.args.last().unwrap() {
                      Expr::Path(path) => path.path.segments.last().unwrap().ident.to_string(),
                      _ => continue,
                    };

                    (method, controller)
                  }

                  _ => continue,
                };

                routes.insert(
                  route,
                  Controller {
                    method,
                    name: controller,
                    request_body: None,
                    response_body: None,
                  },
                );
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

  println!(
    "found structs: {:?}",
    structs
      .values()
      .map(|v| v.ident.to_string())
      .collect::<Vec<_>>()
  );

  /*
  "/foo": Controller {
    method: "post"
    name: "post_foo",
    request_body: PostFooRequestBody {
      pub username: String
    }
    response_body: PostFooResponseBody {
      pub id: String
    }
    TODO: headers, path params, etc
  }
  */

  Ok(())
}
