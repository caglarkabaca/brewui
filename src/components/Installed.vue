<script setup lang="ts">
import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import { _Metadata } from "./List.vue";

const formulas = ref([""]);
const casks = ref([""]);

let deprecatedies = ref<(string | null)[]>([]);

const filter = defineProps({
  filter: String,
  metadatas: Array<_Metadata>
});
let metadatas_count = ref(0);

const emit = defineEmits<{
  loaded: [names: string[]]
}>();

function see_more(pkg: string) {
  console.log(pkg);
}

let load_all = async () => {
  interface Response {
    name: string[];
    count: number;
  }
  let load_formulas = async () => {
    await invoke<Response>("get_installed_formulas", {}).then((r) => {
      r.name.forEach(x => {
        formulas.value.push(x);
      });
    }).catch((e) => console.error(e));
  }
  let load_casks = async () => {
    await invoke<Response>("get_installed_casks", {}).then((r) => {
      r.name.forEach(x => {
        casks.value.push(x);
      });
    }).catch((e) => console.error(e));
  }
  await load_formulas();
  await load_casks();
}
await load_all();
emit('loaded', formulas.value.concat(casks.value));

watch(filter, async () => {
  if (filter.metadatas != null && metadatas_count.value != filter.metadatas.length) {
    deprecatedies.value = filter.metadatas?.filter((m) => m.deprecated).map((m) => m.data.name).filter((n) => n != null);
    metadatas_count.value = filter.metadatas.length;
    console.log("loaded count:", metadatas_count);
  }
})

</script>


<template>
  <h2 class="text-lg text-right align-bottom border-b-2 border-stone-500">Formulas</h2>
  <ul>
    <template v-for="item in   formulas  ">
      <li v-if="item != '' && item.startsWith((filter.filter != null) ? (filter.filter) : '')">
        <button class=" flex flex-row justify-between" @click="see_more(item.split(' ')[0])">
          <div class="text-sm text-left w-max transition-all delay-100 hover:text-lg">
            <span class="mb-0 pb-0">{{ item.split(" ")[0] }}</span><i
              v-if="deprecatedies.filter((d) => d === item.split(' ')[0]).length > 0"
              class="fa-regular fa-circle-up ml-0.5"></i>
            <div class="text-[10px] mt-0 pt-0 text-left text-stone-500">{{
              item.split(" ")[1] }}</div>
          </div>
        </button>
      </li>
    </template>
  </ul>
  <h2 class="text-lg text-right align-bottom border-b-2 border-stone-500">Casks</h2>
  <ul>
    <template v-for="  item   in   casks  ">
      <li v-if="item != '' && item.startsWith((filter.filter != null) ? (filter.filter) : '')">
        <button class=" flex flex-row justify-between" @click="see_more(item.split(' ')[0])">
          <div class="text-sm text-left w-max transition-all delay-100 hover:text-lg">
            <p class="mb-0 pb-0">{{ item.split(" ")[0] }}</p>
            <div class="text-[10px] mt-0 pt-0 text-left text-stone-500">{{
              item.split(" ")[1] }}</div>
          </div>
        </button>
      </li>
    </template>
  </ul>
</template>