use std::sync::atomic::{Ordering, AtomicUsize};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn gen_tmp_name() -> String {
    let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("tmp.{}", counter)
}

pub fn uniquify_identifier(name: &String) -> String {
    let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("{}.{}", name, counter)
}

pub fn gen_label(name: &str) -> String {
    let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("{}.{}", name, counter)
}