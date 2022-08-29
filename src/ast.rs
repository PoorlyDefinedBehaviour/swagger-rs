use syn::{FnArg, GenericArgument};

/// Query in fn handler(axum::Query(query): ...)
pub fn pattern_type_without_path(arg: &FnArg) -> Option<String> {
  dbg!(arg);
  match arg {
    FnArg::Receiver(_) => todo!(),
    FnArg::Typed(pat_type) => {
      match &*pat_type.pat {
        // fn handler(Query(query): Query<...>)
        syn::Pat::TupleStruct(tuple_struct) => tuple_struct
          .path
          .segments
          .last()
          .map(|segment| segment.ident.to_string()),
        _ => todo!(),
      }
    }
  }
}

pub fn has_option_type(arg: &FnArg) -> bool {
  inner_type_without_path(arg) == "Option"
}

/// T in fn handler(Query(q): Query<T>)
pub fn inner_type_without_path(arg: &FnArg) -> String {
  match arg {
    FnArg::Receiver(_) => todo!(),
    FnArg::Typed(pat_type) => match &*pat_type.ty {
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
      syn::Type::Tuple(_) => todo!(),
      syn::Type::Verbatim(_) => todo!(),
      syn::Type::Path(type_path) => {
        let segment = type_path.path.segments.last().unwrap();

        match segment.arguments {
          syn::PathArguments::AngleBracketed(args) => {
            let generic_arg = args.args.last().unwrap();
            match generic_arg {
              GenericArgument::Type(type_path) => match type_path {
                syn::Type::Path(path) => match path.path.segments.last() {
                  None => segment.ident.to_string(),
                  Some(segment) => segment.ident.to_string(),
                },
                _ => todo!(),
              },
              _ => todo!(),
            }
          }
          _ => todo!(),
        }
      }
      _ => todo!(),
    },
  }
}

/// Query in fn handler(query: axum::Query<T>)
pub fn arg_base_type_without_path(arg: &FnArg) -> String {
  match arg {
    FnArg::Receiver(_) => todo!(),
    FnArg::Typed(pat_type) => match &*pat_type.ty {
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
      syn::Type::Tuple(_) => todo!(),
      syn::Type::Verbatim(_) => todo!(),
      syn::Type::Path(type_path) => type_path.path.segments.last().unwrap().ident.to_string(),
      _ => todo!(),
    },
  }
}

pub fn param_name(arg: &FnArg) -> String {
  match arg {
    FnArg::Receiver(_) => todo!(),
    FnArg::Typed(pat_type) => {
      match &*pat_type.pat {
        // fn handler(Query(query): Query<...>)
        syn::Pat::TupleStruct(tuple_struct) => match tuple_struct.pat.elems.last().unwrap() {
          syn::Pat::Ident(ident) => ident.ident.to_string(),
          _ => todo!(),
        },
        _ => todo!(),
      }
    }
  }
}

/*pub fn get_function_parameters(func: &ItemFn) -> Vec<String> {
  let parameters: Vec<_> = func
    .sig
    .inputs
    .iter()
    .map(|input| match input {
      syn::FnArg::Receiver(_) => todo!(),
      syn::FnArg::Typed(pat_type) => {
        let (path, pattern) = match &*pat_type.pat {
          syn::Pat::TupleStruct(tuple_struct) => {
            let path = if tuple_struct.path.segments.is_empty() {
              None
            } else {
              Some(
                tuple_struct
                  .path
                  .segments
                  .iter()
                  .map(|segment| segment.ident.to_string())
                  .collect::<Vec<_>>(),
              )
            };

            let pattern = tuple_struct
              .pat
              .elems
              .iter()
              .map(|elem| match elem {
                syn::Pat::Ident(ident) => ident.ident.to_string(),
                _ => todo!(),
              })
              .collect::<Vec<_>>();

            (path, pattern)
          }
          _ => todo!(),
        };

        let r#type = match &*pat_type.ty {
          syn::Type::Path(type_path) => self.type_path_to_simplified_path(type_path),
          _ => todo!(),
        };

        FnParameter {
          path,
          pattern,
          r#type: *pat_type.ty.clone(),
        }
      }
    })
    .collect();
}
*/

/*

fn type_path_to_simplified_path(&self, type_path: &TypePath) -> SimplifiedTypePath {
  let segments = type_path
    .path
    .segments
    .iter()
    .map(|segment| {
      let ident = segment.ident.to_string();

      let arguments = match &segment.arguments {
        syn::PathArguments::None => vec![],
        syn::PathArguments::Parenthesized(_) => todo!(),
        syn::PathArguments::AngleBracketed(args) => args
          .args
          .iter()
          .filter_map(|arg| match arg {
            syn::GenericArgument::Lifetime(_)
            | syn::GenericArgument::Binding(_)
            | syn::GenericArgument::Constraint(_)
            | syn::GenericArgument::Const(_) => None,
            syn::GenericArgument::Type(typ) => match typ {
              syn::Type::Path(type_path) => {
                dbg!(self.type_path_to_simplified_path(type_path));
                Some(
                  type_path
                    .path
                    .segments
                    .iter()
                    .map(|segment| segment.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::"),
                )
              }
              _ => unreachable!(),
            },
          })
          .collect(),
      };

      Segment { ident, arguments }
    })
    .collect::<Vec<_>>();

  SimplifiedTypePath { segments }
} */
