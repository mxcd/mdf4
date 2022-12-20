extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use serde::Deserialize;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use syn;
use syn::parse_macro_input;

fn __resolve_file<P: AsRef<Path>>(relative_file: P) -> PathBuf {
    let project_root = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is unset");
    let mut path = PathBuf::from(project_root);
    path.push(relative_file);
    path
}

#[derive(Deserialize, Debug)]
struct BlockSpecificationFile {
    blocks: Vec<BlockSpecification>,
}

#[derive(Deserialize, Debug)]
struct BlockSpecification {
    name: String,
    length: u64,
    identifier: String,
    fields: Vec<BlockFieldSpecification>,
}

#[derive(Deserialize, Debug)]
struct BlockFieldSpecification {
    name: String,
    field_type: String,
    length: u32,
    comment: Option<String>,
}

fn read_block_specification_file(file_name: &str) -> BlockSpecificationFile {
    println!("{}", __resolve_file(&file_name).display());
    let f = std::fs::File::open(__resolve_file(&file_name)).expect("Could not open file.");
    let block_specifications: BlockSpecificationFile =
        serde_yaml::from_reader(f).expect("Could not read values.");
    return block_specifications;
}

#[proc_macro]
pub fn block_definition(input: TokenStream) -> TokenStream {
    let file_name = parse_macro_input!(input as syn::LitStr);
    let resolved_file_name = __resolve_file(&file_name.value());
    let block_specifications = read_block_specification_file(&resolved_file_name.to_str().unwrap());

    // generate struct from block specification
    let block = &block_specifications.blocks[0];
    let block_name = syn::Ident::new(&block.name, syn::__private::Span::call_site());
    let mut block_fields = Vec::new();
    for field in &block.fields {
        let field_name = syn::Ident::new(&field.name, syn::__private::Span::call_site());
        let field_type = syn::Ident::new(&field.field_type, syn::__private::Span::call_site());
        let field = quote! {
          pub #field_name: #field_type
        };
        block_fields.push(field);
    }
    let block_definition = quote! {
      #[derive(Debug)]
      pub struct #block_name {
          #(#block_fields),*
      }
    };
    block_definition.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_block_specification_file() {
        let block_specifications = read_block_specification_file(
            "/Users/mapa/github.com/mxcd/mdf4/blockspecs/id.blockspec.yml",
        );
        assert_eq!(block_specifications.blocks.len(), 1);
        let block = &block_specifications.blocks[0];
        assert_eq!(block.name, "IDBlock");
        assert_eq!(block.identifier, "");
        let field = &block.fields[0];
        assert_eq!(field.name, "fileIdentifier");
        assert_eq!(field.field_type, "String");
        assert_eq!(field.length, 8);
        assert!(field.comment.is_some());
    }
}
