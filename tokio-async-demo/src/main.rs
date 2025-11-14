//https://www.doubao.com/thread/wed1dbf7a8a3b9fe6 基于这个知识点写的代码，可以实际体验一下异步编程的执行顺序
// 引入 Tokio 核心模块（必须在 main 函数前标注 #[tokio::main]）
use tokio::net::TcpStream;
use tokio::time::{sleep, timeout, Duration};
use std::error::Error;

// 标注这是 Tokio 异步主函数
// 底层会创建多线程运行时（默认线程数 = CPU 核心数）
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("===== 1. 基础异步任务：异步睡眠 =====");
    // async 块创建一个 Future，await 等待其完成（不阻塞线程）
    let async_task = async {
        println!("开始异步睡眠 2 秒...");
        sleep(Duration::from_secs(2)).await;  // 异步睡眠（非阻塞）
        "睡眠完成！"  // 异步任务的返回值
    };
    let result = async_task.await;
    println!("异步任务结果：{}", result);

    println!("\n===== 2. 并发执行多个异步任务（join!） =====");
    // 定义 3 个不同延迟的异步任务
    let task1 = async {
        sleep(Duration::from_secs(1)).await;
        println!("任务 1 完成（延迟 1 秒）");
        1  // 返回值：i32
    };
    let task2 = async {
        sleep(Duration::from_secs(3)).await;
        println!("任务 2 完成（延迟 3 秒）");
        2  // 返回值：i32
    };
    let task3 = async {
        sleep(Duration::from_secs(2)).await;
        println!("任务 3 完成（延迟 2 秒）");
        3  // 返回值：i32
    };
    // tokio::join! 并发执行多个 Future，等待所有完成后返回结果 tuple
    let (res1, res2, res3) = tokio::join!(task1, task2, task3);
    println!("并发任务总结果：({}, {}, {})", res1, res2, res3);
    println!("注意：总耗时 ≈ 最长任务的延迟（3 秒），而非 1+2+3=6 秒");

    println!("\n===== 3. 带超时的异步任务（timeout） =====");
    let long_task = async {
        sleep(Duration::from_secs(5)).await;  // 要执行 5 秒的任务
        "任务正常完成"
    };
    // 给任务设置 3 秒超时：如果 3 秒内没完成，返回超时错误
    match timeout(Duration::from_secs(3), long_task).await {
        Ok(result) => println!("超时任务结果：{}", result),
        Err(_) => println!("超时任务失败：任务执行超过 3 秒！"),
    }

    println!("\n===== 4. 异步网络请求（TCP 连接） =====");
    // 异步 TCP 连接（非阻塞，不会卡住线程）
    match TcpStream::connect(("baidu.com", 80)).await {
        Ok(stream) => {
            println!("TCP 连接成功！");
            println!("本地地址：{}", stream.local_addr()?);
            println!("远程地址：{}", stream.peer_addr()?);
        }
        Err(e) => println!("TCP 连接失败：{}", e),
    }

    Ok(())
}