use chrono::Utc;
use chrono_tz::Asia::Shanghai;

use tracing_appender::non_blocking::NonBlockingBuilder;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::RollingFileAppender;
use tracing_appender::rolling::Rotation;
use tracing_subscriber::Layer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
struct ShanghaiTime;

impl FormatTime for ShanghaiTime {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let now_shanghai = Utc::now().with_timezone(&Shanghai);
        write!(w, "{}", now_shanghai.format("%Y-%m-%d %H:%M:%S%.3f"))
    }
}
pub fn setup_logger() -> Result<WorkerGuard, anyhow::Error> {
    let file_appender = RollingFileAppender::builder()
                .rotation(Rotation::DAILY) // rotate log files once every hour
                .max_log_files(10)
                .filename_prefix("access") // log file names will be prefixed with `myapp.`
                .filename_suffix("log") // log file names will be suffixed with `.log`
                .build("./logs") // try to build an appender that stores log files in `/var/log`
    ?;
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
