<script setup lang="ts">
import { watch, ref, computed } from 'vue';
import { _Metadata } from './List.vue';
import { Root } from '../bindings/Root.ts';
import { Load_Info } from '../utils.ts';
import { invoke } from '@tauri-apps/api';
import CmdLoading from './CmdLoading.vue';
import { listen } from '@tauri-apps/api/event';

const emit = defineEmits(['refresh']);

let props = defineProps({
    name: String,
    installed_formulas: Array<String>,
    installed_casks: Array<String>,
    metadata: Array<_Metadata>
});
let deprecatedies = computed(() => props.metadata!.filter((p) => p.deprecated))

let info = ref<Root>();
let is_cask = ref(false);

let installed = computed(() => props.installed_formulas!.map((r) => r.split(" ")[0]).concat(props.installed_casks!.map((r) => r.split(" ")[0])).filter((r) => r !== ''));
let is_installed = ref(false);
let need_to_update = ref(false);

watch(props, async () => {
    if (props.name! !== '') {
        let model = { name: props.name!, data: info, isCask: is_cask };

        is_executing.value = true;
        executing_command.value = "info " + ((is_cask) ? '--cask' : '--formula') + ' ' + props.name;
        await Load_Info(model);
        executing_command.value = "";
        is_executing.value = false;

        is_installed.value = installed.value.filter((r) => r === props.name!).length > 0;
        need_to_update.value = is_installed.value && deprecatedies.value.filter((m) => m.data.name === props.name).length > 0;
    }
})

let model = { name: props.name!, data: info, isCask: is_cask };
await Load_Info(model);
is_installed.value = installed.value.filter((r) => r === props.name!).length > 0;
need_to_update.value = is_installed.value && deprecatedies.value.filter((m) => m.data.name === props.name).length > 0;

let install_command = (is_installed: boolean, is_cask: boolean) =>
    (((is_installed) ? 'un' : '') + 'install ' + ((is_cask) ? '--cask' : '--formula') + ' ' + info.value?.name);

let update_command = (is_cask: boolean) =>
    ('upgrade ' + ((is_cask) ? '--cask ' : '') + info.value?.name);

let is_executing = ref(false);
let executing_command = ref("");

async function execute() {
    await invoke('execute_command', { args: executing_command.value.split(" ") })
        .then((o) => console.log(o))
        .catch((e) => console.error(e));
    emit('refresh');
    is_executing.value = false;
    output.value = ""
}

let output = ref("");

listen('test', (event) => {
    if (typeof event.payload === 'string') {
        output.value += (event.payload + '\n')
    }
});

</script>

<template>
    <div v-if="is_executing">
        <CmdLoading :cmd="executing_command" :name="info?.name!" :output="output" />
    </div>
    <div v-else class="ml-4 mr-4 my-4 font-mono">
        <div>
            <span class="text-4xl font-bold">{{ info?.name }}</span>
            <span v-if="info?.name != info?.full_name" class="ml-2 text-xs font-bold italic text-stone-400">{{
                info?.full_name }}</span>
        </div>
        <p>
            <span class="text-stone-400">to <template v-if="is_installed">un</template>install: </span>
            <button @click="executing_command = install_command(is_installed, is_cask); is_executing = true; execute();"
                class="hover:animate-pulse text-sm font-bold bg-stone-400 text-gray-700 shadow-md border rounded border-stone-500 pr-0.5">
                <span class="ml-0.5 font-thin">$</span> brew <span class="italic"><template
                        v-if="is_installed">un</template>install <span v-if="is_cask">--cask</span></span>
                {{
                    info?.name }}
            </button>
            <span class="text-xs text-stone-400"> click to execute </span>
        </p>
        <p v-if="need_to_update">
            <span class="text-stone-400">to update: &nbsp;&nbsp;&nbsp;</span>
            <button @click="executing_command = update_command(is_cask); is_executing = true; execute();"
                class=" hover:animate-pulse text-sm font-bold bg-stone-400 text-gray-700 shadow-md border rounded border-stone-500 pr-0.5">
                <span class="ml-0.5 font-thin">$</span> brew <span class="italic">upgrade <span
                        v-if="is_cask">--cask</span></span>
                {{
                    info?.name }}
            </button>
            <span class="text-xs text-stone-400"> click to execute </span>
        </p>

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