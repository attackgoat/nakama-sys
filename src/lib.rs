#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn str_as_nk_string(s: &str) -> NkString {
    NkString {
        p: s.as_ptr() as *const _,
        n: s.len() as _,
    }
}
