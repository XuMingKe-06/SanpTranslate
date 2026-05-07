use crate::config::ShortcutConfig;
use crate::error::AppError;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use tauri::Manager;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::AppHandle;

// 创建系统托盘图标及菜单
pub fn create_tray(app: &AppHandle, shortcuts: &ShortcutConfig) -> Result<(), AppError> {
    let capture_item = MenuItem::with_id(
        app,
        "capture",
        format!("框选截图翻译  {}", shortcuts.capture),
        true,
        None::<&str>,
    )?;
    let pin_clipboard_item = MenuItem::with_id(
        app,
        "pin_clipboard",
        format!("从剪贴板贴图  {}", shortcuts.pin_clipboard),
        true,
        None::<&str>,
    )?;
    let separator1 = PredefinedMenuItem::separator(app)?;
    let history_item = MenuItem::with_id(app, "history", "截图与翻译历史", true, None::<&str>)?;
    let separator2 = PredefinedMenuItem::separator(app)?;
    let settings_item = MenuItem::with_id(app, "settings", "设置", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[
            &capture_item,
            &pin_clipboard_item,
            &separator1,
            &history_item,
            &separator2,
            &settings_item,
            &quit_item,
        ],
    )?;

    // 加载默认托盘图标
    let icon = tauri::image::Image::from_bytes(include_bytes!("../../icons/icon.png"))?;

    TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "capture" => {
                // 先创建蒙版窗口，再执行截图
                match crate::hotkey::handle_capture_flow(app) {
                    Ok(_) => {}
                    Err(e) => log::error!("截图失败: {}", e),
                }
            }
            "pin_clipboard" => {
                // 从剪贴板读取图像并创建贴图窗口
                match (|| -> Result<(), AppError> {
                    let image_data = match crate::clipboard::read_clipboard_image(app)? {
                        Some(data) => data,
                        None => return Ok(()),
                    };

                    // 解码图像获取宽高
                    let bytes = STANDARD.decode(&image_data)
                        .map_err(|e| AppError::ClipboardError(format!("Base64 解码失败: {}", e)))?;
                    let img = image::load_from_memory(&bytes)
                        .map_err(|e| AppError::ClipboardError(format!("图像解码失败: {}", e)))?;
                    let (img_w, img_h) = (img.width(), img.height());

                    // 从缓存的截图服务获取显示器信息
                    let (mon_x, mon_y, mon_w, mon_h) = {
                        let state = app.state::<std::sync::Mutex<crate::capture::CaptureService>>();
                        let locked = state.lock().map_err(|e| AppError::ConfigError(format!("锁定截图服务失败: {}", e)))?;
                        locked.get_primary_monitor_info()?
                    };

                    // 贴图窗口定位到主显示器中央
                    let center_x = mon_x + ((mon_w - img_w) as i32) / 2;
                    let center_y = mon_y + ((mon_h - img_h) as i32) / 2;

                    crate::window::create_pin_window(app, &image_data, center_x, center_y, img_w, img_h)?;
                    Ok(())
                })() {
                    Ok(_) => {}
                    Err(e) => log::error!("剪贴板贴图失败: {}", e),
                }
            }
            "history" => {
                if let Err(e) = crate::window::create_history_window(app) {
                    log::error!("创建历史窗口失败: {}", e);
                }
            }
            "settings" => {
                if let Err(e) = crate::window::create_settings_window(app) {
                    log::error!("创建设置窗口失败: {}", e);
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}
