<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import {ref, onMounted, onUnmounted} from "vue";
import {invoke} from "@tauri-apps/api";
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
        dataIndex: 'memoryStr',
        key: 'memoryStr',
    },
    {
        title: '读取字节',
        dataIndex: 'readBytes',
        key: 'readBytes',
    },
    {
        title: '写入字节',
        dataIndex: 'writtenBytes',
        key: 'writtenBytes',
    },
    {
        title: 'PID',
        dataIndex: 'pid',
        key: 'pid',
    },
]);
const tableData = ref([]);
const menuStyle = ref({
    display: "none",
    left: 0,
    top: 0,
});
const choosePid = ref(0);

async function callSysInfo() {
    const value = await invoke('sys_info');
    tableData.value = value;
    console.log(value);
}
async function callFindProcess(search){
    const value = await invoke('find_process', {search});
    tableData.value = value;
    console.log(value);
}
function onContextMenu(e){
    e.preventDefault();
    const tr = e.target.parentNode;
    const tds = tr.querySelectorAll("td");
    const target = tds[tds.length - 1];
    const pid = target.innerText;
    menuStyle.value = {
        display: "block",
        left: e.pageX + "px",
        top: e.pageY + "px",
    };
    choosePid.value = pid;
}
function onKill() {
    console.log(choosePid.value);
    invoke('kill_process', {pid: Number(choosePid.value)}).catch(e => {
        console.log(e);
    });
}
function resetMenu() {
    menuStyle.value = {
        display: "none",
        left: 0,
        top: 0,
    };
}
onMounted(() => {
    callSysInfo();
    document.querySelector("#list-table").addEventListener("contextmenu", onContextMenu);
    document.addEventListener("click", resetMenu);
})
onUnmounted(() => {
    document.querySelector("#list-table").removeEventListener("contextmenu", onContextMenu);
    document.removeEventListener("click", resetMenu);
})

</script>

<template>
    <TitleBar :callFindProcess="callFindProcess" />
    <div class="relative">
        <div class="flex-grow overflow-x-scroll">
            <a-table id="list-table" :dataSource="tableData" :columns="columns" :pagination="false"/>
        </div>
        <div class="menu" :style="menuStyle" @click="onKill">kill</div>
    </div>
</template>

<style scoped>
    .menu{
        @apply fixed w-[100px] h-[20px] bg-red-500;
    }
</style>