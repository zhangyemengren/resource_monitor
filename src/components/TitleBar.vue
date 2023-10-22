<script lang="ts" setup>
import {invoke} from '@tauri-apps/api'
import {ref} from "vue";
const props = defineProps({
    mode: {
        type: String,
        default: 'cpu'
    },
    setMode: {
        type: Function,
        default: () => {}
    }
})
const cpuInfo = ref('');

async function callRust() {
    cpuInfo.value = await invoke('test_cpu');
}

function setMode(value: string) {
    props.setMode(value);
}

</script>

<template>
    <div class="flex pl-[100px] bg-slate-700 h-[50px] items-center">
        <div>
            <p class="text-sm">活动监视器</p>
            <p class="text-xs">所有进程</p>
        </div>

        <a-button-group type="primary">
            <a-button @click="setMode('cpu')"> CPU </a-button>
            <a-button @click="setMode('mem')"> 内存 </a-button>
            <a-button @click="setMode('energy')"> 能耗 </a-button>
            <a-button @click="setMode('disk')"> 磁盘 </a-button>
            <a-button @click="setMode('net')"> 网络 </a-button>
        </a-button-group>
        <div>
            {{ cpuInfo }}
        </div>
    </div>
</template>

<style scoped>

</style>