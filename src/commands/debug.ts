import { invoke } from '@tauri-apps/api';

export {
    print,
    test_customs
}

async function print() {
    invoke('print_test', {}).then((response) => console.log(response));
}

async function test_customs() {
    let thisaa;
    invoke('custom_cmd', {}).then((response) => thisaa = response);
    console.log(thisaa);
}

