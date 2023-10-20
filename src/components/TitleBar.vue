<script lang="ts" setup>
import {invoke} from '@tauri-apps/api'
import {ref} from "vue";

const cpuInfo = ref('');

async function callRust() {
    cpuInfo.value = await invoke('test_cpu');
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
                class="relative inline-flex items-center rounded-l-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
                type="button"
                @click="callRust"
            >CPU</button>
            <button
                class="relative -ml-px inline-flex items-center bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
                type="button">内存</button>
  <button
      class="relative -ml-px inline-flex items-center bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
      type="button">能耗</button>
            <button
                class="relative -ml-px inline-flex items-center bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
                type="button">磁盘</button>
  <button
      class="relative -ml-px inline-flex items-center rounded-r-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
      type="button">网络</button>

</span>
        <div>
            {{ cpuInfo }}
        </div>
    </div>
</template>

<style scoped>

</style>