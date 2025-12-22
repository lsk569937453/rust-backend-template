use crate::config::app_config::{Config, EnvConfig};

use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite, SqlitePool};

use std::path::Path;

use tracing::info;
const CREATE_SQL: &str = include_str!("../create.sql");

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<Sqlite>,
    pub config: Config,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, anyhow::Error> {
        let sqlite_pool = connect_and_setup_database("./data/db.sqlite").await?;

        sqlx::raw_sql(CREATE_SQL).execute(&sqlite_pool).await?;
        info!("Database initialized successfully");

        Ok(Self {
            db_pool: sqlite_pool,
            config,
        })
    }
}

pub async fn connect_and_setup_database(db_path: &str) -> Result<SqlitePool, anyhow::Error> {
    info!("数据库路径设置为: {}", db_path);

    // 1. (可选但推荐) 检查并创建数据库文件的父目录
    // 这能确保在路径为 "data/db.sqlite" 这类情况时，"data" 目录会被创建
    if let Some(parent_dir) = Path::new(db_path).parent()
        && !parent_dir.exists()
    {
        info!("数据库目录 {} 不存在，正在创建...", parent_dir.display());
        std::fs::create_dir_all(parent_dir)?;
    }
    let path = Path::new(db_path);
    if !path.exists() {
        info!("数据库文件 {} 不存在，正在创建...", db_path);
        // 通过创建一个空文件来确保文件的存在性和可写性
        // 如果这里失败，我们会得到一个非常明确的 I/O 错误
        std::fs::File::create(path).map_err(|e| {
            error!("无法在路径 '{}' 创建数据库文件: {}", db_path, e);
            anyhow!(
                "创建数据库文件 '{}' 失败: {}. 请检查程序是否具有在当前目录下创建文件的权限。",
                db_path,
                e
            )
        })?;
        info!("成功创建空的数据库文件。");
    }

    // 2. 构造数据库连接URL (对于本地文件，格式为 "sqlite:path/to/db.sqlite")
    let database_url = format!("sqlite:{}", db_path);

    // 3. 建立数据库连接池
    // SqlitePoolOptions::connect 在连接的数据库文件不存在时会自动创建它
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    info!("成功连接到 SQLite 数据库: {}", db_path);

    // 4. 执行建表SQL语句
    // `CREATE TABLE IF NOT EXISTS` 确保了只有在表不存在时才会创建
    info!("正在检查并创建所有数据表...");
    sqlx::query(CREATE_SQL).execute(&pool).await?;

    info!("所有数据表已成功创建或确认已存在。");

    // 5. 返回连接池
    Ok(pool)
}
