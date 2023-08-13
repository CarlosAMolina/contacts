use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::writer::MakeWriterExt;

use api::config_api;
use api::{run, setup_store};

#[tokio::main]
async fn main() {
    let config = config_api::Config::new();
    let store = setup_store(&config).await;

    let log_filter = format!(
        // TODO "handle_errors={},api={},warp={}",
        "api={},warp={}", // TODO
        config.log_level_api, config.log_level_warp
    );
    let logfile = RollingFileAppender::new(
        Rotation::DAILY,
        &config.log_path_name,
        &config.log_file_name,
    );
    let (non_blocking_logfile, _guard_logfile) = tracing_appender::non_blocking(logfile);
    let (non_blocking_stdout, _guard_stdout) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_writer(non_blocking_logfile.and(non_blocking_stdout))
        //.with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();

    tracing::info!("Q&A service build ID {}", env!("RUST_WEB_DEV_VERSION"));
    tracing::info!(
        "Server running addr={}.{}.{}.{}:{}",
        config.api_host[0],
        config.api_host[1],
        config.api_host[2],
        config.api_host[3],
        config.api_port
    );
    run(config, store).await;
}
