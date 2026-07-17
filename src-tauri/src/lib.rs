// std
use std::env ;
use std::sync::Mutex ;
use std::path::PathBuf ;

// tauri
use tauri::{
  AppHandle, 
  Manager, 
  Emitter, 
} ;

// sttk3
mod app_state ;
use app_state::{
  AppState, 
  get_state_files_to_open, 
  set_state_files_to_open, 
} ;
mod menu ;
use menu::create_menu ;
mod path ;
use path::get_extension ;

/// env::args()などからドロップされたファイルを取得する
///
/// ### Arguments
/// * `args` - 対象のイテレータ
/// 
/// ### Returns
/// PathBufの配列
///
#[cfg(target_os = "windows")]
fn collect_files<I>(args: I) -> Vec<PathBuf>
where
  I: IntoIterator<Item = String>,
{
  let res: Vec<PathBuf> = args.into_iter()
    .skip(1)
    .map(PathBuf::from)
    .collect()
  ;

  return res ;
}

/// ドロップされたファイルをフロントエンドの処理に送る
///
/// ### Arguments
/// * `app_handle` - tauri::AppHandle
/// * `files` - 処理対象のPathBufの配列
///
fn handle_files(app_handle: &AppHandle, files: &Vec<PathBuf>) -> anyhow::Result<()> {
  if files.is_empty() {return Ok(()) ;}

  let file_path: PathBuf = files.last().unwrap().clone() ;
  let extension: String = get_extension(&file_path).to_lowercase() ;

  if &extension == "txt" {
    let target_files: Vec<PathBuf> = vec![file_path.clone()] ;

    // stateを取得する。なければ作る
    let state = match app_handle.try_state::<Mutex<AppState>>() {
      Some(state) => state,
      None => {
        app_handle.manage(Mutex::new(AppState::default())) ;
        app_handle.state::<Mutex<AppState>>()
      }
    } ;

    // open対象のファイルをstateに保存しておく
    let _ = set_state_files_to_open(state, target_files) ;

    // フロントエンドのウインドウ'main'にイベント'ts_on_open_files'を送信する
    app_handle.emit_to("main", "ts_on_open_files", file_path)? ;
  }

  return Ok(()) ;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_single_instance::init(|app_handle, argv, _cwd| {
      // macOSでは使わないので警告を抑止する
      #[cfg(target_os = "macos")]
      let _ = (&app_handle, &argv) ;

      // Windowsですでにアプリが起動中にファイルを開く処理
      #[cfg(target_os = "windows")]
      {
        let files: Vec<PathBuf> = collect_files(argv) ;
        let _ = handle_files(app_handle, &files) ;
      }
    }))
    .invoke_handler(tauri::generate_handler![
      get_state_files_to_open, 
      set_state_files_to_open, 
    ])
    .setup(|app| {
      let app_handle: AppHandle = app.handle().clone() ;

      // stateを作る
      app.manage(Mutex::new(AppState::default())) ;

      // メニューを作る
      create_menu(&app_handle)? ;

      // ウインドウのタイトルを指定する
      let window_main: tauri::WebviewWindow = app.get_webview_window("main").unwrap() ;
      let package_info: &tauri::PackageInfo = app.package_info() ;
      window_main
        .set_title(&format!("{} {}", package_info.name, package_info.version))
        .expect("Failed to set window title")
      ;

      // Windowsでアプリ初回起動時にファイルを開く処理
      #[cfg(target_os = "windows")]
      {
        let files: Vec<PathBuf> = collect_files(env::args()) ;
        if !files.is_empty() {
          let _ = handle_files(&app_handle, &files) ;
        }
      }

      Ok(())
    })
    .build(tauri::generate_context!())
    .expect("error while running tauri application")
    .run(
      // macOSでファイルオープンイベントを処理する
      #[allow(unused_variables)]
      |app_handle, event| {
        #[cfg(target_os = "macos")]
        if let tauri::RunEvent::Opened { urls } = event {
          let files: Vec<PathBuf> = urls
            .into_iter()
            .filter_map(|url| url.to_file_path().ok())
            .collect()
          ;

          let _ = handle_files(app_handle, &files) ;
        }
      }, 
    )
  ;
}
