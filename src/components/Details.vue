<script setup lang="ts">
import { watch } from 'vue';
import { _Metadata } from './List.vue';
import { invoke } from '@tauri-apps/api';

let props = defineProps({
    name: String
});

watch(props, async () => {
    if (props.name! !== '') {
        await invoke("get_info_formula", { name: props.name! }).then((e) => console.log(e)).catch((e) => console.error(e));
    }
})

await invoke("get_info_formula", { name: props.name! }).then((e) => console.log(e)).catch((e) => console.error(e));

</script>

<template>
    {{ props.name! }}
</template>