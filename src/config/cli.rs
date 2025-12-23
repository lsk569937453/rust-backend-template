use clap::Parser;

/// SGS 后端服务器
#[derive(Debug, Parser)]
#[command(name = "{{project_name}}")]
#[command(about = "Backend Server")]
#[command(version)]
pub struct Cli {
    /// 配置文件路径
    #[arg(short = 'f', long)]
    pub config: String,
}
