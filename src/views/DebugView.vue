<template>
    <div id="container">
        <el-button id="testbutton" @click="test_customs">debug button</el-button>
    </div>
</template>

<script setup lang="ts">
import { test_customs } from '../commands/debug';
</script>