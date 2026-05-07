// 历史记录模块 - 管理翻译历史的 CRUD 操作，生成缩略图

use crate::error::AppError;
use base64::Engine;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::path::Path;

/// 历史记录最大条数，超过时自动删除最旧的记录
const MAX_HISTORY: u32 = 50;

/// 缩略图最大尺寸（保持宽高比）
const THUMBNAIL_MAX_SIZE: u32 = 200;

/// 缩略图 JPEG 编码质量（通过 JpegEncoder 设置）
const THUMBNAIL_QUALITY: u8 = 80;

/// 历史记录条目（完整数据，用于详情查看）
#[derive(Debug, Clone, Serialize)]
pub struct HistoryEntry {
    /// 记录 ID
    pub id: i64,
    /// 缩略图数据（Base64 编码的 JPEG）
    pub thumbnail: String,
    /// OCR 识别原文
    pub ocr_text: Option<String>,
    /// 翻译后文本
    pub translated_text: String,
    /// 创建时间（YYYY-MM-DD HH:MM:SS 格式）
    pub created_at: String,
}

/// 历史记录列表条目（用于列表展示，不含大字段）
#[derive(Debug, Clone, Serialize)]
pub struct HistoryListItem {
    /// 记录 ID
    pub id: i64,
    /// 缩略图数据（Base64 编码的 JPEG）
    pub thumbnail: String,
    /// 翻译摘要（截取前 50 字符）
    pub summary: String,
    /// 创建时间（YYYY-MM-DD HH:MM:SS 格式）
    pub created_at: String,
}

/// 新建历史记录的输入数据
pub struct NewHistoryEntry {
    /// 原始图像数据（PNG/JPEG 等格式）
    pub image_data: Vec<u8>,
    /// OCR 识别原文
    pub ocr_text: Option<String>,
    /// 翻译后文本
    pub translated_text: String,
}

/// 历史记录服务，封装 SQLite 数据库操作
pub struct HistoryService {
    db: Connection,
}

impl HistoryService {
    /// 创建历史记录服务实例，初始化数据库表结构
    pub fn new(db_path: &Path) -> Result<Self, AppError> {
        // 确保数据库文件所在目录存在
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let db = Connection::open(db_path)?;

        // 创建历史记录表
        db.execute_batch(
            "CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                thumbnail BLOB NOT NULL,
                ocr_text TEXT,
                translated_text TEXT NOT NULL,
                created_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_history_created_at ON history(created_at DESC);"
        )?;

        log::info!("[HISTORY] 数据库初始化完成: {:?}", db_path);

        Ok(HistoryService { db })
    }

    /// 添加一条历史记录
    pub fn add_entry(&self, entry: NewHistoryEntry) -> Result<i64, AppError> {
        // 生成缩略图
        let thumbnail_bytes = generate_thumbnail(&entry.image_data)?;

        // 获取当前时间戳
        let created_at = chrono_now_iso8601();

        // 插入记录
        self.db.execute(
            "INSERT INTO history (thumbnail, ocr_text, translated_text, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![thumbnail_bytes, entry.ocr_text, entry.translated_text, created_at],
        )?;

        let id = self.db.last_insert_rowid();
        log::info!("[HISTORY] 添加历史记录: id={}, ocr_text长度={:?}, translated_text长度={}", 
            id, entry.ocr_text.as_ref().map(|t| t.len()), entry.translated_text.len());

        // 检查总记录数，超过上限时删除最旧的记录
        self.enforce_max_history()?;

        Ok(id)
    }

    /// 获取历史记录列表（按时间倒序，最多 limit 条）
    pub fn get_list(&self, limit: u32) -> Result<Vec<HistoryListItem>, AppError> {
        let mut stmt = self.db.prepare(
            "SELECT id, thumbnail, translated_text, created_at FROM history ORDER BY created_at DESC LIMIT ?1"
        )?;

        let items = stmt.query_map(params![limit], |row| {
            let id: i64 = row.get(0)?;
            let thumbnail_bytes: Vec<u8> = row.get(1)?;
            let translated_text: String = row.get(2)?;
            let created_at: String = row.get(3)?;

            // 缩略图转 Base64
            let thumbnail = base64::engine::general_purpose::STANDARD.encode(&thumbnail_bytes);

            // 翻译摘要：截取前 50 个字符（按 Unicode 字符边界截取）
            let summary = truncate_str(&translated_text, 50);

            Ok(HistoryListItem {
                id,
                thumbnail,
                summary,
                created_at,
            })
        })?.filter_map(|item| item.ok()).collect();

        Ok(items)
    }

    /// 获取单条历史记录详情
    pub fn get_detail(&self, id: i64) -> Result<HistoryEntry, AppError> {
        let mut stmt = self.db.prepare(
            "SELECT id, thumbnail, ocr_text, translated_text, created_at FROM history WHERE id = ?1"
        )?;

        let entry = stmt.query_row(params![id], |row| {
            let id: i64 = row.get(0)?;
            let thumbnail_bytes: Vec<u8> = row.get(1)?;
            let ocr_text: Option<String> = row.get(2)?;
            let translated_text: String = row.get(3)?;
            let created_at: String = row.get(4)?;

            let thumbnail = base64::engine::general_purpose::STANDARD.encode(&thumbnail_bytes);

            Ok(HistoryEntry {
                id,
                thumbnail,
                ocr_text,
                translated_text,
                created_at,
            })
        }).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => AppError::DatabaseError(format!("历史记录不存在: id={}", id)),
            other => AppError::DatabaseError(other.to_string()),
        })?;

        Ok(entry)
    }

    /// 删除单条历史记录
    pub fn delete_entry(&self, id: i64) -> Result<(), AppError> {
        let affected = self.db.execute(
            "DELETE FROM history WHERE id = ?1",
            params![id],
        )?;

        if affected == 0 {
            return Err(AppError::DatabaseError(format!("历史记录不存在: id={}", id)));
        }

        log::info!("[HISTORY] 删除历史记录: id={}", id);
        Ok(())
    }

    /// 清空所有历史记录
    pub fn clear_all(&self) -> Result<(), AppError> {
        self.db.execute("DELETE FROM history", [])?;
        log::info!("[HISTORY] 已清空所有历史记录");
        Ok(())
    }

    /// 获取历史记录总数
    pub fn count(&self) -> Result<u32, AppError> {
        let count: u64 = self.db.query_row(
            "SELECT COUNT(*) FROM history",
            [],
            |row| row.get(0),
        )?;
        Ok(count as u32)
    }

    /// 检查记录总数，超过上限时删除最旧的记录
    fn enforce_max_history(&self) -> Result<(), AppError> {
        let count = self.count()?;
        if count > MAX_HISTORY {
            let delete_count = count - MAX_HISTORY;
            self.db.execute(
                "DELETE FROM history WHERE id IN (SELECT id FROM history ORDER BY created_at ASC LIMIT ?1)",
                params![delete_count],
            )?;
            log::info!("[HISTORY] 超过最大记录数({}), 已删除 {} 条最旧记录", MAX_HISTORY, delete_count);
        }
        Ok(())
    }
}

/// 生成缩略图：将原始图像缩放到最大 200x200，编码为 JPEG
fn generate_thumbnail(image_data: &[u8]) -> Result<Vec<u8>, AppError> {
    let img = image::load_from_memory(image_data)
        .map_err(|e| AppError::DatabaseError(format!("图像解码失败: {}", e)))?;

    // 计算缩略图尺寸，保持宽高比
    let (w, h) = (img.width(), img.height());
    let (thumb_w, thumb_h) = if w > h {
        let ratio = THUMBNAIL_MAX_SIZE as f64 / w as f64;
        (THUMBNAIL_MAX_SIZE, (h as f64 * ratio) as u32)
    } else {
        let ratio = THUMBNAIL_MAX_SIZE as f64 / h as f64;
        ((w as f64 * ratio) as u32, THUMBNAIL_MAX_SIZE)
    };

    // 缩放图像
    let thumbnail = img.thumbnail(thumb_w, thumb_h);

    // 编码为 JPEG（指定质量）
    let mut buf = std::io::Cursor::new(Vec::new());
    let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, THUMBNAIL_QUALITY);
    thumbnail
        .write_with_encoder(encoder)
        .map_err(|e| AppError::DatabaseError(format!("缩略图编码失败: {}", e)))?;

    Ok(buf.into_inner())
}

/// 按 Unicode 字符边界截取字符串，超出部分用省略号代替
fn truncate_str(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_chars).collect();
        format!("{}...", truncated)
    }
}

/// 获取当前时间的 YYYY-MM-DD HH:MM:SS 格式字符串
fn chrono_now_iso8601() -> String {
    // 使用 std 时间，避免引入 chrono 依赖
    let now = std::time::SystemTime::now();
    let duration = now.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
    let secs = duration.as_secs();
    // 简单格式化：YYYY-MM-DD HH:MM:SS
    let days = secs / 86400;
    let time_of_day = secs % 86400;
    let hours = time_of_day / 3600;
    let minutes = (time_of_day % 3600) / 60;
    let seconds = time_of_day % 60;

    // 计算年月日（从 1970-01-01 开始）
    let (year, month, day) = days_to_ymd(days);

    format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", year, month, day, hours, minutes, seconds)
}

/// 将自 1970-01-01 以来的天数转换为年月日
fn days_to_ymd(mut days: u64) -> (u64, u64, u64) {
    let mut year = 1970;
    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if days < days_in_year {
            break;
        }
        days -= days_in_year;
        year += 1;
    }

    let leap = is_leap_year(year);
    let month_days = [
        31, if leap { 29 } else { 28 }, 31, 30, 31, 30,
        31, 31, 30, 31, 30, 31,
    ];

    let mut month = 1;
    for &md in &month_days {
        if days < md {
            break;
        }
        days -= md;
        month += 1;
    }

    (year, month, days + 1)
}

/// 判断是否为闰年
fn is_leap_year(year: u64) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}
