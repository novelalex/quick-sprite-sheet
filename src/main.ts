import { listen } from '@tauri-apps/api/event'
import {invoke} from "@tauri-apps/api/tauri";

listen('tauri://file-drop', event => {
    invoke('sprite_dropped', {payload: JSON.stringify(event.payload)})
})