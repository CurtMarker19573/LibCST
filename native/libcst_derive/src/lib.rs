// Copyright (c) Meta Platforms, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree

mod inflate;
use inflate::impl_inflate;
mod parenthesized_node;
use parenthesized_node::impl_parenthesized_node;
mod codegen;
use codegen::impl_codegen;
mod into_py;
use into_py::impl_into_py;
mod cstnode;
use cstnode::impl_cst_node;

use proc_macro::TokenStream;
use syn::{parse, parse_macro_input, DeriveInput};

#[proc_macro_derive(Inflate)]
pub fn inflate_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_inflate(&ast)
}

#[proc_macro_derive(ParenthesizedNode)]
pub fn parenthesized_node_derive(input: TokenStream) -> TokenStream {
    impl_parenthesized_node(&syn::parse(input).unwrap())
}

#[proc_macro_derive(Codegen)]
pub fn parenthesized_node_codegen(input: TokenStream) -> TokenStream {
    impl_codegen(&syn::parse(input).unwrap())
}

#[proc_macro_derive(IntoPy, attributes(skip_py, no_py_default))]
pub fn into_py(input: TokenStream) -> TokenStream {
    impl_into_py(&syn::parse(input).unwrap())
}

#[proc_macro_attribute]
pub fn cst_node(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(args as parse::Nothing);
    impl_cst_node(parse_macro_input!(input as DeriveInput))
}
