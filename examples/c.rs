#![feature(async_await, futures_api, await_macro)]

mod b;

use {
    self::b::{b, b_async},
    example_crate::a,
};

fn c() {}

pub async fn c_async() {
    await!(b_async());
}

fn main() {
    a();
    b();
    c();
}
