#![warn(
    clippy::as_underscore,
    clippy::bool_to_int_with_if,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::cast_lossless,
    clippy::cast_possible_wrap,
    clippy::checked_conversions,
    clippy::clone_on_ref_ptr,
    clippy::cloned_instead_of_copied,
    clippy::copy_iterator,
    clippy::create_dir,
    clippy::default_trait_access,
    clippy::deref_by_slicing,
    clippy::doc_link_with_quotes,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::empty_line_after_outer_attr,
    clippy::empty_structs_with_brackets,
    clippy::enum_glob_use,
    clippy::equatable_if_let,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::explicit_iter_loop,
    clippy::filetype_is_file,
    clippy::filter_map_next,
    clippy::flat_map_option,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::fn_params_excessive_bools,
    clippy::fn_to_numeric_cast_any,
    clippy::from_iter_instead_of_collect,
    clippy::future_not_send,
    clippy::get_unwrap,
    clippy::if_not_else,
    clippy::if_then_some_else_none,
    clippy::implicit_hasher,
    clippy::imprecise_flops,
    clippy::inconsistent_struct_constructor,
    clippy::index_refutable_slice,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::items_after_statements,
    clippy::iter_not_returning_iterator,
    clippy::iter_on_empty_collections,
    clippy::iter_on_single_items,
    clippy::iter_with_drain,
    clippy::large_digit_groups,
    clippy::large_stack_arrays,
    clippy::large_types_passed_by_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::manual_assert,
    clippy::manual_instant_elapsed,
    clippy::manual_let_else,
    clippy::manual_ok_or,
    clippy::manual_string_new,
    clippy::many_single_char_names,
    clippy::map_err_ignore,
    clippy::map_unwrap_or,
    clippy::match_on_vec_items,
    clippy::mismatching_type_param_order,
    clippy::missing_const_for_fn,
    clippy::missing_enforced_import_renames,
    clippy::must_use_candidate,
    clippy::mut_mut,
    clippy::naive_bytecount,
    clippy::needless_bitwise_bool,
    clippy::needless_collect,
    clippy::needless_continue,
    clippy::needless_for_each,
    clippy::needless_pass_by_value,
    clippy::negative_feature_names,
    clippy::non_ascii_literal,
    clippy::non_send_fields_in_send_ty,
    clippy::or_fun_call,
    clippy::range_minus_one,
    clippy::range_plus_one,
    clippy::rc_buffer,
    clippy::redundant_closure_for_method_calls,
    clippy::redundant_else,
    clippy::redundant_feature_names,
    clippy::redundant_pub_crate,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::return_self_not_must_use,
    clippy::same_functions_in_if_condition,
    clippy::semicolon_if_nothing_returned,
    clippy::separated_literal_suffix,
    clippy::significant_drop_in_scrutinee,
    clippy::single_match_else,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_add_assign,
    clippy::string_slice,
    clippy::struct_excessive_bools,
    clippy::suboptimal_flops,
    clippy::suspicious_operation_groupings,
    clippy::suspicious_xor_used_as_pow,
    clippy::trailing_empty_array,
    clippy::trait_duplication_in_bounds,
    clippy::transmute_ptr_to_ptr,
    clippy::transmute_undefined_repr,
    clippy::trivial_regex,
    clippy::trivially_copy_pass_by_ref,
    clippy::try_err,
    clippy::type_repetition_in_bounds,
    clippy::undocumented_unsafe_blocks,
    clippy::unicode_not_nfc,
    clippy::uninlined_format_args,
    clippy::unnecessary_join,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unnested_or_patterns,
    clippy::unreadable_literal,
    clippy::unsafe_derive_deserialize,
    clippy::unused_async,
    clippy::unused_peekable,
    clippy::unused_rounding,
    clippy::unused_self,
    clippy::unwrap_in_result,
    clippy::use_self,
    clippy::useless_let_if_seq,
    clippy::verbose_bit_mask,
    clippy::verbose_file_reads
)]
#![deny(
    clippy::derive_partial_eq_without_eq,
    clippy::match_bool,
    clippy::mem_forget,
    clippy::mutex_atomic,
    clippy::mutex_integer,
    clippy::nonstandard_macro_braces,
    clippy::path_buf_push_overwrite,
    clippy::rc_mutex,
    clippy::wildcard_dependencies
)]

mod airport_names;
mod cmds;

use std::path::Path;

use bunt::println;
use color_eyre::eyre::{eyre, Result};
use common::data_types::{timetable::AirlineTimetable, RAW_DATA};
use itertools::Itertools;
use native_dialog::FileDialog;
use rustyline::{error::ReadlineError, history::FileHistory, Editor};

use crate::cmds::{
    c::c, d::d, e::e, h::h, i::i, ie::ie, is::is, m::m, n::n, q::q, sa::sa, sae::sae, sas::sas,
    sd::sd, Action,
};

macro_rules! cprintln {
    (red $($f:tt)+) => {
        println!("{$red+bold}{}{/$}", format!($($f)+))
    };
    (yellow $($f:tt)+) => {
        println!("{$yellow+bold}{}{/$}", format!($($f)+))
    }
}

fn main() -> Result<()> {
    let mut rl = Editor::<(), FileHistory>::new()?;
    cprintln!(yellow "MRT FlightRadar Timetable Editor");
    let (mut file, path) = loop {
        println!("Select file...");
        let dialog = FileDialog::new()
            .add_filter("MRT FlightRadar timetable file", &["fpln"])
            .show_open_single_file()?;
        let Some(file) = dialog else {
            cprintln!(yellow "Quitting");
            return Ok(());
        };
        break (
            match AirlineTimetable::from_file(file.to_owned()) {
                Ok(at) => at,
                Err(err) => {
                    cprintln!(red "Error reading file: {err}");
                    continue;
                }
            },
            file.parent().map_or(file.to_owned(), Path::to_path_buf),
        );
    };

    let air_facilities = &RAW_DATA.air_facilities;
    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("Editing {[yellow]}\nEnter {$cyan}h{/$} for help", file.name);
        cprintln!(yellow "#\t(a) Aircraft\t(reg) Registry\t(f1) Flight 1\t(a1) Airport 1\t(d1) Dep. 1\t(f2) Flight 2\t\t(a2) Airport 2\t(d2) Dep. 2\tetc...");
        println!(
            "{}",
            file.flights
                .iter()
                .enumerate()
                .map(|(i, f)| format!(
                    "{}\t{}\t\t{}\t\t{}",
                    i,
                    f.aircraft,
                    f.registry,
                    f.segments
                        .iter()
                        .map(|seg| format!(
                            "{}\t\t{}\t\t{}",
                            seg.flight_no, seg.airport, seg.depart_time
                        ))
                        .join("\t\t")
                ))
                .join("\n")
        );
        match rl.readline("> ") {
            Ok(cmd_str) => {
                let mut cmd_str = cmd_str.split(' ').peekable();

                let action = match cmd_str.next() {
                    Some("q") => q(),
                    Some("h") => h(),
                    Some("i") => i(&mut cmd_str, &mut file, air_facilities),
                    Some("is") => is(&mut cmd_str, &mut file, air_facilities),
                    Some("ie") => ie(&mut cmd_str, &mut file, air_facilities),
                    Some("c") => c(&mut cmd_str, &mut file),
                    Some("d") => d(&mut cmd_str, &mut file),
                    Some("m") => m(&mut cmd_str, &mut file),
                    Some("e") => e(&mut cmd_str, air_facilities),
                    Some("n") => n(&mut cmd_str),
                    Some("sa") => sa(&mut cmd_str, &mut file, air_facilities),
                    Some("sae") => sae(&mut cmd_str, &mut file, air_facilities),
                    Some("sas") => sas(&mut cmd_str, &mut file, air_facilities),
                    Some("sd") => sd(&mut cmd_str, &mut file),
                    Some(a) => Err(eyre!("Unknown command `{a}`")),
                    None => Ok(Action::Refresh),
                };
                match action {
                    Ok(Action::Refresh) => {}
                    Ok(Action::Hold) => {
                        let _ = rl.readline("Press enter to continue...");
                    }
                    Ok(Action::Msg(str)) => {
                        cprintln!(yellow "{str}");
                        let _ = rl.readline("Press enter to continue...");
                    }
                    Ok(Action::Quit(str)) => {
                        cprintln!(yellow "{str}");
                        file.to_file(path)?;
                        return Ok(());
                    }
                    Err(err) => {
                        cprintln!(red "{err}");
                        let _ = rl.readline("Press enter to continue...");
                    }
                }

                file.to_file(path.to_owned())?;
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                file.to_file(path)?;
                cprintln!(yellow "Quitting");
                return Ok(());
            }
            Err(err) => return Err(err.into()),
        }
    }
}
