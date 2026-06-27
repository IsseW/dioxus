use std::{cell::Cell, thread, time::Duration};

use cross_tls_crate::get_bar;
use cross_tls_crate_dylib::get_baz;

fn main() {
    dioxus_devtools::connect_subsecond();
    const MAGIC: u64 = 0x123456789;
    cross_tls_crate::set_identity(MAGIC);
    loop {
        dioxus_devtools::subsecond::call(|| {
            let id = cross_tls_crate::identity();
            assert_eq!(id, MAGIC, "TLS NOT SHARED: patch read a private copy");

            use rayon::prelude::*;
            let sum: u64 = (0..1000u64).into_par_iter().sum();
            println!("rayon sum = {sum}");

            use cross_tls_crate::BAR;
            use cross_tls_crate_dylib::BAZ;

            thread_local! {
                pub static FOO: Cell<f32> = const { Cell::new(2.0) };
            }

            println!("Foo: {}", FOO.get());
            get_bar().with(|f| println!("Bar: {:?}", f.borrow()));
            thread::sleep(Duration::from_secs(1));

            FOO.set(2.0);
            get_bar().with(|f| f.borrow_mut().as_mut().unwrap().value = 3.0);
            get_baz().with(|f| f.borrow_mut().as_mut().unwrap().value = 4.0);

            BAR.with_borrow(|f| {
                println!("Bar: {:?}", f);
            });
            BAZ.with_borrow(|f| {
                println!("Baz: {:?}", f);
            });
        });
    }
}
