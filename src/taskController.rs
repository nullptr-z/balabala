use std::future;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

pub struct TaskController {
    tasks: Vec<Pin<Box<dyn future::Future<Output = String>>>>,
    context: Context<'static>,
}

impl future::Future for TaskController {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.run_task();
        if self.tasks.is_empty() {
            Poll::Ready(())
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

impl TaskController {
    pub fn new() -> Self {
        let waker = Box::leak(Box::new(create_waker()));
        let context = Context::from_waker(waker);

        Self {
            tasks: Vec::new(),
            context,
        }
    }

    pub fn awaits(mut self) -> Poll<()> {
        let waker = Box::leak(Box::new(create_waker()));
        let mut context = Context::from_waker(waker);

        future::Future::poll(Pin::new(&mut self), &mut context)
    }

    pub fn spawn(&mut self, task: Pin<Box<dyn future::Future<Output = String>>>) {
        self.tasks.push(task);
    }

    pub fn run_task(&mut self) {
        while !self.tasks.is_empty() {
            let mut idx = 0;
            while idx < self.tasks.len() {
                let task = &mut self.tasks[idx];
                match task.as_mut().poll(&mut self.context) {
                    Poll::Ready(value) => {
                        println!("【 value 】==> {:?}", value);
                        self.tasks.swap_remove(idx);
                    }
                    Poll::Pending => {
                        idx += 1;
                    }
                }
            }
        }
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

    use super::TaskController;

    #[test]
    fn test_task_controller() {
        let mut task_controller = TaskController::new();
        task_controller.spawn(Box::pin(async { "Hello".to_string() }));
        task_controller.spawn(Box::pin(async { "future".to_string() }));

        task_controller.awaits();
    }
}