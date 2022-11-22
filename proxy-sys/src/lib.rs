use syn::__private::{TokenStream, quote::quote};

#[proc_macro_attribute]
pub fn proxy(_attribute: TokenStream, function: TokenStream) -> TokenStream {
    let fn_ts = function.clone();
    let item: syn::Item = syn::parse_macro_input!(fn_ts);
    if let syn::Item::Fn(function) = item {
        let syn::ItemFn {
            attrs,
            block,
            vis,
            sig:
                syn::Signature {
                    ident,
                    unsafety,
                    constness,
                    abi,
                    output,
                    ..
                },
            ..
        } = function;

        let output = quote!(
            #(#attrs)*
            #vis #unsafety #abi #constness fn #ident() #output #block

            //turn the original function into DllMain
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "system" fn DllMain(
                _hinstDLL: proxy_dll::HINSTANCE,
                fdwReason: proxy_dll::DWORD,
                _lpvReserved: proxy_dll::LPVOID,
            ) -> proxy_dll::BOOL {

                if fdwReason == proxy_dll::DLL_PROCESS_ATTACH {
                    //call the original function
                    proxy_dll::exports::initialize(_hinstDLL).unwrap_or_else(|e| {
                        ::std::panic!("{}", e);
                    });
                    #ident();
                }
                proxy_dll::TRUE
            }
            
        );

        return output.into();
    }

    function
}
