<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import {ref, onMounted} from "vue";
import {invoke} from '@tauri-apps/api'
import TitleBar from "./components/TitleBar.vue";

const mode = ref('cpu');

function setMode(value: string) {
    console.log(value)
    mode.value = value;
}
async function callSysInfo() {
    const value = await invoke('sys_info');
    console.log(value);
}
onMounted(() => {
    document.body.setAttribute('arco-theme', 'dark');
    callSysInfo();
})

</script>

<template>
    <a-layout>
        <a-layout-header>
            <TitleBar :mode="mode" :setMode="setMode"/>
        </a-layout-header>
        <a-layout-content>
            {{ mode }}
        </a-layout-content>
    </a-layout>
</template>