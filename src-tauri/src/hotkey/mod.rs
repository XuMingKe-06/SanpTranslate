use std::sync::{Arc, Mutex};
use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

use crate::config::ShortcutConfig;
use crate::error::AppError;

/// 当前已注册的快捷键（解析后的 Shortcut 对象），供全局处理器动态查询
pub struct CurrentShortcuts {
    pub capture: Shortcut,
    pub pin_clipboard: Shortcut,
    pub text_translate: Shortcut,
}

/// 注册全局快捷键（应用启动时调用）
pub fn register_hotkeys(app: &tauri::AppHandle, config: &ShortcutConfig) -> Result<(), AppError> {
    let capture_shortcut = parse_shortcut(&config.capture)?;
    let pin_clipboard_shortcut = parse_shortcut(&config.pin_clipboard)?;
    let text_translate_shortcut = parse_shortcut(&config.text_translate)?;

    // 使用 Arc<Mutex> 存储快捷键，闭包直接捕获 Arc 避免生命周期问题
    let shortcuts = Arc::new(Mutex::new(CurrentShortcuts {
        capture: capture_shortcut,
        pin_clipboard: pin_clipboard_shortcut,
        text_translate: text_translate_shortcut,
    }));

    // 存入应用状态，供 reregister_hotkeys 更新
    app.manage(shortcuts.clone());

    // 闭包捕获 Arc 的克隆，直接通过 Arc 访问快捷键
    let shortcuts_handler = shortcuts.clone();

    app.plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |app, shortcut, event| {
                if event.state() != ShortcutState::Pressed {
                    return;
                }

                // 通过 Arc 直接访问，将比较结果复制出来后再调用处理函数
                let is_capture;
                let is_pin;
                let is_text_translate;
                {
                    let current = shortcuts_handler.lock().unwrap();
                    is_capture = shortcut == &current.capture;
                    is_pin = shortcut == &current.pin_clipboard;
                    is_text_translate = shortcut == &current.text_translate;
                }

                if is_capture {
                    handle_capture_hotkey(app);
                } else if is_pin {
                    handle_pin_clipboard_hotkey(app);
                } else if is_text_translate {
                    handle_text_translate_hotkey(app);
                }
            })
            .build(),
    )
    .map_err(|e| AppError::ConfigError(format!("注册全局快捷键插件失败: {}", e)))?;

    app.global_shortcut()
        .register(capture_shortcut)
        .map_err(|e| {
            #[cfg(target_os = "macos")]
            log::warn!(
                "[PERMISSION] 快捷键注册失败需要辅助功能权限 (macOS)。\
                 请前往 系统设置 > 隐私与安全性 > 辅助功能 添加本应用"
            );
            AppError::ConfigError(format!("注册截图快捷键失败: {}", e))
        })?;

    app.global_shortcut()
        .register(pin_clipboard_shortcut)
        .map_err(|e| {
            #[cfg(target_os = "macos")]
            log::warn!(
                "[PERMISSION] 快捷键注册失败需要辅助功能权限 (macOS)。\
                 请前往 系统设置 > 隐私与安全性 > 辅助功能 添加本应用"
            );
            AppError::ConfigError(format!("注册剪贴板贴图快捷键失败: {}", e))
        })?;

    app.global_shortcut()
        .register(text_translate_shortcut)
        .map_err(|e| {
            #[cfg(target_os = "macos")]
            log::warn!(
                "[PERMISSION] 快捷键注册失败需要辅助功能权限 (macOS)。\
                 请前往 系统设置 > 隐私与安全性 > 辅助功能 添加本应用"
            );
            AppError::ConfigError(format!("注册文本翻译快捷键失败: {}", e))
        })?;

    Ok(())
}

/// 重新注册快捷键（配置变更后调用）
pub fn reregister_hotkeys(app: &tauri::AppHandle, new_config: &ShortcutConfig) -> Result<(), AppError> {
    let new_capture = parse_shortcut(&new_config.capture)?;
    let new_pin = parse_shortcut(&new_config.pin_clipboard)?;
    let new_text_translate = parse_shortcut(&new_config.text_translate)?;

    // 注销所有已注册的快捷键
    app.global_shortcut()
        .unregister_all()
        .map_err(|e| AppError::ConfigError(format!("注销快捷键失败: {}", e)))?;

    // 注册新的快捷键
    app.global_shortcut()
        .register(new_capture)
        .map_err(|e| AppError::ConfigError(format!("注册截图快捷键失败: {}", e)))?;

    app.global_shortcut()
        .register(new_pin)
        .map_err(|e| AppError::ConfigError(format!("注册剪贴板贴图快捷键失败: {}", e)))?;

    app.global_shortcut()
        .register(new_text_translate)
        .map_err(|e| AppError::ConfigError(format!("注册文本翻译快捷键失败: {}", e)))?;

    // 通过 Arc 更新状态中的快捷键，使处理器能匹配新的快捷键
    let shortcuts = app.state::<Arc<Mutex<CurrentShortcuts>>>();
    let mut current = shortcuts
        .lock()
        .map_err(|e| AppError::ConfigError(format!("锁定快捷键状态失败: {}", e)))?;
    current.capture = new_capture;
    current.pin_clipboard = new_pin;
    current.text_translate = new_text_translate;

    log::info!(
        "[HOTKEY] 快捷键已更新: 截图={}, 剪贴板贴图={}, 文本翻译={}",
        new_config.capture,
        new_config.pin_clipboard,
        new_config.text_translate
    );

    Ok(())
}

/// 截屏流程：先快速捕获原始像素确保裁剪数据就绪，再创建蒙版窗口
pub fn handle_capture_flow(app: &tauri::AppHandle) -> Result<(), AppError> {
    let monitor = app
        .primary_monitor()
        .ok()
        .flatten()
        .ok_or_else(|| AppError::ConfigError("获取主显示器信息失败".to_string()))?;
    let scale_factor = monitor.scale_factor();
    let monitor_x = (monitor.position().x as f64 * scale_factor).round() as i32;
    let monitor_y = (monitor.position().y as f64 * scale_factor).round() as i32;

    // 第一步：同步捕获原始像素（仅 raw capture，不编码，速度快）
    // 确保在蒙版窗口出现前裁剪数据已就绪，用户松开鼠标时可立即裁剪
    let rgba_image = {
        let state = app.state::<std::sync::Mutex<crate::capture::CaptureService>>();
        let locked = state.lock().map_err(|e| {
            AppError::ConfigError(format!("锁定截图服务失败: {}", e))
        })?;
        locked.capture_fullscreen_raw(None)?
    };

    // 第二步：立即存储原始像素到缓存，供后续裁剪使用
    {
        let store = app.state::<std::sync::Mutex<crate::window::CachedScreenStore>>();
        let mut store = store.lock().map_err(|e| {
            AppError::ConfigError(format!("锁定缓存失败: {}", e))
        })?;
        store.screen = Some(crate::window::CachedScreen {
            image: rgba_image.clone(),
            monitor_x,
            monitor_y,
            scale_factor,
        });
    }

    // 第三步：创建蒙版窗口（此时 screen 数据已就绪，前端可立即裁剪）
    crate::window::create_overlay_window_lazy(app)?;
    log::info!("[HOTKEY] overlay 窗口创建成功（屏幕数据已就绪，后台编码截图中...）");

    // 第四步：后台线程编码 JPEG（仅用于蒙版背景显示，不影响裁剪）
    let app_clone = app.clone();
    std::thread::spawn(move || {
        let result = (|| -> Result<(), AppError> {
            let dynamic_image = image::DynamicImage::ImageRgba8(rgba_image);
            let jpeg_base64 = crate::capture::encode_to_base64_jpeg(&dynamic_image, 50)?;

            {
                let store = app_clone.state::<std::sync::Mutex<crate::window::CachedScreenStore>>();
                let mut store = store.lock().map_err(|e| {
                    AppError::ConfigError(format!("锁定缓存失败: {}", e))
                })?;
                store.overlay_image = Some(crate::window::OverlayImageData {
                    data: jpeg_base64,
                    mime: "image/jpeg".to_string(),
                });
            }

            log::info!("[HOTKEY] 后台 JPEG 编码完成，蒙版背景数据已就绪");
            Ok(())
        })();

        if let Err(e) = result {
            log::error!("[HOTKEY] 后台 JPEG 编码失败: {}", e);
        }
    });

    Ok(())
}

fn handle_capture_hotkey(app: &tauri::AppHandle) {
    log::info!("[HOTKEY] 截图快捷键触发");

    let result = handle_capture_flow(app);

    if let Err(e) = result {
        log::error!("[HOTKEY] 截图快捷键处理失败: {}", e);
    }
}

fn handle_pin_clipboard_hotkey(app: &tauri::AppHandle) {
    log::info!("[HOTKEY] 剪贴板贴图快捷键触发");

    let result = (|| -> Result<(), AppError> {
        // 优先从后台监控缓存中获取最近的图片（即使当前剪贴板已被文本覆盖）
        let image_data = {
            let cache = app.state::<crate::clipboard::ClipboardImageCache>();
            match cache.get() {
                Some(data) => {
                    log::info!("[HOTKEY] 使用剪贴板图片缓存");
                    data
                }
                None => {
                    // 缓存中没有图片，回退到直接读取当前剪贴板
                    match crate::clipboard::read_clipboard_image(app)? {
                        Some(data) => data,
                        None => {
                            log::info!("[HOTKEY] 剪贴板无图像，跳过");
                            return Ok(());
                        }
                    }
                }
            }
        };

        use base64::Engine;
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(&image_data)
            .map_err(|e| AppError::ClipboardError(format!("Base64 解码失败: {}", e)))?;
        let img = image::load_from_memory(&bytes)
            .map_err(|e| AppError::ClipboardError(format!("图像解码失败: {}", e)))?;
        let (img_w, img_h) = (img.width(), img.height());

        let (mon_x, mon_y, mon_w, mon_h) = {
            let state = app.state::<std::sync::Mutex<crate::capture::CaptureService>>();
            let locked = state.lock().map_err(|e| {
                AppError::ConfigError(format!("锁定截图服务失败: {}", e))
            })?;
            locked.get_primary_monitor_info()?
        };

        let x = mon_x + ((mon_w - img_w) as i32) / 2;
        let y = mon_y + ((mon_h - img_h) as i32) / 2;

        crate::window::create_pin_window(app, &image_data, x, y, img_w, img_h)?;
        Ok(())
    })();

    if let Err(e) = result {
        log::error!("[HOTKEY] 剪贴板贴图快捷键处理失败: {}", e);
    }
}

/// 文本翻译快捷键处理：创建文本翻译窗口
fn handle_text_translate_hotkey(app: &tauri::AppHandle) {
    log::info!("[HOTKEY] 文本翻译快捷键触发");

    if let Err(e) = crate::window::create_text_translate_window(app) {
        log::error!("[HOTKEY] 创建文本翻译窗口失败: {}", e);
    }
}

fn parse_shortcut(shortcut_str: &str) -> Result<Shortcut, AppError> {
    let parts: Vec<&str> = shortcut_str.split('+').collect();

    let mut modifiers = Modifiers::empty();
    let mut key_code = None;

    for part in parts {
        let trimmed = part.trim();
        match trimmed.to_lowercase().as_str() {
            "ctrl" | "control" => modifiers |= Modifiers::CONTROL,
            "shift" => modifiers |= Modifiers::SHIFT,
            "alt" => modifiers |= Modifiers::ALT,
            "super" | "win" | "meta" => modifiers |= Modifiers::SUPER,
            _ => {
                key_code = Some(parse_key_code(trimmed)?);
            }
        }
    }

    let key = key_code.ok_or_else(|| {
        AppError::ConfigError(format!("快捷键缺少按键: {}", shortcut_str))
    })?;

    Ok(Shortcut::new(Some(modifiers), key))
}

fn parse_key_code(key: &str) -> Result<Code, AppError> {
    if key.len() > 1 {
        let lower = key.to_lowercase();
        if let Some(stripped) = lower.strip_prefix('f') {
            let num: u32 = stripped
                .parse()
                .map_err(|_| AppError::ConfigError(format!("无效的功能键: {}", key)))?;
            return match num {
                1 => Ok(Code::F1),
                2 => Ok(Code::F2),
                3 => Ok(Code::F3),
                4 => Ok(Code::F4),
                5 => Ok(Code::F5),
                6 => Ok(Code::F6),
                7 => Ok(Code::F7),
                8 => Ok(Code::F8),
                9 => Ok(Code::F9),
                10 => Ok(Code::F10),
                11 => Ok(Code::F11),
                12 => Ok(Code::F12),
                _ => Err(AppError::ConfigError(format!(
                    "不支持的功能键编号: {}",
                    num
                ))),
            };
        }
    }

    if key.len() == 1 {
        let c = key.chars().next().unwrap();
        if c.is_ascii_alphabetic() {
            return Ok(match c.to_ascii_uppercase() {
                'A' => Code::KeyA,
                'B' => Code::KeyB,
                'C' => Code::KeyC,
                'D' => Code::KeyD,
                'E' => Code::KeyE,
                'F' => Code::KeyF,
                'G' => Code::KeyG,
                'H' => Code::KeyH,
                'I' => Code::KeyI,
                'J' => Code::KeyJ,
                'K' => Code::KeyK,
                'L' => Code::KeyL,
                'M' => Code::KeyM,
                'N' => Code::KeyN,
                'O' => Code::KeyO,
                'P' => Code::KeyP,
                'Q' => Code::KeyQ,
                'R' => Code::KeyR,
                'S' => Code::KeyS,
                'T' => Code::KeyT,
                'U' => Code::KeyU,
                'V' => Code::KeyV,
                'W' => Code::KeyW,
                'X' => Code::KeyX,
                'Y' => Code::KeyY,
                'Z' => Code::KeyZ,
                _ => unreachable!(),
            });
        }
        if c.is_ascii_digit() {
            return Ok(match c {
                '0' => Code::Digit0,
                '1' => Code::Digit1,
                '2' => Code::Digit2,
                '3' => Code::Digit3,
                '4' => Code::Digit4,
                '5' => Code::Digit5,
                '6' => Code::Digit6,
                '7' => Code::Digit7,
                '8' => Code::Digit8,
                '9' => Code::Digit9,
                _ => unreachable!(),
            });
        }
    }

    Err(AppError::ConfigError(format!(
        "不支持的按键: {}",
        key
    )))
}
