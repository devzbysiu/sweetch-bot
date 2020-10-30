pub(crate) fn setup_logger() {
    let _ = flexi_logger::Logger::with_str("debug").start();
}
