pub(crate) fn schedule<F: FnOnce()>(fun: F) {
    fun();
}
