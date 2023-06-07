use std::{env::current_dir, fs::read_to_string, path::PathBuf};

use derive_syn_parse::Parse;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse2, Error, LitStr, Result, Token};
use toml::{Table, Value};
use walkdir::WalkDir;

#[proc_macro]
pub fn settings(tokens: TokenStream) -> TokenStream {
    match settings_internal(tokens) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[derive(Parse)]
struct SettingsProcArgs {
    crate_name: LitStr,
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

/// Finds the root of the current workspace, falling back to the outer-most directory with a
/// Cargo.toml, and then falling back to the current directory.
fn workspace_root() -> PathBuf {
    let mut current_dir = current_dir().expect("failed to unwrap env::current_dir()!");
    let mut best_match = current_dir.clone();
    loop {
        let cargo_toml = current_dir.join("Cargo.toml");
        if let Ok(cargo_toml) = read_to_string(&cargo_toml) {
            best_match = current_dir.clone();
            if cargo_toml.contains("[workspace]") {
                return best_match;
            }
        }
        match current_dir.parent() {
            Some(dir) => current_dir = dir.to_path_buf(),
            None => break,
        }
    }
    best_match
}

fn crate_root<S: AsRef<str>>(crate_name: S, current_dir: &PathBuf) -> PathBuf {
    let root = workspace_root();
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let Some(file_name) = path.file_name() else { continue };
        if file_name != "Cargo.toml" {
            continue;
        }
        let Ok(cargo_toml) = read_to_string(path) else { continue };
        let Ok(cargo_toml) = cargo_toml.parse::<Table>() else { continue };
        let Some(package) = cargo_toml.get("package") else { continue };
        let Some(name) = package.get("name") else { continue };
        let Value::String(name) = name else { continue };
        if name == crate_name.as_ref() {
            println!("found it: {}", path.parent().unwrap().display());
            return path.parent().unwrap().to_path_buf();
        }
    }
    current_dir.clone()
}

fn settings_internal_helper(
    crate_name: String,
    key: String,
    current_dir: PathBuf,
) -> Result<TokenStream2> {
    println!(
        "settings_internal_helper({}, {}, {})",
        crate_name,
        key,
        current_dir.display()
    );
    let parent_dir = match current_dir.parent() {
        Some(parent_dir) => {
            let parent_toml = parent_dir.join("Cargo.toml");
            match parent_toml.exists() {
                true => Some(parent_dir.to_path_buf()),
                false => None,
            }
        }
        None => None,
    };
    let cargo_toml_path = current_dir.join("Cargo.toml");
    let Ok(cargo_toml) = read_to_string(&cargo_toml_path) else {
		if let Some(parent_dir) = parent_dir {
			return settings_internal_helper(crate_name, key, parent_dir);
		}
		return Err(Error::new(Span::call_site(), format!(
			"Failed to read '{}'",
			cargo_toml_path.display(),
		)));
	};
    let Ok(cargo_toml) = cargo_toml.parse::<Table>() else {
		if let Some(parent_dir) = parent_dir {
			return settings_internal_helper(crate_name, key, parent_dir);
		}
		return Err(Error::new(Span::call_site(), format!(
			"Failed to parse '{}' as valid TOML.",
			cargo_toml_path.display(),
		)));
	};
    let Some(package) = cargo_toml.get("package") else {
		if let Some(parent_dir) = parent_dir {
			return settings_internal_helper(crate_name, key, parent_dir);
		}
		return Err(Error::new(Span::call_site(), format!(
			"Failed to find table 'package' in '{}'.",
			cargo_toml_path.display(),
		)));
	};
    let Some(metadata) = package.get("metadata") else {
		if let Some(parent_dir) = parent_dir {
			return settings_internal_helper(crate_name, key, parent_dir);
		}
		return Err(Error::new(Span::call_site(), format!(
			"Failed to find table 'package.metadata' in '{}'.",
			cargo_toml_path.display(),
		)));
	};
    let Some(settings) = metadata.get("settings") else {
		if let Some(parent_dir) = parent_dir {
			return settings_internal_helper(crate_name, key, parent_dir);
		}
		return Err(Error::new(Span::call_site(), format!(
			"Failed to find table 'package.metadata.settings' in '{}'.",
			cargo_toml_path.display(),
		)));
	};
    let Some(crate_name_table) = settings.get(&crate_name) else {
		if let Some(parent_dir) = parent_dir {
			return settings_internal_helper(crate_name, key, parent_dir);
		}
		return Err(Error::new(Span::call_site(), format!(
			"Failed to find table 'package.metadata.settings.{}' in '{}'.",
			crate_name,
			cargo_toml_path.display(),
		)));
	};
    let Some(value) = crate_name_table.get(&key) else {
		if let Some(parent_dir) = parent_dir {
			return settings_internal_helper(crate_name, key, parent_dir);
		}
		return Err(Error::new(Span::call_site(), format!(
			"Failed to find table 'package.metadata.settings.{}.{}' in '{}'.",
			crate_name,
			key,
			cargo_toml_path.display(),
		)));
	};
    emit_toml_value(value.clone())
}

fn settings_internal(tokens: impl Into<TokenStream2>) -> Result<TokenStream2> {
    let args = parse2::<SettingsProcArgs>(tokens.into())?;
    let Ok(current_dir) = current_dir() else {
		return Err(Error::new(Span::call_site(), "Failed to read current directory."));
	};
    let starting_dir = crate_root(args.crate_name.value(), &current_dir);
    settings_internal_helper(args.crate_name.value(), args.key.value(), starting_dir)
}
