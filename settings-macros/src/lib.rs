use std::{env::current_dir, fs::read_to_string};

use derive_syn_parse::Parse;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse2, Error, LitStr, Result, Token};
use toml::{Table, Value};

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
		return Err(Error::new(Span::call_site(), format!(
			"Failed to read '{}'",
			cargo_toml_path.display(),
		)));
	};
    let namespace = args.namespace.value();
    let key = args.key.value();
    let Ok(cargo_toml) = cargo_toml.parse::<Table>() else {
		return Err(Error::new(Span::call_site(), format!(
			"Failed to parse '{}' as valid TOML.",
			cargo_toml_path.display(),
		)));
	};
    let Some(package) = cargo_toml.get("package") else {
		return Err(Error::new(Span::call_site(), format!(
			"Failed to find table 'package' in '{}'.",
			cargo_toml_path.display(),
		)));
	};
    let Some(metadata) = package.get("metadata") else {
		return Err(Error::new(Span::call_site(), format!(
			"Failed to find table 'package.metadata' in '{}'.",
			cargo_toml_path.display(),
		)));
	};
    let Some(settings) = metadata.get("settings") else {
		return Err(Error::new(Span::call_site(), format!(
			"Failed to find table 'package.metadata.settings' in '{}'.",
			cargo_toml_path.display(),
		)));
	};
    let Some(namespace_table) = settings.get(&namespace) else {
		return Err(Error::new(Span::call_site(), format!(
			"Failed to find table 'package.metadata.settings.{}' in '{}'.",
			namespace,
			cargo_toml_path.display(),
		)));
	};
    let Some(value) = namespace_table.get(&key) else {
		return Err(Error::new(Span::call_site(), format!(
			"Failed to find table 'package.metadata.settings.{}.{}' in '{}'.",
			namespace,
			key,
			cargo_toml_path.display(),
		)));
	};
    Ok(match value {
        Value::String(string) => quote!(#string),
        Value::Integer(integer) => quote!(#integer),
        Value::Float(float) => quote!(#float),
        Value::Boolean(bool) => quote!(#bool),
        Value::Datetime(datetime) => {
            let datetime = datetime.to_string();
            quote!(#datetime)
        }
        Value::Array(arr) => {
            let arr = arr.as_slice().iter().map(|item| item.to_string());
            quote!([#(#arr),*])
        }
        Value::Table(_) => {
            return Err(Error::new(
                Span::call_site(),
                "Tables within a namespace are not supported at this time",
            ));
        }
    })
}
