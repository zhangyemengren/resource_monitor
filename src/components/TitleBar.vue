<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
async function callRust() {
    const response = await invoke('test_cpu');
    console.log(response);
}
</script>

<template>
    <div :class="['text-red-500', 'flex']">
        <ul :class="['flex']">
            <li @click="callRust">CPU</li>
            <li>内存</li>
            <li>能耗</li>
            <li>磁盘</li>
            <li>网路</li>
        </ul>
    </div>
</template>

<style scoped>

</style>