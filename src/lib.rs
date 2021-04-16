#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


pub fn str_as_nk_string<S>(s: S) -> NkString
where
    S: AsRef<str>,
{
    let s_ref = s.as_ref();

    NkString {
        p: s_ref.as_ptr() as *const _,
        n: s_ref.len() as _,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
