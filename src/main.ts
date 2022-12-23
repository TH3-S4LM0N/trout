import { createApp } from 'vue';
import App from './App.vue';
import ElementPlus from 'element-plus';
import * as ElementPlusIconsVue from '@element-plus/icons-vue';
import { createRouter, createWebHashHistory } from "vue-router";
import HomeView from './views/HomeView.vue';
import SpotifyView from './views/libraries/SpotifyView.vue';
import DebugView from  './views/DebugView.vue';

const app = createApp(App);

import 'element-plus/theme-chalk/index.css'

app.use(ElementPlus);

for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component);
}

const routes = [
    { path: '/home', name: 'Home', component: async () => HomeView },
    { path: '/debug', name: 'Debug', component: async () => DebugView },
    { path: '/libraries/spotify', name: 'Spotify', component: async () => SpotifyView }
];

export const router = createRouter({
    history: createWebHashHistory(),
    routes
});

app.use(router);

app.mount('#app');