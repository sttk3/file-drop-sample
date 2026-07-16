/**
  * @file Rust側が保持するstateのget/set
  * @author sttk3.com
  * @copyright © 2026 sttk3.com
*/

// tauri
import { invoke } from '@tauri-apps/api/core' ;

/**
  * アプリstateのfiles_to_openを返す
  * @returns ファイルパス一覧。空のときは[]。エラーは起こり得ない
*/
export const getStateFilesToOpen = async (): Promise<Array<string>> => {
  let res: Array<string> = [] ;

  try {
    const files: Array<string> = await invoke<Array<string>>('get_state_files_to_open') ;
    res = files ;
  } catch(e) {
    console.error(e) ;
  }
  
  return res ;
} ;

/**
  * アプリstateのfiles_to_openに値をセットする
  * @param files - セットするファイルパス一覧
*/
export const setStateFilesToOpen = async (files: Array<string>): Promise<void> => {
  await invoke('set_state_files_to_open', {files}) ;
} ;
