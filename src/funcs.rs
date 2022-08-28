use syn::ItemFn;

pub fn get_fn_arguments(func: &ItemFn) {
  // async fn json(Json(payload): Json<serde_json::Value>) {}
  dbg!(&func);

  let x = func
    .sig
    .inputs
    .iter()
    .map(|input| match input {
      syn::FnArg::Receiver(_) => todo!(),
      syn::FnArg::Typed(pat_type) => match &*pat_type.pat {
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
          let segments = tuple_struct
            .path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()
            .join("::");

          let patterns = tuple_struct
            .pat
            .elems
            .iter()
            .map(|elem| match elem {
              syn::Pat::Box(_) => todo!(),
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
              syn::Pat::TupleStruct(_) => todo!(),
              syn::Pat::Type(_) => todo!(),
              syn::Pat::Verbatim(_) => todo!(),
              syn::Pat::Wild(_) => todo!(),
              syn::Pat::Ident(ident) => ident.ident.to_string(),
              _ => todo!(),
            })
            .collect::<Vec<_>>()
            .join(", ");
        }
        _ => todo!(),
      },
    })
    .collect::<Vec<_>>();
}
