// preact
import { useEffect } from 'preact/hooks' ;

// tauri
import {
  getCurrentWebviewWindow, 
  WebviewWindow, 
} from '@tauri-apps/api/webviewWindow' ;
import { message } from '@tauri-apps/plugin-dialog' ;
import { type UnlistenFn } from '@tauri-apps/api/event' ;

// sttk3
import {
  getStateFilesToOpen, 
  setStateFilesToOpen, 
} from './appState' ;
import './App.css' ;

/**
  * ダイアログでファイル名を表示するサンプルfunction
  * @param pathText - ドロップしたファイルのパス
*/
const showDialog = async (pathText: string) => {
  const filename = pathText.replace(/^.+\//, '') ;
  await message(filename) ;
} ;

export const App = () => {
  // マウント時実行する
  useEffect(() => {
    // このウインドウ'main'を取得する
    const windowMain: WebviewWindow = getCurrentWebviewWindow() ;

    let unlistenList: Array<UnlistenFn> = [] ;
    (async () => {
      // イベント'ts_on_open_files'に応じた処理を登録する
      unlistenList.push(
        await windowMain.listen<string>('ts_on_open_files', async (event) => {
          await showDialog(event.payload) ;
        })
      ) ;

      /*
         アプリ初回起動時用。
         ファイルドロップ時に記録したファイル一覧を取得して、中身があれば処理を実行する
      */
      const filesToOpen: Array<string> = await getStateFilesToOpen() ;
      if(filesToOpen.length) {
        // もしfilesToOpenの中身が存在したら変数に記録して、stateは空にする
        const targetPath: string = filesToOpen[0] ;
        await setStateFilesToOpen([]) ;

        // イベント'ts_on_open_files'のときと同じ処理を実行する
        await showDialog(targetPath) ;
      }
    })() ;
    
    // アンマウント時イベントリスナーを削除する
    return () => {
      for(let unlisten of unlistenList) {
        if(unlisten) {unlisten() ;}
      }
    } ;
  }, []) ;

  // ウインドウUI。今回は特別な内容はない
  return (
    <div
      class='group-panel'
    >
      <p class='description'>アプリアイコンにtxtファイルをドロップしてください。反応します。</p>
      <p class='description'>※このウインドウは反応しません。</p>
    </div>
  ) ;
} ;
