<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import {ref, onMounted} from "vue";
import TitleBar from "./components/TitleBar.vue";

const mode = ref('cpu');

function setMode(value: string) {
    console.log(value)
    mode.value = value;
}
onMounted(() => {
    document.body.setAttribute('arco-theme', 'dark')
})

</script>

<template>
    <a-config-provider theme="dark">
    <a-layout>
    <a-layout-header>
        <TitleBar :mode="mode" :setMode="setMode"/>
    </a-layout-header>
    <a-layout-content>
        {{ mode }}
    </a-layout-content>
    </a-layout>
    </a-config-provider>
</template>