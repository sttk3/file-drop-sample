use std::path::Path ;

/// ファイルパスから拡張子を取り出す
///
/// ### Arguments
/// * `filepath` - 対象のファイルパス
/// 
pub fn get_extension<P: AsRef<Path>>(filepath: P) -> String {
  let res = filepath.as_ref().extension()
    .map(|ext| ext.to_string_lossy().into_owned())
    .unwrap_or_else(|| "".to_string())
  ;

  return res ;
}
