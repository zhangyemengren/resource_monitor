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
        <span class="isolate inline-flex rounded-md shadow-sm h-[30px]">
            <button
                class="btn-l"
                type="button"
                @click="setMode('cpu')"
            >
                CPU
            </button>
            <button
                class="btn-m"
                type="button"
                @click="setMode('mem')"
            >
                内存
            </button>
            <button
                class="btn-m"
                type="button"
                @click="setMode('energy')"
            >
              能耗
            </button>
            <button
               class="btn-m"
               type="button"
               @click="setMode('disk')"
            >
               磁盘
            </button>
            <button
                class="btn-r"
                type="button"
                @click="setMode('net')"
            >
                网络
            </button>
        </span>
        <div>
            {{ cpuInfo }}
        </div>
    </div>
</template>

<style scoped>
.btn-l {
    @apply relative inline-flex items-center rounded-l-md bg-slate-500 px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-slate-300 focus:z-10;
}

.btn-m {
    @apply relative inline-flex items-center -ml-px bg-slate-500 px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-slate-300 focus:z-10;
}

.btn-r {
    @apply relative inline-flex items-center -ml-px rounded-r-md bg-slate-500 px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-slate-300 focus:z-10;
}
</style>