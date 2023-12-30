<script setup lang="ts">
import { ref, watch, computed } from "vue";
import { Metadata } from "../bindings/Metadata";

enum Type {
    Formula,
    Cask
}

export interface _Metadata {
    data: Metadata,
    installed: boolean,
    deprecated: boolean,
    type: Type
}

let page = ref(0);
let props = defineProps({
    filter: String,
    installed: Array<string>,
    datas: Array<_Metadata>
});

watch(props, async () => {
    page.value = 0;
})

let arr = computed(() => props.datas!.filter((d) => d.data.name?.startsWith(props.filter!)))

</script>

<template>
    <div class="flex flex-row justify-between mr-2">
        <h1 class="text-xl text-left ml-0.5 text-stone-400 font-mono my-2">All awaiable Homebrew formulae/casks</h1>
        <div class="flex flex-row justify-between gap-2">
            <button v-if="page != 0" @click="page--"><i class="fa-solid fa-arrow-left"></i></button>
            <button v-if="props && props.datas && arr.length > (page + 1) * 20" @click="page++"><i
                    class="fa-solid fa-arrow-right"></i></button>
        </div>
    </div>
    <div
        class="overflow-scroll leading-loose container m-auto grid grid-cols-1 md:grid-cols-2 lg:grid-cals-4 gap-4 font-mono">
        <template v-for=" metadata  in   arr.slice(page * 20, (page + 1) * 20)  ">
            <button @click="$emit('emit-show', metadata.data.name)"
                class="group rounded-md bg-stone-300 h-24 transition-all delay-200 ease-in-out hover:h-40">
                <div class="text-xs break-all text-left mt-1 mx-0.5 flex flex-row justify-between">
                    <span class="italic">
                        <span class="border-b-2 rounded border-gray-700 mr-2">{{ metadata.data.full_name }}</span>
                        <i v-if="metadata.installed"
                            :class="(!metadata.deprecated) ? 'fa-solid fa-check' : 'fa-regular fa-circle-up'"></i>
                        <span v-if="metadata.type == Type.Formula" class="font-bold text-stone-400 ml-0.5">
                            {{ (metadata.type == Type.Formula) ? '--formula' : '--cask' }}
                        </span>
                        <span v-if="metadata.type == Type.Cask" class="font-bold text-stone-400 ml-0.5">--cask</span>
                    </span>
                    <span class="italic text-stone-200 rounded-md bg-stone-400 py-0.5 px-0.5 font-light"
                        :class="(metadata.data.deprecated) ? 'line-through' : ''">v{{
                            metadata.data.versions?.stable
                        }}{{ (metadata.data.outdated) ? '!' : '' }}</span>
                </div>
                <p class="text-xs break-all italic mx-0.5 mt-0.5 text-gray-500 line-clamp-2 group-hover:line-clamp-none">
                    {{ metadata.data.desc }}</p>
                <div
                    class="my-2 mx-0.5 invisible transition-all ease-in delay-200 text-transparent font-mono text-xs group-hover:text-gray-500 group-hover:visible">
                    <p class="text-center">---</p>
                    <p v-if="metadata.data.license">License: {{ metadata.data.license }}</p>
                    <!-- <p class="text-center my-0.5"><button
                            class="border rounded-sm w-1/4 transition-all delay-75 hover:border-transparent hover:bg-stone-200 border-stone-500 bg-transparent text-gray-500 font-bold">{{
                                (metadata.deprecated) ? 'Update' : 'Install' }}</button>
                    </p> -->
                </div>
            </button>
        </template>
    </div>
</template>