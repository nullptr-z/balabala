use std::fmt::Debug;
use std::future;
use std::pin::Pin;
use std::process::Output;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use async_trait::async_trait;
use futures::Future;

#[derive(Debug)]
pub struct MyFuture<T>(T);

impl<T: Default + Unpin> future::Future for MyFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        Poll::Ready(std::mem::replace(&mut this.0, Default::default()))
    }
}

impl<T: Future<Output = T>> MyFuture<T> {
    pub async fn polls(self) -> T::Output {
        self.0.await
    }
}

pub struct TaskController<T> {
    // todo: 改成自己 Vec<Futures>
    tasks: Vec<Pin<Box<dyn future::Future<Output = T>>>>,
    context: Context<'static>,
}

impl<T> TaskController<T> {
    pub fn new() -> Self {
        let waker = Box::leak(Box::new(create_waker()));
        let context = Context::from_waker(waker);

        Self {
            tasks: Vec::new(),
            context,
        }
    }

    pub fn spawn_join(&mut self, task: Pin<Box<dyn future::Future<Output = T>>>) {
        self.tasks.push(task);
    }

    pub fn awaits(&mut self) -> Vec<T> {
        let mut result = Vec::new();
        while !self.tasks.is_empty() {
            let mut idx = 0;
            while idx < self.tasks.len() {
                let task = &mut self.tasks[idx];
                match task.as_mut().poll(&mut self.context) {
                    Poll::Ready(value) => {
                        self.tasks.swap_remove(idx);
                        result.push(value);
                    }
                    Poll::Pending => {
                        self.context.waker().wake_by_ref();
                        idx += 1;
                    }
                }
            }
        }

        result
    }
}

// Context（上下文）是一个封装了异步操作执行所需环境和信息的结构体。
// 它包含了与异步执行相关的信息，如唤醒器（Waker）和调度器。
// Context是通过Waker和其他数据来提供给Future的poll方法，以便Future能够与异步运行时进行交互。
// ----------------------------------------------------
// Waker（唤醒器）是一个用于唤醒（Wake）异步操作的机制。
// 当异步操作在poll方法中需要进行阻塞等待时，它可以通过Waker请求被唤醒，以便再次执行。
// Waker是由异步运行时创建的，并在需要唤醒任务时传递给任务的poll方法。
// - 通过Context和Waker的结合使用，异步操作能够在合适的时机进行自我唤醒，以推动异步任务的执行。
// todo: 暂不深究
fn create_waker() -> Waker {
    fn dummy(_: *const ()) {}

    static VTABLE: RawWakerVTable =
        RawWakerVTable::new(|ptr| RawWaker::new(ptr, &VTABLE), dummy, dummy, dummy);

    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
}

#[cfg(test)]
mod test_task_controller {
    use super::{MyFuture, TaskController};

    #[test]
    fn test_polls() {
        let res = reqwest::get("https://docs.rs/v8/latest/v8".to_string());
        let my_fu = MyFuture(res);
        // my_fu.polls();
    }

    #[test]
    fn test_task_controller() {
        let mut task_controller = TaskController::new();
        task_controller.spawn_join(Box::pin(MyFuture(123)));
        task_controller.spawn_join(Box::pin(MyFuture(233)));
        let result = task_controller.awaits();
        println!("Result: {:?}", result);

        let mut task_controller = TaskController::new();
        task_controller.spawn_join(Box::pin(MyFuture("哈哈")));
        let result = task_controller.awaits();
        println!("Result: {:?}", result);

        async {
            let fu = MyFuture(999).await;
            println!("【 fu 】==> {:?}", fu);
        };
    }
}
