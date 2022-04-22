#![feature(type_alias_impl_trait)]
extern crate std;

use embassy::executor::{Executor, Spawner, SpawnToken};
use embassy::executor::raw::TaskStorage;


fn make_task_storage() -> ::embassy::executor::SpawnToken<impl ::core::future::Future + 'static> {
    type F = impl ::core::future::Future + 'static;
    static new_task: TaskStorage<F> = TaskStorage::new();
    async fn task() {
        println!("new_task task...");
    }
    //unsafe { new_task.spawn(|| task()) }
    unsafe { TaskStorage::spawn(&new_task, || task()) }

}

fn main() -> () {
    unsafe fn make_static<T>(t: &mut T) -> &'static mut T {
        ::core::mem::transmute(t)
    }
    let mut executor = Executor::new();
    let executor = unsafe { make_static(&mut executor) };


    let init = |spawner: Spawner| { 
        println!("spawer init...");
        spawner.spawn(make_task_storage());
    };
    executor.run(init);
}
