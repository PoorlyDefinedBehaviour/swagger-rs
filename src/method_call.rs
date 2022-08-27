use std::collections::HashMap;

use syn::{Expr, ExprMethodCall, ItemFn, PatType};

use crate::Controller;

pub fn handle_method_call(routes: &mut HashMap<String, Controller>, method_call: ExprMethodCall) {
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

fn get_function_params(func: ItemFn) {
  for input in func.sig.inputs.into_iter() {
    match input {
      syn::FnArg::Receiver(_) => todo!(),
      syn::FnArg::Typed(pat_type) => match *pat_type.pat {
        syn::Pat::Box(_) => todo!(),
        syn::Pat::Ident(_) => todo!(),
        syn::Pat::Lit(_) => todo!(),
        syn::Pat::Macro(_) => todo!(),
        syn::Pat::Or(_) => todo!(),
        syn::Pat::Path(_) => todo!(),
        syn::Pat::Range(_) => todo!(),
        syn::Pat::Reference(_) => todo!(),
        syn::Pat::Rest(_) => todo!(),
        syn::Pat::Slice(_) => todo!(),
        syn::Pat::Struct(_) => todo!(),
        syn::Pat::Tuple(_) => todo!(),
        syn::Pat::Type(_) => todo!(),
        syn::Pat::Verbatim(_) => todo!(),
        syn::Pat::Wild(_) => todo!(),
        syn::Pat::TupleStruct(tuple_struct) => {
          let path = tuple_struct
            .path
            .segments
            .into_iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()
            .join("::");

          todo!()
        }
        _ => todo!(),
      },
    }
  }
}
