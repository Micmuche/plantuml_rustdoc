use proc_macro::TokenStream;
use quote::quote;
use std::io::Write;
use std::process::{Command, Stdio};
use syn::{LitStr, parse_macro_input};

#[proc_macro]
pub fn plantuml_svg(input: TokenStream) -> TokenStream {
    let src = parse_macro_input!(input as LitStr).value();

    let mut child = Command::new("plantuml")
        .args(["-tsvg", "-pipe"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Unable to launch plantuml command, is plantuml installed? (cf README)");

    child
        .stdin
        .take()
        .unwrap()
        .write_all(src.as_bytes())
        .unwrap();

    let out = child.wait_with_output().unwrap();
    assert!(out.status.success(), "plantuml failed");
    let svg = String::from_utf8(out.stdout).unwrap();

    quote!(#svg).into()
}
