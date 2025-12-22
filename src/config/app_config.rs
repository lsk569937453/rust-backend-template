use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 应用程序配置结构体 - 仅包含配置文件中的设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

/// 环境变量配置
#[derive(Debug, Clone)]
pub struct EnvConfig {
    pub aliyun_access_key_id: String,
    pub aliyun_access_key_secret: String,
}

impl EnvConfig {
    /// 初始化环境变量配置
    pub fn new() -> Result<Self, anyhow::Error> {
        let aliyun_access_key_id = std::env::var("ALIYUN_ACCESS_KEY_ID")
            .map_err(|_| anyhow::anyhow!("ALIYUN_ACCESS_KEY_ID 环境变量未设置"))?;
        let aliyun_access_key_secret = std::env::var("ALIYUN_ACCESS_KEY_SECRET")
            .map_err(|_| anyhow::anyhow!("ALIYUN_ACCESS_KEY_SECRET 环境变量未设置"))?;
        Ok(Self {
            aliyun_access_key_id,
            aliyun_access_key_secret,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig { port: 8080 },
        }
    }
}

impl Config {
    /// 从文件加载配置
    pub fn load_from_file<P: Into<PathBuf>>(path: P) -> Result<Self, anyhow::Error> {
        let path_buf = path.into();
        let content = std::fs::read_to_string(&path_buf)?;
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// 加载配置，如果失败则使用默认配置
    pub fn load_or_default<P: Into<PathBuf>>(path: P) -> Self {
        let path_buf = path.into();
        match Self::load_from_file(&path_buf) {
            Ok(config) => {
                tracing::info!("成功加载配置文件: {:?}", path_buf);
                config
            }
            Err(e) => {
                tracing::warn!("无法加载配置文件 {:?}，使用默认配置: {}", path_buf, e);
                Self::default()
            }
        }
    }
}
