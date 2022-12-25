import { invoke } from '@tauri-apps/api';

export {
    print,
}

function print() {
    invoke('print_test', {}).then((response) => console.log(response));
}
