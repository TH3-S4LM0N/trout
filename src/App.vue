<template>
    <div id="container">
        <el-menu
            default-active="/home"
            router
            mode="horizontal"
            id="tbar"
            background-color="#1e2127"
            text-color="#abb2bf"
            active-text-color="#40a4ff"
            :ellipsis="false"
        >
            <el-menu-item index="/home">Home</el-menu-item>
            <div id="flex-grow" />
            <el-button circle>a</el-button>
            <div id="flex-grow" />
            <el-sub-menu id="submenu">
                <template #title>Libraries</template>
                <el-menu-item index="/libraries/spotify">Spotify</el-menu-item>
            </el-sub-menu>
        </el-menu>
    </div>
</template>

<script setup lang="ts">
    import { init } from './commands/utils';

    let init_vars = init();
</script>

<style>
#tbar {
    border: none !important;
}

.container {
    height: 100%;
    width: 100%;
}

#flex-grow {
    flex-grow: 1;
}
</style>

