use syn::{parse_macro_input, Block};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};

///
/// Wraps one or more gl::xxxx calls in both unsafe and OpenGl error handling code
///
/// # External Use
/// If you are going to be using this macro outside of [jeremyirvine/glfw-rust-app](https://github.com/jeremyirvine/glfw-rust-app), 
/// you will need to supply 2 `crate::` level exports
/// ```rust 
/// pub fn gl_clear_errors()
/// ```
///
/// and
///
/// ```rust
/// pub fn gl_log_errors(file: impl Display, line: impl Display, statement: String) -> bool {
///     //...impl
/// }
/// ```
///
/// # Usage
/// ```rust
/// gl_call!({ /* insert gl call here */ });
/// ```
///
/// ##### Examples
/// ```rust
/// // Single line
/// gl_call!({ gl::GenBuffers(1, &mut vao); });
///
/// // Multi line
/// gl_call!({ 
///     gl::GenBuffers(1, &mut vbo); });
///     gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
/// });
/// ```
///
#[proc_macro]
pub fn gl_call(body: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(body as Block);
    let stmts = parsed.stmts;

    stmts.iter().map(|stmt| -> TokenStream {
        let stmt_str = stmt.to_token_stream().to_string();
        TokenStream::from(quote!(
            crate::gl_clear_errors();
            unsafe { #stmt }
            crate::gl_log_errors(file!(), line!(), #stmt_str.into());
        ))
    }).collect()
}
