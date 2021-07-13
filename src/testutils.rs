pub(crate) fn setup_logger() {
    let _logger_res = flexi_logger::Logger::try_with_str("debug")
        .expect("failed to initialize logger")
        .start();
}
