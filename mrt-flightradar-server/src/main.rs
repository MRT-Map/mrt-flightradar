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

mod flight_generation;
mod purge;
mod status_calculation;
mod types_consts;

use std::{collections::HashMap, time::UNIX_EPOCH};

use color_eyre::eyre::Result;
use common::{
    data_types::{vec::Pos, RAW_DATA},
    flight_route::types::path::Path,
};
use glam::Vec2;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{Header, Status},
    response,
    response::{content, Responder},
    routes, Request, Response,
};
use serde::Serialize;
use tokio::time::Duration;
use tracing::error;
use tracing_subscriber::EnvFilter;
use types_consts::FLIGHT_ACTIONS;
use uuid::Uuid;

use crate::{
    flight_generation::generate_flights,
    purge::purge_outdated_data,
    status_calculation::calculate_statuses,
    types_consts::{ActiveFlight, FlightAction, FLIGHTS},
};

#[derive(Debug)]
struct CustomMsgPack<T>(pub T);

impl<'r, T: Serialize> Responder<'r, 'static> for CustomMsgPack<T> {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        #[allow(clippy::map_err_ignore)]
        let buf = rmp_serde::to_vec_named(&self.0).map_err(|_| Status::InternalServerError)?;

        content::RawMsgPack(buf).respond_to(req)
    }
}

#[rocket::get("/actions")]
async fn actions() -> CustomMsgPack<HashMap<String, Vec<FlightAction<'static>>>> {
    CustomMsgPack(
        FLIGHT_ACTIONS
            .lock()
            .await
            .iter()
            .map(|(k, v)| {
                (
                    k.duration_since(UNIX_EPOCH).unwrap().as_secs().to_string(),
                    v.to_owned(),
                )
            })
            .collect::<HashMap<_, _>>(),
    )
}

#[rocket::get("/flights")]
async fn flights() -> CustomMsgPack<Vec<ActiveFlight<'static>>> {
    CustomMsgPack(
        FLIGHTS
            .lock()
            .await
            .iter()
            .map(|a| (**a).to_owned())
            .collect::<Vec<_>>(),
    )
}

#[rocket::get("/airports")]
fn airports() -> CustomMsgPack<HashMap<&'static str, Pos<Vec2>>> {
    CustomMsgPack(
        RAW_DATA
            .waypoints
            .iter()
            .filter_map(|w| {
                (w.name.starts_with("AA") && &*w.name != "RDV").then(|| (&w.name[2..], w.coords))
            })
            .collect::<HashMap<_, _>>(),
    )
}

#[rocket::get("/route/<id>")]
async fn flight_route(id: String) -> Option<CustomMsgPack<Vec<Pos<Vec2>>>> {
    let id = id.parse::<Uuid>().ok()?;
    let flights = FLIGHTS.lock().await;
    let flight = flights.iter().find(|a| a.id == id)?;
    Some(CustomMsgPack(
        flight
            .route
            .0
            .iter()
            .flat_map(|p| match p {
                Path::Straight(fl) => {
                    vec![fl.tail, fl.head()]
                }
                Path::Curve {
                    centre,
                    from,
                    angle,
                } => (0..=36)
                    .map(|i| {
                        *centre
                            + (*from - *centre).rotate(Vec2::from_angle(i as f32 / 36f32 * *angle))
                    })
                    .collect(),
            })
            .collect::<Vec<_>>(),
    ))
}

// https://stackoverflow.com/questions/62412361/how-to-set-up-cors-or-options-for-rocket-rs
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[rocket::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .event_format(tracing_subscriber::fmt::format().without_time().compact())
        .with_env_filter(EnvFilter::from_env("RUST_LOG"))
        .init();

    let r = rocket::build()
        .mount("/", routes![actions, flights, flight_route, airports])
        .attach(CORS)
        .ignite()
        .await?;

    let h = tokio::spawn(async {
        loop {
            purge_outdated_data().await;
            match generate_flights().await {
                Ok(flights) => calculate_statuses(flights).await,
                Err(e) => error!("{e}"),
            };
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    });
    let _ = r.launch().await?;
    h.abort();
    Ok(())
}
