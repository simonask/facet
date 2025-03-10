// use proc_macro::TokenStream;
use unsynn::*;

keyword! {
    Struct = "struct";
}

unsynn! {
    enum Enum {
        Two(Plus, Plus, Dot),
        One(Plus, Dot),
        TwoS { a: Minus, b: Minus, c: Dot},
        OneS { a: Minus, b: Dot },
        // the Expect<Dollar> shows a rust-analyzer error here, which is probably a bug in r-a
        PunctBreak(Punct, Expect<Dollar>),
    }

    struct StructLike {
        _kw_struct: Struct,
        name: Ident,
        body: GroupContaining<Vec<FieldLike>>,
    }

    struct FieldLike {
        name: Ident,
        _colon: Colon,
        typ: Ident,
        _comma: Option<Comma>,
    }
}

#[proc_macro]
pub fn parse_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let mut i = input.to_token_iter();
    let parsed: Enum = i.parse().unwrap();
    let dbg_s = format!("{parsed:#?}");
    let s = format!("println!(\"Parsed: {{}}\", {dbg_s:?});");
    s.into_token_stream().into()
}

#[proc_macro]
pub fn parse_struct_like(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let mut i = input.to_token_iter();
    let parsed: StructLike = i.parse().unwrap();
    let dbg_s = format!("{parsed:#?}");
    let s = format!("println!(\"Parsed: {{}}\", {dbg_s:?});");
    s.into_token_stream().into()
}

#[proc_macro_derive(Shapely)]
pub fn shapely_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let mut i = input.to_token_iter();
    let parsed: StructLike = i.parse().unwrap();
    let dbg_s = format!("{parsed:#?}");

    let struct_def = format!(
        "impl {name} {{
            fn get_parsed_structure() -> &'static str {{
                {dbg_s:?}
            }}
        }}",
        // FIXME: interpolation is no good here, does unsynn provide a good way to do this?
        name = parsed.name
    );
    struct_def.into_token_stream().into()
}
