//https://www.doubao.com/thread/wdd7fa4d23d362f63 基于这个知识点写的代码，可以实际体验一下 Arc 和 RwLock 的使用
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

// 定义一个共享的配置结构体（模拟实际应用中的配置）
#[derive(Debug, Clone)]
struct AppConfig {
    app_name: String,
    max_connections: u32,
    debug_mode: bool,
}

impl AppConfig {
    // 构造函数：创建默认配置
    fn default() -> Self {
        AppConfig {
            app_name: "MyApp".to_string(),
            max_connections: 100,
            debug_mode: false,
        }
    }
}

fn main() {
    // 1. 创建初始配置（普通结构体，所有权归 main 线程）
    let initial_config = AppConfig::default();
    println!("初始配置: {:?}", initial_config);

    // 2. 用 RwLock 包装配置：提供「读写锁」支持（多线程安全的内部可变性）
    // RwLock 特点：多个读锁可同时持有，写锁独占（读多写少场景高效）
    let config_lock = RwLock::new(initial_config);

    // 3. 用 Arc 包装 RwLock：提供「原子引用计数」支持（多线程共享所有权）
    // Arc 特点：线程安全的引用计数，克隆时仅增加计数，不拷贝数据
    // Arc<T> 要求 T 必须实现 Send + Sync（RwLock 已满足）
    let shared_config = Arc::new(config_lock);

    // -------------------------- 模拟多线程读操作 --------------------------
    let mut reader_handles = Vec::new();
    for reader_id in 0..3 {
        // 克隆 Arc：计数从 1 → 2 → 3 → 4（main 持有 1，3 个读线程各持有 1）
        let config_clone = Arc::clone(&shared_config);

        // 创建读线程
        let handle = thread::spawn(move || {
            // 循环 2 次读取配置（模拟频繁读场景）
            for _ in 0..2 {
                // 获取「读锁」：如果当前有写锁，会阻塞直到写锁释放
                // read() 返回 Result<RwLockReadGuard<T>, PoisonError>
                // RwLockReadGuard 是智能指针，自动管理锁的生命周期（离开作用域自动释放锁）
                match config_clone.read() {
                    Ok(config) => {
                        println!(
                            "[读线程 {}] 读取配置：名称={}, 最大连接数={}, 调试模式={}",
                            reader_id, config.app_name, config.max_connections, config.debug_mode
                        );
                    }
                    Err(e) => eprintln!("[读线程 {}] 读取失败：{}", reader_id, e),
                }

                // 模拟读操作耗时
                thread::sleep(Duration::from_millis(100));
            }
        });

        reader_handles.push(handle);
    }

    // 让读线程先运行一段时间
    thread::sleep(Duration::from_millis(150));

    // -------------------------- 模拟单线程写操作 --------------------------
    // 克隆 Arc：计数增加到 5（写线程持有 1）
    let config_clone = Arc::clone(&shared_config);
    let writer_handle = thread::spawn(move || {
        println!("[写线程] 尝试修改配置...");
        // 获取「写锁」：如果当前有读锁/写锁，会阻塞直到所有锁释放
        // write() 返回 Result<RwLockWriteGuard<T>, PoisonError>
        // RwLockWriteGuard 是智能指针，自动管理锁的生命周期（离开作用域自动释放锁）
        match config_clone.write() {
            Ok(mut config) => {
                // 修改配置（通过 mutable guard 获得可变访问权）
                config.app_name = "MySuperApp".to_string();
                config.max_connections = 200;
                config.debug_mode = true;
                println!("[写线程] 配置修改完成：{:?}", config);
            }
            Err(e) => eprintln!("[写线程] 修改失败：{}", e),
        }
    });

    // -------------------------- 等待所有线程结束 --------------------------
    // 等待读线程完成（所有读锁释放）
    for handle in reader_handles {
        handle.join().unwrap();
    }
    // 等待写线程完成（写锁释放）
    writer_handle.join().unwrap();

    // -------------------------- 主线程验证最终配置 --------------------------
    // 主线程获取读锁，验证配置是否被修改
    let final_config = shared_config.read().unwrap();
    println!("主线程验证最终配置：{:?}", final_config);

    // 知识点：Arc 的引用计数在所有持有者离开作用域后自动减为 0，内存被释放
    // 不需要手动释放 Arc 或 RwLock，智能指针自动管理生命周期
}