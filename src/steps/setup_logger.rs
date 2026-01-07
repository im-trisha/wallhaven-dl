use fern::colors::{Color, ColoredLevelConfig};

pub fn setup_logger() -> crate::Result<()> {
    let colors = ColoredLevelConfig {
        error: Color::Red,
        warn: Color::Yellow,
        info: Color::Green,
        debug: Color::Blue,
        trace: Color::BrightBlue,
    };

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] {} {} - {}",
                humantime::format_rfc3339(std::time::SystemTime::now()),
                record.target(),
                colors.color(record.level()),
                message
            ));
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("./wallhaven-dl.logs")?)
        .apply()?;

    Ok(())
}
