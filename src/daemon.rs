pub(crate) fn daemonize<F: FnOnce()>(fun: F) {
    fun();
}
