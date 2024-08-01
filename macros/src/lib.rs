use proc_macro::TokenStream;

#[proc_macro]
pub fn generate_loading_env_code(_input: TokenStream) -> TokenStream {
    //let input: proc_macro2::TokenStream = input.into();

    quote::quote! {

        let resource = use_resource(|| get_envs());

        let data = resource.read_unchecked();

        match &*data {
            Some(data) => match data {
                Ok(result) => {
                    consume_context::<Signal<EnvListState>>()
                        .write()
                        .set_items(result.clone());
                    return rsx! {
                        ActiveApp {}
                    };
                }
                Err(err) => {
                    let err = format!("Error loading environments. Err: {}", err);
                    return rsx! {
                        {err}
                    };
                }
            },

            None => {
                return rsx! { "Loading environments..." };
            }
        }

    }
    .into()
}
