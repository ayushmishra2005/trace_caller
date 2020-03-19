/**
 * Copyright (c) 2020 Ayush Kumar Mishra
 *
 * This source code is licensed under the MIT License found in
 * the LICENSE file in the root directory of this source tree.
 */
extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};

/// This attribute will enable a function to access the caller's source location.
#[proc_macro_attribute]
pub fn trace_caller(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut prefix: TokenStream = "
    use backtrace::{Backtrace, BacktraceFrame};
    let (trace, curr_file, curr_line) = (Backtrace::new(), file!(), line!());
    let frames = trace.frames();
    let previous_symbol = frames
        .iter()
        .flat_map(BacktraceFrame::symbols)
        .skip_while(|s| {
            s.filename()
                .map(|p| !p.ends_with(curr_file))
                .unwrap_or(true)
                || s.lineno() != Some(curr_line)
        })
        .nth(1 as usize)
        .unwrap();

        println!(\"Called from {:?} at line {:?}\",
            previous_symbol.filename().unwrap(), previous_symbol.lineno().unwrap());
    "
    .parse()
    .unwrap();

    item.into_iter()
        .map(|tt| match tt {
            TokenTree::Group(ref g) if g.delimiter() == proc_macro::Delimiter::Brace => {
                prefix.extend(g.stream());

                TokenTree::Group(proc_macro::Group::new(
                    proc_macro::Delimiter::Brace,
                    prefix.clone(),
                ))
            }
            other => other,
        })
        .collect()
}
