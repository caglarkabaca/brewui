<script setup lang="ts">
import { computed } from "vue";
import { _Metadata } from "./List.vue";

const props = defineProps({
  filter: String,
  metadatas: Array<_Metadata>,
  formulas: Array<String>,
  casks: Array<String>
});

let deprecatedies = computed(() => props.metadatas!.filter((p) => p.deprecated))
</script>


<template>
  <h2 class="text-lg text-right align-bottom border-b-2 border-stone-500">Formulas</h2>
  <ul>
    <template v-for="item in   formulas  ">
      <li v-if="item != '' && item.startsWith(props.filter!)">
        <button class=" flex flex-row justify-between" @click="$emit('emit-show', item.split(' ')[0])">
          <div class="text-sm text-left w-max transition-all delay-100 hover:text-lg">
            <span class="mb-0 pb-0">{{ item.split(" ")[0] }}</span><i
              v-if="deprecatedies.filter((d) => d.data.name! === item.split(' ')[0]).length > 0"
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
    <template v-for="item in   casks  ">
      <li v-if="item != '' && item.startsWith(props.filter!)">
        <button class=" flex flex-row justify-between" @click="$emit('emit-show', item.split(' ')[0])">
          <div class=" text-sm text-left w-max transition-all delay-100 hover:text-lg">
            <span class="mb-0 pb-0">{{ item.split(" ")[0] }}</span><i
              v-if="deprecatedies.filter((d) => d.data.name! === item.split(' ')[0]).length > 0"
              class="fa-regular fa-circle-up ml-0.5"></i>
            <div class="text-[10px] mt-0 pt-0 text-left text-stone-500">{{
              item.split(" ")[1] }}</div>
          </div>
        </button>
      </li>
    </template>
  </ul>
</template>