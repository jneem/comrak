//! This crate provides a C ABI interface for the [comrak](https://crates.io/crate/comrak) crate.

use std::{ptr, slice, str};

macro_rules! make_bool_option_func {
    ($opt_type:ident, $name:ident) => {
        paste! {
            #[no_mangle]
            pub extern "C" fn [<comrak_ set_ $opt_type _ option_ $name>](
                c_comrak_options: *mut ComrakOptions,
                value: bool,
            ) {
                let comrak_options = to_ref_mut!(c_comrak_options);

                comrak_options.$opt_type.$name = value;
            }
        }
    };
}

macro_rules! make_c_char_option_func {
    ($opt_type:ident, $name:ident) => {
        paste! {
            #[no_mangle]
            pub extern "C" fn [<comrak_ set_ $opt_type _ option_ $name>](
                c_comrak_options: *mut ComrakOptions,
                v: *const c_char,
                v_len: size_t,
            ) {
                let comrak_options = to_ref_mut!(c_comrak_options);
                let value = unwrap_or_ret_err_code! { to_str!(v, v_len) };

                comrak_options.$opt_type.$name = Some(value.to_string());
            }
        }
    };
}

macro_rules! make_size_t_option_func {
    ($opt_type:ident, $name:ident) => {
        paste! {
            #[no_mangle]
            pub extern "C" fn [<comrak_ set_ $opt_type _ option_ $name>](
                c_comrak_options: *mut ComrakOptions,
                value: size_t,
            ) {
                let comrak_options = to_ref_mut!(c_comrak_options);

                comrak_options.$opt_type.$name = value;
            }
        }
    };
}

// aborts the thread if we receive NULL where unexpected
macro_rules! assert_not_null {
    ($var:ident) => {
        assert!(!$var.is_null(), "{} is NULL", stringify!($var));
    };
}

macro_rules! to_ref_mut {
    ($ptr:ident) => {{
        assert_not_null!($ptr);
        unsafe { &mut *$ptr }
    }};
}

macro_rules! to_bytes {
    ($data:ident, $len:ident) => {{
        assert_not_null!($data);
        unsafe { slice::from_raw_parts($data as *const u8, $len) }
    }};
}

macro_rules! to_str {
    ($data:ident, $len:ident) => {
        str::from_utf8(to_bytes!($data, $len)).into()
    };
}

macro_rules! unwrap_or_ret {
    ($expr:expr, $ret_val:expr) => {
        match $expr {
            Ok(v) => v,
            Err(_) => {
                return $ret_val;
            }
        }
    };
}

macro_rules! unwrap_or_ret_err_code {
    ($expr:expr) => {
        unwrap_or_ret!($expr, ())
    };
}

#[inline]
fn to_ptr_mut<T>(val: T) -> *mut T {
    Box::into_raw(Box::new(val))
}

mod comrak_extension_options;
pub mod comrak_ffi;

mod comrak_options;
mod comrak_parse_options;
mod comrak_render_options;

mod string;
pub use self::string::Str;
