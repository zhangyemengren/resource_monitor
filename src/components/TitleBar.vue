<script lang="ts" setup>
    import {ref, watch} from "vue";
    import {debounce} from "../utils.ts";
    const props = defineProps(['callFindProcess'])
    const search = ref("");

    const onSearch = debounce((value) => {
        console.log(value);
        props.callFindProcess(value);
    }, 200);

    watch(search, onSearch)
</script>

<template>
    <div class="flex pl-[100px] bg-slate-700 h-[50px] items-center" data-tauri-drag-region>
        <div>
            <p class="text-sm">活动监视器</p>
            <p class="text-xs">所有进程</p>
        </div>
        <div>
            <input v-model="search" placeholder="搜索">
        </div>
    </div>
</template>

<style scoped>

</style>