pub(crate) fn setup_logger() {
    let _logger_res = flexi_logger::Logger::with_str("debug").start();
}
