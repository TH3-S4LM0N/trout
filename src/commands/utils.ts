import { invoke } from "@tauri-apps/api";

export {
    init
}

function init() {
    let init;
    invoke('init', {}).then((response) => init = response);
    return init;
}