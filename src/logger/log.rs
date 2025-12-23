use chrono::Utc;
use chrono_tz::Asia::Shanghai;

use logroller::{Compression, LogRollerBuilder, Rotation, RotationAge, TimeZone};
use tracing_appender::non_blocking::NonBlockingBuilder;
use tracing_appender::non_blocking::WorkerGuard;

use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;
struct ShanghaiTime;

impl FormatTime for ShanghaiTime {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let now_shanghai = Utc::now().with_timezone(&Shanghai);
        write!(w, "{}", now_shanghai.format("%Y-%m-%d %H:%M:%S%.3f"))
    }
}
pub fn setup_logger() -> Result<WorkerGuard, anyhow::Error> {
    let file_appender = LogRollerBuilder::new("./logs", "{{project_name}}.log")
        .rotation(Rotation::AgeBased(RotationAge::Daily)) // Rotate daily
        .max_keep_files(7) // Keep a week's worth of logs
        .time_zone(TimeZone::Local) // Use local timezone
        .build()?;
    let (non_blocking_writer, guard) = NonBlockingBuilder::default()
        .buffered_lines_limit(10 * 1000 * 1000)
        .finish(file_appender);

    let file_layer = tracing_subscriber::fmt::Layer::new()
        .with_timer(ShanghaiTime)
        .with_line_number(true)
        .with_target(true)
        .with_ansi(false)
        .with_writer(non_blocking_writer)
        .with_filter(tracing_subscriber::filter::LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(file_layer)
        .with(tracing_subscriber::filter::LevelFilter::INFO)
        .init();

    Ok(guard)
}
