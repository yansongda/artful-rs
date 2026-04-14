use std::sync::OnceLock;
use std::time::Duration;

/// 全局 HTTP 客户端单例（共享连接池）
pub fn get_client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .pool_idle_timeout(Some(Duration::from_secs(90)))
            .pool_max_idle_per_host(usize::MAX)
            .build()
            .expect("Failed to create HTTP client")
    })
}
