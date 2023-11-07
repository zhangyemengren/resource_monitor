<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import {ref, onMounted} from "vue";
import {invoke} from '@tauri-apps/api'
import TitleBar from "./components/TitleBar.vue";

const columns = ref([
    {
        title: '进程名称',
        dataIndex: 'name',
        key: 'name',
    },
    {
        title: '%CPU',
        dataIndex: 'cpuUsage',
        key: 'cpuUsage',
    },
    {
        title: '运行时间',
        dataIndex: 'runTimeStr',
        key: 'runTimeStr',
    },
    {
        title: '用户',
        dataIndex: 'userName',
        key: 'userName',
    },
    {
        title: '内存',
        dataIndex: 'memory',
        key: 'memory',
    },
    {
        title: 'PID',
        dataIndex: 'pid',
        key: 'pid',
    },
]);
const tableData = ref([]);

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
    <TitleBar />
    <div>
        {{ mode }}
        <button @click="callSysInfo">手动刷新</button>
    </div>
    <div class="flex-grow overflow-x-scroll">
        <a-table :dataSource="tableData" :columns="columns" :pagination="false"/>
    </div>
</template>