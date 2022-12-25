import { invoke } from '@tauri-apps/api';

export {

}

function spotify_passwd_login(username: string, passwd: string) {
    let returnv;
    invoke('spotify_passwd_login', { username, passwd }).then((response) => returnv = response);
    return returnv;
}