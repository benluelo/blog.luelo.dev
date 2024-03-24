#![feature(
    adt_const_params,
    generic_const_exprs,
    const_intrinsic_copy,
    const_mut_refs,
    inline_const,
    trace_macros
)]
#![allow(incomplete_features)]

use core::ascii;

use macros::apply;

static BASE: &str = concat::<
    "<!DOCTYPE html>",
    {
        html! {
            #[lang="en"]
            'html: {
                'head: {
                    'title: {
                        "oogabooga";
                    }
                    #[rel = "stylesheet"]
                    #[href = "https://cdn.jsdelivr.net/npm/purecss@3.0.0/build/base-min.css"]
                    #[r#type = "text/css"]
                    'link: {};
                 }
                'body: {
                    INDEX;
                }
            }
        }
    },
>();

#[apply(html)]
const INDEX: &str = {
    // #[a = ""]
    // #[b = ""]
    'div: {
        'div: {
            "Hello";
            ", World";
        }
        'div: {
            #[data_a = ""]
            #[data_b = ""]
            'div: {
                "this is incredibly cursed lmao";
                'br: {};
                itoa::<{ 1 + 2 }>();
                'br: {};
                'p: {
                    'span: {
                        "hi";
                    }
                }
                'pre: {
                    "look ma, code!";
                }
                'hr: {};
            }
        }
    }
};

fn main() {
    println!("{BASE}");
}

#[macro_export]
macro_rules! html {
    (
        const $CONST:ident: $ty:ty = { $($tt:tt)* };
    ) => {
        const $CONST: $ty = html!($($tt)*);
    };
    (
        $(#[$attr:ident=$value:literal])*
        $elem:lifetime: {};
        $($rem:tt)*
    ) => {
         concat::<
            {
                concat::<
                    "<",
                    {
                        concat::<
                            { utf8(stringify!($elem).as_bytes().split_at(1).1) },
                            {
                                concat::<
                                    { attrs!($($attr=$value)*) },
                                    ">",
                                >()
                            },
                        >()
                    },
                >()
            },
            { html!($($rem)*) },
        >()
    };
    (
        $(#[$attr:ident=$value:literal])*
        $elem:lifetime: { $($tt:tt)* }
        $($rem:tt)*
    ) => {
         concat::<
            {
                concat::<
                    "<",
                    {
                        concat::<
                            { utf8(stringify!($elem).as_bytes().split_at(1).1) },
                            {
                                concat::<
                                    { attrs!($($attr=$value)*) },
                                    {
                                        concat::<
                                            ">",
                                            {
                                                concat::<
                                                    { html!($($tt)*) },
                                                    {
                                                        concat::<
                                                            "</",
                                                            {
                                                                concat::<
                                                                    { utf8(stringify!($elem).as_bytes().split_at(1).1) },
                                                                    ">"
                                                                >()
                                                            },
                                                        >()
                                                    },
                                                >()
                                            },
                                        >()
                                    },
                                >()
                            },
                        >()
                    },
                >()
            },
            { html!($($rem)*) },
        >()
    };
    ($lit:expr; $($rem:tt)*) => {
        concat::<{ concat::<{ $lit }, "\n">() }, { html!($($rem)*) }>()
    };
    () => {
        ""
    };
}

#[macro_export]
macro_rules! attrs {
    (
        $attr:ident=$value:literal $($attrs:ident=$values:literal)*
    ) => {
        concat::<" ", { concat::<{ strip_prefix(replace::<{ stringify!($attr) }, '_', '-'>(), "r#") }, { concat::<"=", { concat::<{ concat::<"\"", $value>() }, { concat::<"\"", { attrs!($($attrs=$values)*) }>() }>() }>() }>() }>()
    };
    () => {
        ""
    };
}

const fn concat<const A: &'static str, const B: &'static str>() -> &'static str
where
    [(); A.len() + B.len()]:,
{
    const fn do_concat<const A: &'static str, const B: &'static str>() -> [u8; A.len() + B.len()]
    where
        [(); A.len() + B.len()]:,
    {
        let mut buffer = [0u8; { A.len() + B.len() }];

        let a = A.as_bytes();
        let b = B.as_bytes();

        let mut i = 0;

        while i < A.len() {
            buffer[i] = a[i];
            i += 1;
        }

        while i < buffer.len() {
            buffer[i] = b[i - A.len()];
            i += 1;
        }

        buffer
    }

    utf8(&const { do_concat::<A, B>() })
}

const fn utf8(bz: &[u8]) -> &str {
    match core::str::from_utf8(bz) {
        core::result::Result::Ok(ok) => ok,
        core::result::Result::Err(_) => {
            core::panic!()
        }
    }
}

const fn itoa<const N: u128>() -> &'static str
where
    [(); (N.ilog10() + 1) as usize]:,
{
    const fn do_itoa<const N: u128>() -> [u8; (N.ilog10() + 1) as usize] {
        let mut n = N;

        let mut arr = [0u8; { (N.ilog10() + 1) as usize }];

        if n == 0 {
            arr[0] = b'0';
        } else {
            let i = 0;
            while n != 0 {
                let digit = n % 10;

                assert!(digit <= 9);

                arr[i] = digit as u8 + 48;

                n /= 10;
            }
        }

        arr
    }

    utf8(&const { do_itoa::<N>() })
}

const fn replace<const STR: &'static str, const FROM: char, const TO: char>() -> &'static str
where
    [(); STR.len()]:,
{
    const fn do_replace<const STR: &'static str, const FROM: char, const TO: char>(
    ) -> [u8; STR.len()] {
        assert!(FROM.is_ascii());
        assert!(TO.is_ascii());

        let mut s = [0; STR.len()];

        let mut i = 0;

        while i < STR.len() {
            s[i] = if STR.as_bytes()[i] == FROM as u8 {
                TO as u8
            } else {
                STR.as_bytes()[i]
            };
            i += 1;
        }

        s
    }

    utf8(&const { do_replace::<STR, FROM, TO>() })
}

const fn strip_prefix(s: &'static str, prefix: &'static str) -> &'static str {
    if s.len() < prefix.len() {
        s
    } else if slice_eq(slice(s.as_bytes(), 0, prefix.len()), prefix.as_bytes()) {
        utf8(slice(s.as_bytes(), prefix.len(), s.len()))
    } else {
        s
    }
}

pub const fn slice<T>(bytes: &[T], idx_start: usize, idx_curr: usize) -> &[T] {
    let first_split = &bytes.split_at(idx_start).1;
    let line = first_split.split_at(idx_curr - idx_start).0;
    line
}

pub const fn slice_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() == b.len() {
        let mut i = 0;
        while i < a.len() {
            if a[i] != b[i] {
                return false;
            }
            i += 1;
        }

        true
    } else {
        false
    }
}
