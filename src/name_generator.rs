use std::sync::atomic::{Ordering, AtomicUsize};

static TMP_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn gen_tmp_name() -> String {
    let counter = TMP_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("tmp.{}", counter)
}

pub fn uniquify_identifier(name: &String) -> String {
    let counter = TMP_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("{}.{}", name, counter)
}