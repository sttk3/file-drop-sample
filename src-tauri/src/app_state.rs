// std
use std::sync::Mutex ;
use std::result::Result ;
use std::path::PathBuf ;

// tauri
use tauri::State ;

// アプリの持つ共通state
#[derive(Default)]
pub struct AppState {
  // ファイルがドロップされたときにそのパスの一覧を保存する項目
  pub files_to_open: Vec<PathBuf>, 
}

/// アプリstateのfiles_to_openを返す
///
/// ### Arguments
/// * `state` - tauri::State
/// 
/// ### Returns
/// ファイルパスの一覧
///
#[tauri::command(rename_all = "snake_case")]
pub fn get_state_files_to_open(state: State<'_, Mutex<AppState>>) -> Result<Vec<PathBuf>, String> {
  let state = state.lock().map_err(|e| e.to_string())? ;

  return Ok(state.files_to_open.clone()) ;
}

/// アプリstateのfiles_to_openに値をセットする
///
/// ### Arguments
/// * `state` - tauri::State
/// * `files` - セットするファイルパスの一覧
///
#[tauri::command(rename_all = "snake_case")]
pub fn set_state_files_to_open(state: State<'_, Mutex<AppState>>, files: Vec<PathBuf>) -> Result<(), String> {
  let mut state = state.lock().map_err(|e| e.to_string())? ;
  state.files_to_open = files ;

  return Ok(()) ;
}
