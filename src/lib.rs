//! Low-level bindings for the [Bitwuzla] SMT solver.
//!
//! Please see the Bitwuzla [C API documentation] for function descriptions.
//!
//! [Bitwuzla]: https://bitwuzla.github.io/
//! [C API documentation]: https://bitwuzla.github.io/docs/c/api.html

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

include!("../src-generated/bindings.rs");

#[cfg(test)]
mod tests {
    #[test]
    fn smoke_test() {
        use std::ffi::CStr;
        use crate::*;

        unsafe {
            let term_manager = bitwuzla_term_manager_new();

            let s = bitwuzla_mk_bv_sort(term_manager, 16);
            let a = bitwuzla_mk_const(term_manager, s, b"a\0" as *const _ as _);
            let b = bitwuzla_mk_const(term_manager, s, b"b\0" as *const _ as _);
            let c = bitwuzla_mk_const(term_manager, s, b"c\0" as *const _ as _);

            let a_lt_b = bitwuzla_mk_term2(term_manager, BITWUZLA_KIND_BV_SLT, a, b);
            let a_plus_c = bitwuzla_mk_term2(term_manager, BITWUZLA_KIND_BV_ADD, a, c);
            let b_plus_c = bitwuzla_mk_term2(term_manager, BITWUZLA_KIND_BV_ADD, b, c);
            let a_plus_c_gt_b_plus_c = bitwuzla_mk_term2(term_manager, BITWUZLA_KIND_BV_SGT, a_plus_c, b_plus_c);

            let options = bitwuzla_options_new();
            bitwuzla_set_option(options, BITWUZLA_OPT_PRODUCE_MODELS, 1);

            let bzla = bitwuzla_new(term_manager, options);
            bitwuzla_assert(bzla, a_lt_b);
            bitwuzla_assert(bzla, a_plus_c_gt_b_plus_c);

            assert_eq!(bitwuzla_check_sat(bzla), BITWUZLA_SAT);

            let a = CStr::from_ptr(bitwuzla_term_value_get_str(bitwuzla_get_value(bzla, a))).to_str().unwrap().to_string();
            let b = CStr::from_ptr(bitwuzla_term_value_get_str(bitwuzla_get_value(bzla, b))).to_str().unwrap().to_string();
            let c = CStr::from_ptr(bitwuzla_term_value_get_str(bitwuzla_get_value(bzla, c))).to_str().unwrap().to_string();

            let a = u16::from_str_radix(&a, 2).unwrap() as i16;
            let b = u16::from_str_radix(&b, 2).unwrap() as i16;
            let c = u16::from_str_radix(&c, 2).unwrap() as i16;

            assert!(a.checked_add(c).is_none() || b.checked_add(c).is_none());

            bitwuzla_delete(bzla);
            bitwuzla_options_delete(options);
            bitwuzla_term_manager_delete(term_manager);
        }
    }
}
