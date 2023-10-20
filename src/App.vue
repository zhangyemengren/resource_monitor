<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import {ref} from "vue";
import TitleBar from "./components/TitleBar.vue";

const mode = ref('cpu');

function setMode(value: string) {
    console.log(value)
    mode.value = value;
}

</script>

<template>
    <div>
        <TitleBar :mode="mode" :setMode="setMode"/>
        {{mode}}
    </div>
</template>