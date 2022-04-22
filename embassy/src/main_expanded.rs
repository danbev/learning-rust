#![feature(prelude_import)]
#![feature(type_alias_impl_trait)]
#![feature(fmt_internals)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;

use embassy::executor::Spawner;
use embassy::time::{Duration, Timer};
use log::*;

fn run() -> ::embassy::executor::SpawnToken<impl ::core::future::Future + 'static> {
    use ::embassy::executor::raw::TaskStorage;
    async fn task() {
        loop {
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                   {
                    ::log::__private_api_log(::core::fmt::Arguments::new_v1(&["tick"],
                                                                            &match ()
                                                                                 {
                                                                                 ()
                                                                                 =>
                                                                                 [],
                                                                             }),
                                             lvl,
                                             &("embassy_exploration",
                                               "embassy_exploration",
                                               "src/main.rs", 10u32));
                }
            };
            Timer::after(Duration::from_secs(1)).await;
        }
    }
    type F = impl ::core::future::Future + 'static;
    #[allow(clippy :: declare_interior_mutable_const)]
    const NEW_TASK: TaskStorage<F> = TaskStorage::new();
    static POOL: [TaskStorage<F>; 1usize] = [NEW_TASK; 1usize];
    unsafe { TaskStorage::spawn_pool(&POOL, move || task()) }
}

fn __embassy_main(spawner: Spawner)
 -> ::embassy::executor::SpawnToken<impl ::core::future::Future + 'static> {
    use ::embassy::executor::raw::TaskStorage;
    async fn task(spawner: Spawner) {
        {
            env_logger::builder().filter_level(log::LevelFilter::Debug).format_timestamp_nanos().init();
            spawner.spawn(run()).unwrap();
        }
    }
    type F = impl ::core::future::Future + 'static;
    #[allow(clippy :: declare_interior_mutable_const)]
    const NEW_TASK: TaskStorage<F> = TaskStorage::new();
    static POOL: [TaskStorage<F>; 1usize] = [NEW_TASK; 1usize];
    unsafe { TaskStorage::spawn_pool(&POOL, move || task(spawner)) }
}

fn main() -> ! {
    unsafe fn make_static<T>(t: &mut T) -> &'static mut T {
        ::core::mem::transmute(t) // This is similar to C's memcpy.
    }
    let mut executor = ::embassy::executor::Executor::new();
    let executor = unsafe { make_static(&mut executor) };
    executor.run(|spawner|
                     { spawner.spawn(__embassy_main(spawner)).unwrap(); })
}
