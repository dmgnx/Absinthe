pub mod prelude {
    pub use proc_macro::TokenStream;

    pub use proc_macro2::{
        TokenStream as TokenStream2,
        Span,
    };

    pub use quote::{
        quote, 
        format_ident, 
        ToTokens,
        spanned::Spanned,
    };
    
    pub use syn::{
        parenthesized,
        parse_macro_input, 
        Token,
        AngleBracketedGenericArguments,
        Expr,
        FnArg,
        Generics,
        GenericParam,
        Ident,
        Item,
        ItemEnum,
        ItemFn,
        ItemImpl,
        ItemStruct,
        ReturnType,
        parse::{
            Parse, 
            ParseStream
        },
        punctuated::Punctuated,
    };

    pub use crate::msg::*;
}