<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Installed from "./components/Installed.vue";
import List from "./components/List.vue";

import { ref } from "vue";
import { _Metadata } from "./components/List.vue";

let filter = ref("");
let installed = ref<string[]>([]);
let metadatas = ref<Array<_Metadata>>([]);

</script>

<template>
  <div class="flex flex-row h-screen overflow-hidden">
    <div class="basis-1/4 lg:basis-1/5 py-2 mx-2 pr-2 border-r-4 border-stone-300 font-mono">
      <div class="flex pb-2">
        <button class="flex-none"><i class="fa-solid fa-bars"></i></button>
        <input type="text" v-model="filter" class="mx-auto w-10/12 rounded-md">
      </div>
      <div class="overflow-scroll h-screen leading-loose">
        <h1 class="text-center text-stone-400">Installed</h1>
        <Suspense>
          <Installed :filter="filter" :metadatas="metadatas" @loaded="(r) => r.forEach((x) => installed.push(x))" />
          <template #fallback>
            <h2>Fetching..</h2>
          </template>
        </Suspense>
      </div>
    </div>
    <div class="basis-3/4 lg:basis-4/5">
      <Suspense>
        <List :filter="filter" :installed="installed" @loaded="((datas: _Metadata[]) => metadatas = datas)" />
        <template #fallback>
          <h2>Fetching..</h2>
        </template>
      </Suspense>
    </div>
  </div>
</template>
