mod app;

use app::App;

use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::prelude::*;
use tracing_web::{performance_layer, MakeConsoleWriter};

const MAX_LOG_LEVEL: LevelFilter = LevelFilter::DEBUG;

fn main() {
    // NOTE To cause an error you can either have
    // `tracing` enabled here, `BrowserRouter` enabled in app, or both.

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false) // Only partially supported across browsers
        .with_timer(UtcTime::rfc_3339()) // std::time is not available in browsers
        .with_writer(MakeConsoleWriter) // write events to the console
        .with_filter(MAX_LOG_LEVEL);

    tracing_subscriber::registry().with(fmt_layer).init();

    yew::Renderer::<App>::new().render();
}
