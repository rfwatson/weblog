use wasm_bindgen::prelude::*;
use weblog::*;

#[wasm_bindgen(start)]
pub fn main() {
    // A simple example.
    console_log!("Hello world");

    // Multiple arguments.
    console_log!("Hello", "there", "world");

    // Various types.
    console_log!(
        1.0,
        2f64,
        true,
        "&str",
        String::from("owned string"),
        Some("an option"),
        None as Option<i32>,
    );

    // Various levels.
    console_debug!("debug");
    console_info!("informational");
    console_warn!("warning");
    console_error!("an", "error", "occurred");
    console_exception!("an", "exception", "occurred");

    // Print a stacktrace.
    console_trace!();

    // And include some data.
    console_trace!("some", "data", 1, 2, 3);

    // Unlimited arguments.
    //
    // Note that due to the design of the web-sys library, passing greater
    // than 7 variadic arguments (or 8 in the case of console_assert!)
    // implies a performance hit.
    console_log!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);

    // With a trailing comma.
    console_log!(1,);

    // With no arguments.
    console_log!();

    // With an assertion.
    console_assert!(true); // outputs nothing
    console_assert!(false);
    console_assert!(false, "Something went wrong");
    console_assert!(false, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, "Done!");

    // With a simple count.
    console_count!();
    console_count_reset!();

    // A count with a label.
    console_count!("myCounter");
    console_count!("myCounter");
    console_count!("myCounter");
    console_count_reset!("myCounter");

    // A timer.
    console_time!();
    console_time_end!();
    console_time!("with a label");
    console_time_log!("with a label", "and", 1, "data");
    console_time_log!("with a label", 1, 2, 3, 4, 5, 6, 7, 8, "data");
    console_time_end!("with a label");

    // A performance timestamp.
    console_time_stamp!(); // outputs nothing to console
    console_time_stamp!("with a label"); // outputs nothing to console
}
