<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import {ref, onMounted} from "vue";
import {invoke} from '@tauri-apps/api'
import TitleBar from "./components/TitleBar.vue";

const mode = ref('cpu');
const columns = ref([
    {
        title: '进程名称',
        dataIndex: 'name',
        key: 'name',
    },
    {
        title: 'PID',
        dataIndex: 'pid',
        key: 'pid',
    },
]);
const tableData = ref([]);

function setMode(value: string) {
    console.log(value)
    mode.value = value;
}
async function callSysInfo() {
    const value = await invoke('sys_info');
    tableData.value = value;
    console.log(value);
}
onMounted(() => {
    callSysInfo();
})

</script>

<template>
    <div>
        <TitleBar :mode="mode" :setMode="setMode"/>
    </div>
    <div>
        {{ mode }}
    </div>
    <div>
        <a-table :dataSource="tableData" :columns="columns" />
    </div>
</template>