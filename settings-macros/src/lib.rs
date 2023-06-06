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

#[derive(PartialEq, Copy, Clone)]
enum ValueType {
    String,
    Integer,
    Float,
    Boolean,
    Datetime,
    Array,
    Table,
}

trait GetValueType {
    fn value_type(&self) -> ValueType;
}

impl GetValueType for Value {
    fn value_type(&self) -> ValueType {
        use ValueType::*;
        match self {
            Value::String(_) => String,
            Value::Integer(_) => Integer,
            Value::Float(_) => Float,
            Value::Boolean(_) => Boolean,
            Value::Datetime(_) => Datetime,
            Value::Array(_) => Array,
            Value::Table(_) => Table,
        }
    }
}

fn emit_toml_value(value: Value) -> Result<TokenStream2> {
    match value {
        Value::String(string) => Ok(quote!(#string)),
        Value::Integer(integer) => Ok(quote!(#integer)),
        Value::Float(float) => Ok(quote!(#float)),
        Value::Boolean(bool) => Ok(quote!(#bool)),
        Value::Datetime(date_time) => {
            let date_time = date_time.to_string();
            Ok(quote!(#date_time))
        }
        Value::Array(arr) => {
            let mut new_arr: Vec<TokenStream2> = Vec::new();
            let mut current_type: Option<ValueType> = None;
            for value in arr.iter() {
                if let Some(typ) = current_type {
                    if typ != value.value_type() {
                        let arr = arr.iter().map(|item| match item.as_str() {
                            Some(st) => String::from(st),
                            None => item.to_string(),
                        });
                        return Ok(quote!([#(#arr),*]));
                    }
                } else {
                    current_type = Some(value.value_type());
                }
                new_arr.push(emit_toml_value(value.clone())?)
            }
            Ok(quote!([#(#new_arr),*]))
        }
        Value::Table(table) => {
            let st = format!("{{ {} }}", table.to_string().trim().replace("\n", ", "));
            Ok(quote!(#st))
        }
    }
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
    emit_toml_value(value.clone())
}
