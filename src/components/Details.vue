<script setup lang="ts">
import { watch, ref } from 'vue';
import { _Metadata } from './List.vue';
import { Root } from '../bindings/Root.ts';
import { Load_Info } from '../utils.ts';

let props = defineProps({
    name: String
});

let info = ref<Root>();

watch(props, async () => {
    if (props.name! !== '') {
        let model = { name: props.name!, data: info };
        await Load_Info(model);
    }
})

let model = { name: props.name!, data: info };
await Load_Info(model);

</script>

<template>
    <div class="ml-4 mr-4 my-4 font-mono">
        <div>
            <span class="text-4xl font-bold">{{ info?.name }}</span>
            <span v-if="info?.name != info?.full_name" class="ml-2 text-xs font-bold italic text-stone-400">{{
                info?.full_name }}</span>
        </div>
        <span class="text-stone-400">to install: </span>
        <span class="text-sm font-bold bg-stone-400 text-gray-700 shadow-md border rounded border-stone-500 pr-0.5">
            <span class="ml-0.5 font-thin">$</span> brew <span class="italic">install</span> {{ info?.name }}
        </span>
        <span class="text-stone-400 ml-1 mr-1">or</span>
        <button class="bg-stone-400 px-1.5 rounded text-sm italic font-light text-stone-300">Click</button>
        <p class="text-left text-stone-400 text-xl">-----</p>
        <p class="text-sm italic">{{ info?.desc }}</p>
        <p class="text-sm mt-1.5 leading-3">License: <span class="font-bold text-stone-400 align-middle inline-block">{{
            info?.license
        }}</span></p>
        <div class="flex flex-col">
            <p class="underline mt-2">Current versions</p>
            <div class="flex flex-col items-start w-1/2 text-sm">
                <button>stable: {{ info?.versions?.stable }}</button>
                <button v-if="info?.versions?.head">head: &nbsp;&nbsp;{{ info?.versions?.head }}</button>
            </div>
            <table class="text-left items-center text-sm bg-stone-400 rounded-md">
                <tr>
                    <th>
                        <p class="ml-2">Apple Silicon</p>
                    </th>
                    <template v-for="list in info?.bottle">
                        <th v-for="value in Object.keys(list.files!).filter((k) => k.startsWith('arm64')).map((v) => v.replace('arm64_', '').replace('_', ' '))"
                            class="flex flex-col ml-4 font-light">
                            <div class="flex flex-row justify-between">
                                <p>{{ value }}</p>
                                <p class="ml-2">✅</p>
                            </div>
                        </th>
                    </template>
                </tr>
                <tr class="border-t-2 border-stone-500">
                    <th>
                        <p class="ml-2">Intel</p>
                    </th>
                    <template v-for="list in info?.bottle">
                        <th v-for="value in Object.keys(list.files!).filter((k) => !k.startsWith('arm64')).map((v) => v.replace('_', ' '))"
                            class="flex flex-col ml-4 font-light">
                            <div class="flex flex-row justify-between">
                                <p>{{ value }}</p>
                                <p class="ml-2">✅</p>
                            </div>
                        </th>
                    </template>
                </tr>
            </table>
            <div class="flex flex-col text-sm">
                <p class="text-center text-lg">Analytics</p>
                <table>
                    <tr>
                        <th v-for="(_, key) in info?.analytics">{{ key }}</th>
                    </tr>
                    <tr>
                        <th v-for="analytic in info?.analytics">
                            <div class="grid grid-cols-1 grid-flow-row gap-2 font-light">
                                <div v-if="analytic['30d']">
                                    <span>30d: </span>
                                    <span>{{ Object.values(analytic['30d']!)[0] }}</span>
                                </div>
                                <div v-if="analytic['90d']">
                                    <span>90d: </span>
                                    <span>{{ Object.values(analytic['90d']!)[0] }}</span>
                                </div>
                                <div v-if="analytic['365d']">
                                    <span>365d: </span>
                                    <span>{{ Object.values(analytic['365d']!)[0] }}</span>
                                </div>
                            </div>
                        </th>
                    </tr>
                </table>
            </div>
        </div>
    </div>
</template>