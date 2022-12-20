import { invoke } from '@tauri-apps/api';

export {
    toggle_play
}

async function toggle_play() {
    invoke('panic_cmd', {}).then((response) => console.log(response));
}