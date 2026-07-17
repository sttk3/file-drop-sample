// tauri
use tauri::menu::{MenuBuilder, SubmenuBuilder, PredefinedMenuItem} ;
use tauri::AppHandle ;

#[cfg(target_os = "macos")]
use tauri::menu::AboutMetadataBuilder ;

pub fn create_menu(app_handle: &AppHandle) -> anyhow::Result<()> {
  let menu ;

  #[cfg(target_os = "macos")]
  {
    let info = app_handle.package_info() ;
    let app_name: &String = &info.name ;

    // Apple
    let submenu_app = SubmenuBuilder::with_id(app_handle, "app", &app_name)
      .items(&[
        &PredefinedMenuItem::about(
          app_handle,
          None,
          Some(AboutMetadataBuilder::default().build()),
        )?,
        &PredefinedMenuItem::separator(app_handle)?,
        &PredefinedMenuItem::services(app_handle, None)?,
        &PredefinedMenuItem::separator(app_handle)?,
        &PredefinedMenuItem::hide(app_handle, None)?,
        &PredefinedMenuItem::hide_others(app_handle, None)?,
        &PredefinedMenuItem::separator(app_handle)?,
        &PredefinedMenuItem::quit(app_handle, None)?,
    ]).build()? ;

    // Edit
    let submenu_edit = SubmenuBuilder::with_id(app_handle, "edit", "Edit")
      .items(&[
        &PredefinedMenuItem::undo(app_handle, None)?,
        &PredefinedMenuItem::redo(app_handle, None)?,
        &PredefinedMenuItem::separator(app_handle)?,
        &PredefinedMenuItem::cut(app_handle, None)?,
        &PredefinedMenuItem::copy(app_handle, None)?,
        &PredefinedMenuItem::paste(app_handle, None)?,
        &PredefinedMenuItem::select_all(app_handle, None)?,
    ]).build()? ;

    // Window
    let submenu_window = SubmenuBuilder::with_id(app_handle, "window", "Window")
      .items(&[
        &PredefinedMenuItem::separator(app_handle)?,
        &PredefinedMenuItem::minimize(app_handle, None)?,
        &PredefinedMenuItem::maximize(app_handle, None)?,
        &PredefinedMenuItem::separator(app_handle)?,
        &PredefinedMenuItem::close_window(app_handle, None)?,
    ]).build()? ;

    menu = MenuBuilder::new(app_handle)
      .items(&[&submenu_app, &submenu_edit, &submenu_window])
      .build()? ;
  }

  #[cfg(not(target_os = "macos"))]
  {
    // Window
    let submenu_window = SubmenuBuilder::with_id(app_handle, "window", "Window")
      .items(&[&PredefinedMenuItem::close_window(app_handle, None)?])
      .build()? ;

    menu = MenuBuilder::new(app_handle)
      .items(&[&submenu_window])
      .build()? ;
  }

  // 実際にメニューを登録する
  app_handle.set_menu(menu)? ;

  return Ok(()) ;
}
