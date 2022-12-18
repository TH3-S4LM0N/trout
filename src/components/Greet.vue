<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greet_msg = ref("");
const name = ref("");

async function greet() {
    greet_msg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
    <div class="card">
        <input id="greet-input" v-model="name" placeholder="Enter a name" />
        <button type="button" @click="greet()">Greet</button>
    </div>

    <p>{{ greet_msg }}</p>
</template>