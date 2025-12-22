CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    phone TEXT NOT NULL UNIQUE,
    created_at DATETIME  DEFAULT (datetime('now','localtime'))
);
CREATE TABLE IF NOT EXISTS devices (
    id TEXT PRIMARY KEY,
    device_uuid TEXT NOT NULL UNIQUE,
    user_id TEXT,                     -- 可为空：设备可能尚未绑定用户
    sys_info TEXT,
    bound_until  DATETIME  DEFAULT (datetime('now','localtime')),
    created_at DATETIME  DEFAULT (datetime('now','localtime'))
);
CREATE TABLE IF NOT EXISTS login_codes (
    id TEXT PRIMARY KEY,
    phone TEXT NOT NULL,
    created_at DATETIME DEFAULT (datetime('now','localtime'))
);
CREATE TABLE IF NOT EXISTS login_logs (
    id TEXT PRIMARY KEY,        -- UUID
    user_id TEXT NOT NULL,
    device_uuid TEXT,
    created_at DATETIME  DEFAULT (datetime('now','localtime'))
);
