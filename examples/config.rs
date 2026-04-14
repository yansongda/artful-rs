//! 配置初始化示例

use artful::{Artful, Config, HttpOptions};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // 基础配置初始化
    Artful::config(Config::default());
    
    // 带 HTTP 选项的配置
    let _config = Config {
        http: HttpOptions {
            timeout: Some(10),
            connect_timeout: Some(5),
        },
        ..Default::default()
    };
    
    // 强制覆盖已有配置
    let config_with_force = Config {
        _force: true,
        http: HttpOptions {
            timeout: Some(30),
            connect_timeout: Some(10),
        },
        ..Default::default()
    };
    Artful::config(config_with_force);
    
    // 带扩展配置（如支付渠道配置）
    let mut extra = HashMap::new();
    extra.insert("alipay".to_string(), json!({
        "app_id": "2016082000295641",
        "notify_url": "https://example.com/alipay/notify",
    }));
    extra.insert("wechat".to_string(), json!({
        "mch_id": "1234567890",
        "notify_url": "https://example.com/wechat/notify",
    }));
    
    let config_with_extra = Config {
        extra,
        http: HttpOptions {
            timeout: Some(5),
            connect_timeout: Some(3),
        },
        ..Default::default()
    };
    Artful::config(config_with_extra);
    
    // 获取全局配置
    let global_config = Artful::get_config();
    println!("HTTP timeout: {:?}", global_config.http.timeout);
    
    // 获取扩展配置中的渠道信息
    if let Some(alipay) = global_config.extra.get("alipay") {
        println!("Alipay config: {}", alipay);
    }
    
    println!("Config initialized successfully!");
}