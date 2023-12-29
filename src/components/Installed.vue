<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const formulas = ref([""]);
const casks = ref([""]);

const filter = defineProps(['filter']);

const emit = defineEmits<{
  loaded: [names: string[]]
}>();

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

function see_more(pkg: string) {
  console.log(pkg);
}

</script>


<template>
  <h2 class="text-lg text-right align-bottom border-b-2 border-stone-500">Formulas</h2>
  <ul>
    <template v-for="item in formulas">
      <li v-if="item.startsWith(filter.filter)">
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
  <h2 class="text-lg text-right align-bottom border-b-2 border-stone-500">Casks</h2>
  <ul>
    <template v-for="item in casks">
      <li v-if="item.startsWith(filter.filter)">
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