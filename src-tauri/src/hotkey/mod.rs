use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

use crate::config::ShortcutConfig;
use crate::error::AppError;

pub fn register_hotkeys(app: &tauri::AppHandle, config: &ShortcutConfig) -> Result<(), AppError> {
    let capture_shortcut = parse_shortcut(&config.capture)?;
    let pin_clipboard_shortcut = parse_shortcut(&config.pin_clipboard)?;

    let cs = capture_shortcut;
    let ps = pin_clipboard_shortcut;

    app.plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |app, shortcut, event| {
                if event.state() != ShortcutState::Pressed {
                    return;
                }

                if shortcut == &cs {
                    handle_capture_hotkey(app);
                } else if shortcut == &ps {
                    handle_pin_clipboard_hotkey(app);
                }
            })
            .build(),
    )
    .map_err(|e| AppError::ConfigError(format!("注册全局快捷键插件失败: {}", e)))?;

    app.global_shortcut()
        .register(capture_shortcut)
        .map_err(|e| AppError::ConfigError(format!("注册截图快捷键失败: {}", e)))?;

    app.global_shortcut()
        .register(pin_clipboard_shortcut)
        .map_err(|e| AppError::ConfigError(format!("注册剪贴板贴图快捷键失败: {}", e)))?;

    Ok(())
}

/// 截屏流程：先创建蒙版窗口，再执行耗时截图
pub fn handle_capture_flow(app: &tauri::AppHandle) -> Result<(), AppError> {
    // Step 1: 获取显示器信息（快速操作）
    let monitor = app.primary_monitor()
        .ok()
        .flatten()
        .ok_or_else(|| AppError::ConfigError("获取主显示器信息失败".to_string()))?;
    let scale_factor = monitor.scale_factor();
    let monitor_x = (monitor.position().x as f64 * scale_factor).round() as i32;
    let monitor_y = (monitor.position().y as f64 * scale_factor).round() as i32;

    // Step 2: 立即创建蒙版窗口
    crate::window::create_overlay_window_lazy(app)?;
    log::info!("[HOTKEY] overlay 窗口创建成功（加载中...）");

    // Step 3: 执行耗时的全屏截图 + JPEG编码
    let (jpeg_base64, rgba_image) = {
        let state = app.state::<std::sync::Mutex<crate::capture::CaptureService>>();
        let locked = state.lock().map_err(|e| AppError::ConfigError(format!("锁定截图服务失败: {}", e)))?;
        locked.capture_fullscreen_with_cache(None)?
    };

    // Step 4: 存储截图数据到缓存，前端通过轮询获取
    {
        let store = app.state::<std::sync::Mutex<crate::window::CachedScreenStore>>();
        let mut store = store.lock().map_err(|e| AppError::ConfigError(format!("锁定缓存失败: {}", e)))?;
        store.screen = Some(crate::window::CachedScreen {
            image: rgba_image,
            monitor_x,
            monitor_y,
            scale_factor,
        });
        store.overlay_image = Some(crate::window::OverlayImageData {
            data: jpeg_base64,
            mime: "image/jpeg".to_string(),
        });
    }

    log::info!("[HOTKEY] 截图完成，图像数据已就绪");
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
        let image_data = match crate::clipboard::read_clipboard_image(app)? {
            Some(data) => data,
            None => {
                log::info!("[HOTKEY] 剪贴板无图像，跳过");
                return Ok(());
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
            let locked = state.lock().map_err(|e| AppError::ConfigError(format!("锁定截图服务失败: {}", e)))?;
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
