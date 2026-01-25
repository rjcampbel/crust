use std::sync::atomic::{Ordering, AtomicUsize};

static TMP_COUNTER: AtomicUsize = AtomicUsize::new(0);
static LBL_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn gen_var_name() -> String {
    let counter = TMP_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("tmp.{}", counter)
}