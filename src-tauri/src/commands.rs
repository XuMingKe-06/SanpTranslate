use crate::config::{AppConfig, ConfigManager};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use image::ImageEncoder;
use std::error::Error;
use tauri::Manager;

#[tauri::command]
pub fn get_config(app: tauri::AppHandle) -> Result<AppConfig, String> {
    let manager = ConfigManager::new(&app).map_err(|e| e.to_string())?;
    manager.load().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_config(app: tauri::AppHandle, config: AppConfig) -> Result<(), String> {
    let manager = ConfigManager::new(&app).map_err(|e| e.to_string())?;
    manager.save(&config).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn write_clipboard_image(image_data: String, app: tauri::AppHandle) -> Result<(), String> {
    log::info!("[CMD] write_clipboard_image 被调用，数据长度={}", image_data.len());
    crate::clipboard::write_clipboard_image(&app, &image_data).map_err(|e| {
        log::error!("[CMD] 写入剪贴板失败: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub fn read_clipboard_image(app: tauri::AppHandle) -> Result<Option<String>, String> {
    crate::clipboard::read_clipboard_image(&app).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn write_clipboard_text(text: String, app: tauri::AppHandle) -> Result<(), String> {
    crate::clipboard::write_clipboard_text(&app, &text).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn close_pin_window(window_id: String, app: tauri::AppHandle) -> Result<(), String> {
    crate::window::close_pin_window(&app, &window_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_pin_image(window_id: String, app: tauri::AppHandle) -> Result<Option<String>, String> {
    crate::window::get_pin_image(&app, &window_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn capture_region_from_cache(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    app: tauri::AppHandle,
) -> Result<crate::window::CropResult, String> {
    log::info!("[CMD] capture_region_from_cache 被调用，x={}, y={}, w={}, h={}", x, y, width, height);

    let cached_screen = {
        let store = app.state::<std::sync::Mutex<crate::window::CachedScreenStore>>();
        let mut store = store.lock().map_err(|e| {
            log::error!("[CMD] 锁定 CachedScreenStore 失败: {}", e);
            e.to_string()
        })?;
        store.screen.take()
    };

    let mut screen = cached_screen.ok_or_else(|| {
        log::error!("[CMD] 缓存中无全屏截图数据");
        "缓存中无全屏截图数据，请重新截图".to_string()
    })?;

    let cropped = image::imageops::crop(&mut screen.image, x, y, width, height);
    let cropped_image = cropped.to_image();

    let rgba_for_clipboard = cropped_image.as_raw().clone();
    let crop_w = cropped_image.width();
    let crop_h = cropped_image.height();

    let png_bytes = encode_png_fast(&cropped_image).map_err(|e| {
        log::error!("[CMD] PNG 编码失败: {}", e);
        e.to_string()
    })?;

    let base64_data = STANDARD.encode(&png_bytes);

    // 异步写入剪贴板（不阻塞返回）
    let app_clone = app.clone();
    std::thread::spawn(move || {
        match crate::clipboard::write_clipboard_image_raw(&app_clone, rgba_for_clipboard, crop_w, crop_h) {
            Ok(_) => log::info!("[CMD] 异步剪贴板写入成功"),
            Err(e) => log::error!("[CMD] 异步剪贴板写入失败: {}", e),
        }
    });

    let scale_factor = screen.scale_factor;
    let logical_x = (screen.monitor_x + x as i32) as f64 / scale_factor;
    let logical_y = (screen.monitor_y + y as i32) as f64 / scale_factor;
    let logical_w = width as f64 / scale_factor;
    let logical_h = height as f64 / scale_factor;

    // 阴影内边距和控制栏高度，需与前端 PinView.vue 保持一致
    const PIN_PADDING: f64 = 14.0;
    const CONTROL_BAR_H: f64 = 36.0;

    let result = crate::window::CropResult {
        base64_data,
        // 窗口位置左移/上移一个 PIN_PADDING，配合前端 padding 使图片保持在原始裁剪位置
        x: logical_x - PIN_PADDING,
        y: logical_y - PIN_PADDING,
        width: logical_w + PIN_PADDING * 2.0,
        height: logical_h + CONTROL_BAR_H + PIN_PADDING * 2.0,
        crop_width: width,
        crop_height: height,
    };

    log::info!("[CMD] capture_region_from_cache 完成，返回 CropResult");
    Ok(result)
}

/// 使用快速压缩级别编码 PNG，性能远优于默认压缩
fn encode_png_fast(image: &image::RgbaImage) -> Result<Vec<u8>, String> {
    let mut buf = std::io::Cursor::new(Vec::new());
    let encoder = image::codecs::png::PngEncoder::new_with_quality(
        &mut buf,
        image::codecs::png::CompressionType::Fast,
        image::codecs::png::FilterType::Sub,
    );
    encoder
        .write_image(
            image.as_raw(),
            image.width(),
            image.height(),
            image::ExtendedColorType::Rgba8,
        )
        .map_err(|e| format!("PNG 快速编码失败: {}", e))?;
    Ok(buf.into_inner())
}

#[tauri::command]
pub fn get_overlay_image(app: tauri::AppHandle) -> Result<Option<crate::window::OverlayImageData>, String> {
    let store = app.state::<std::sync::Mutex<crate::window::CachedScreenStore>>();
    let mut store = store.lock().map_err(|e| {
        log::error!("[CMD] 锁定 CachedScreenStore 失败: {}", e);
        e.to_string()
    })?;
    Ok(store.overlay_image.take())
}

#[tauri::command]
pub fn store_pin_image(label: String, image_data: String, app: tauri::AppHandle) -> Result<(), String> {
    let store = app.state::<std::sync::Mutex<crate::window::PinImageStore>>();
    let mut store = store.lock().map_err(|e| {
        log::error!("[CMD] 锁定 PinImageStore 失败: {}", e);
        e.to_string()
    })?;
    store.images.insert(label.clone(), image_data);
    log::info!("[CMD] store_pin_image: 图像数据已存储，label={}, store中共{}条记录", label, store.images.len());
    Ok(())
}

#[tauri::command]
pub async fn translate_image(
    image_data: String,
    target_language: String,
    app: tauri::AppHandle,
) -> Result<crate::translate::TranslateResult, String> {
    // 加载配置
    let config_manager = crate::config::ConfigManager::new(&app).map_err(|e| e.to_string())?;
    let config = config_manager.load().map_err(|e| e.to_string())?;

    // 获取 API 密钥
    log::info!("[CMD] translate_image: 正在从密钥环读取 API 密钥...");
    let api_key = config_manager.get_api_key().map_err(|e| {
        log::error!("[CMD] translate_image: 读取 API 密钥失败: {}", e);
        e.to_string()
    })?;
    let api_key = api_key.ok_or_else(|| {
        log::error!("[CMD] translate_image: API 密钥未配置");
        "API 密钥未配置，请在设置中配置 API 密钥".to_string()
    })?;
    log::info!("[CMD] translate_image: API 密钥读取成功");

    // ===== 第一步：OCR 识别文字（同时用于历史缓存匹配和后续翻译） =====
    log::info!("[CMD] translate_image: 正在进行 OCR 识别...");
    let ocr_blocks = crate::ocr::extract_text_with_positions(
        &app, &image_data, "chi_sim+eng"
    ).await.map_err(|e| format!("OCR识别失败: {}", e))?;

    if ocr_blocks.is_empty() {
        log::info!("[CMD] OCR未识别到文字，返回空结果");
        return Ok(crate::translate::TranslateResult { blocks: Vec::new(), from_cache: false });
    }

    // 拼接 OCR 文本用于历史匹配（使用 \n 以匹配数据库存储格式）
    let ocr_text = ocr_blocks
        .iter()
        .map(|b| b.text.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    log::info!("[CMD] OCR识别完成，共 {} 个文字块", ocr_blocks.len());

    // ===== 第二步：查找历史缓存 =====
    {
        let history_service = app.state::<std::sync::Mutex<crate::history::HistoryService>>();
        let service = history_service.lock().map_err(|e| e.to_string())?;
        if let Some((id, blocks_json)) = service.find_by_ocr_text(&ocr_text, &target_language)
            .map_err(|e| e.to_string())?
        {
            if let Ok(blocks) = serde_json::from_str::<Vec<crate::translate::TranslatedBlock>>(&blocks_json) {
                log::info!("[CMD] OCR文本匹配历史缓存(id={})，跳过API翻译", id);
                return Ok(crate::translate::TranslateResult { blocks, from_cache: true });
            } else {
                log::warn!("[CMD] 历史缓存 blocks_json 解析失败，重新翻译");
            }
        }
    }

    // ===== 第三步：未命中缓存，调用 API 翻译（使用已有 OCR 结果，避免重复 OCR） =====
    log::info!("[CMD] translate_image: 未命中缓存，调用翻译 API...");
    let result = crate::translate::translate_with_ocr_blocks(
        ocr_blocks,
        &config.api_base_url,
        &api_key,
        &config.model,
        &target_language,
    )
    .await
    .map_err(|e| e.to_string())?;

    // ===== 第四步：保存历史记录（含缓存数据） =====
    if !result.blocks.is_empty() {
        let app_clone = app.clone();
        let image_data_clone = image_data;
        let ocr_text_clone = ocr_text;
        let translated_text = result.blocks.iter().map(|b| b.translated.as_str()).collect::<Vec<_>>().join("\n");
        let blocks_json = serde_json::to_string(&result.blocks).map_err(|e| e.to_string())?;
        let target_language_clone = target_language;

        std::thread::spawn(move || {
            match save_translation_history(
                &app_clone, &image_data_clone, &ocr_text_clone, &translated_text,
                &target_language_clone, &blocks_json,
            ) {
                Ok(id) => log::info!("[CMD] 翻译历史已保存, id={}", id),
                Err(e) => log::error!("[CMD] 保存翻译历史失败: {}", e),
            }
        });
    }

    Ok(result)
}

/// 保存翻译历史记录到数据库（含目标语言和缓存 JSON）
fn save_translation_history(
    app: &tauri::AppHandle,
    image_base64: &str,
    ocr_text: &str,
    translated_text: &str,
    target_language: &str,
    blocks_json: &str,
) -> Result<i64, String> {
    // 将 Base64 图像数据解码为原始字节
    let image_bytes = STANDARD.decode(image_base64).map_err(|e| format!("Base64解码失败: {}", e))?;

    let history_service = app.state::<std::sync::Mutex<crate::history::HistoryService>>();
    let service = history_service.lock().map_err(|e| format!("锁定HistoryService失败: {}", e))?;

    let entry = crate::history::NewHistoryEntry {
        image_data: image_bytes,
        ocr_text: if ocr_text.is_empty() { None } else { Some(ocr_text.to_string()) },
        translated_text: translated_text.to_string(),
        target_language: target_language.to_string(),
        blocks_json: blocks_json.to_string(),
    };

    service.add_entry(entry).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_api_key(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let config_manager = crate::config::ConfigManager::new(&app).map_err(|e| e.to_string())?;
    let result = config_manager.get_api_key().map_err(|e| {
        log::error!("[CMD] get_api_key: 读取失败: {}", e);
        e.to_string()
    })?;
    log::info!("[CMD] get_api_key: 读取结果={}", if result.is_some() { "有密钥" } else { "无密钥" });
    Ok(result)
}

#[tauri::command]
pub fn set_api_key(key: String, app: tauri::AppHandle) -> Result<(), String> {
    log::info!("[CMD] set_api_key: 正在保存 API 密钥到密钥环...");
    let config_manager = crate::config::ConfigManager::new(&app).map_err(|e| e.to_string())?;
    config_manager.set_api_key(&key).map_err(|e| {
        log::error!("[CMD] set_api_key: 保存失败: {}", e);
        e.to_string()
    })?;
    log::info!("[CMD] set_api_key: API 密钥保存成功");
    Ok(())
}

#[tauri::command]
pub async fn test_api_connection(
    api_base_url: String,
    api_key: String,
    model: String,
    app: tauri::AppHandle,
) -> Result<String, String> {
    // 使用提供的参数直接测试，不读取配置
    let _ = app; // 避免 unused 警告

    let url = format!("{}/chat/completions", api_base_url.trim_end_matches('/'));

    let request_body = serde_json::json!({
        "model": model,
        "messages": [{
            "role": "user",
            "content": "Hello"
        }],
        "max_tokens": 5
    });

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .connect_timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("创建HTTP客户端失败: {}", e))?;
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| {
            let mut msg = format!("连接失败: {}", e);
            let mut source = e.source();
            while let Some(err) = source {
                msg.push_str(&format!("\n  原因: {}", err));
                source = err.source();
            }
            msg
        })?;

    if response.status().is_success() {
        Ok("连接成功".to_string())
    } else {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        // 尝试解析错误响应中的 message 字段，提供更友好的错误信息
        let error_msg = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
            json["error"]["message"]
                .as_str()
                .map(|s| s.to_string())
                .unwrap_or_else(|| body.clone())
        } else {
            body.clone()
        };
        // 根据状态码提供更友好的提示
        match status.as_u16() {
            401 => Err("API 密钥无效或已过期".to_string()),
            403 => Err("无权访问该 API，请检查密钥权限".to_string()),
            404 => Err("API 地址不存在，请检查地址是否正确".to_string()),
            429 => Err("请求过于频繁，请稍后再试".to_string()),
            500..=599 => Err("服务器错误，请稍后再试".to_string()),
            _ => Err(format!("连接失败: {}", error_msg)),
        }
    }
}

// ===== 历史记录相关命令 =====

#[tauri::command]
pub fn get_history_list(
    limit: u32,
    app: tauri::AppHandle,
) -> Result<Vec<crate::history::HistoryListItem>, String> {
    let history_service = app.state::<std::sync::Mutex<crate::history::HistoryService>>();
    let service = history_service.lock().map_err(|e| {
        log::error!("[CMD] 锁定 HistoryService 失败: {}", e);
        e.to_string()
    })?;
    service.get_list(limit).map_err(|e| {
        log::error!("[CMD] 获取历史列表失败: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub fn get_history_detail(
    id: i64,
    app: tauri::AppHandle,
) -> Result<crate::history::HistoryEntry, String> {
    let history_service = app.state::<std::sync::Mutex<crate::history::HistoryService>>();
    let service = history_service.lock().map_err(|e| {
        log::error!("[CMD] 锁定 HistoryService 失败: {}", e);
        e.to_string()
    })?;
    service.get_detail(id).map_err(|e| {
        log::error!("[CMD] 获取历史详情失败: id={}, error={}", id, e);
        e.to_string()
    })
}

#[tauri::command]
pub fn delete_history(id: i64, app: tauri::AppHandle) -> Result<bool, String> {
    let history_service = app.state::<std::sync::Mutex<crate::history::HistoryService>>();
    let service = history_service.lock().map_err(|e| {
        log::error!("[CMD] 锁定 HistoryService 失败: {}", e);
        e.to_string()
    })?;
    service.delete_entry(id).map(|_| true).map_err(|e| {
        log::error!("[CMD] 删除历史记录失败: id={}, error={}", id, e);
        e.to_string()
    })
}

#[tauri::command]
pub fn clear_history(app: tauri::AppHandle) -> Result<bool, String> {
    let history_service = app.state::<std::sync::Mutex<crate::history::HistoryService>>();
    let service = history_service.lock().map_err(|e| {
        log::error!("[CMD] 锁定 HistoryService 失败: {}", e);
        e.to_string()
    })?;
    service.clear_all().map(|_| true).map_err(|e| {
        log::error!("[CMD] 清空历史记录失败: {}", e);
        e.to_string()
    })
}
