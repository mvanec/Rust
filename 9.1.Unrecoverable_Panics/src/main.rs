fn main() {
    let v = vec![1, 2, 3];

    // This well panic. set RUST_BACKTRACE=1 to see
    // the full stack trace.
    v[99];
}
