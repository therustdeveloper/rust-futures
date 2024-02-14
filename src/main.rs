use futures::Future;
use futures::executor::LocalPool;
use futures::task::LocalSpawnExt;

fn main() {
    simple_future(false);

    executor_future(true);
}

// running a simple future
fn returns_future() -> impl Future<Output=i32> {
    futures::future::ready(1)
}

fn simple_future(show: bool) {
    if show {
        let future = returns_future();

        let output = futures::executor::block_on(future);
        println!("future output: {}", output);
    }
}

// executor future
fn executor_future(show: bool) {
    if show {
        let mut executor = LocalPool::new();
        let spawner = executor.spawner();

        spawner.spawn_local(async {
            println!("Running on the executor");
        }).unwrap();

        executor.run();
    }
}