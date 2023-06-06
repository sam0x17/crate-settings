use std::{env::current_dir, fs::read_to_string};

use derive_syn_parse::Parse;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse2, Error, LitStr, Result, Token};

#[proc_macro]
pub fn settings(tokens: TokenStream) -> TokenStream {
    match settings_internal(tokens) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[derive(Parse)]
struct SettingsProcArgs {
    namespace: LitStr,
    #[prefix(Token![,])]
    key: LitStr,
}

fn settings_internal(tokens: impl Into<TokenStream2>) -> Result<TokenStream2> {
    let args = parse2::<SettingsProcArgs>(tokens.into())?;
    let Ok(current_dir) = current_dir() else {
		return Err(Error::new(Span::call_site(), "Failed to read current directory."));
	};
    let cargo_toml_path = current_dir.join("Cargo.toml");
    let Ok(cargo_toml) = read_to_string(&cargo_toml_path) else {
		return Err(Error::new(Span::call_site(), format!("Failed to read '{}'", cargo_toml_path.display())));
	};
    let namespace = args.namespace.value();
    let key = args.key.value();

    Ok(quote!())
}
