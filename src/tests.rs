// 编写 _get_html3 测试用例
#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::_get_html3;

    use super::*;

    #[test]
    fn test_get_html3() {
        _get_html3("https://docs.rs/v8/latest/v8".to_string());
    }

    use futures::stream::FuturesUnordered;
    use futures::stream::StreamExt;
    use rand::Rng;
    use tokio::time::sleep;
    use tokio::time::Instant; // For `.next()`.

    async fn random_pause() {
        let mut rng = rand::thread_rng();
        let pause = rng.gen_range(1..=10);
        // println!("Pausing for {} seconds...", pause);

        let start = Instant::now();
        sleep(Duration::from_secs(pause)).await;
        // println!("Resumed after {} seconds", start.elapsed().as_secs());
    }

    async fn do_something_async(i: i32) -> i32 {
        // Do something asynchronous here...
        random_pause().await;
        i
    }

    #[tokio::test]
    async fn test_futuresunordered() {
        let mut futures = FuturesUnordered::new();

        for i in 0..10 {
            futures.push(do_something_async(i));
        }

        while let Some(result) = futures.next().await {
            println!("Got result: {}", result);
        }
    }
}
