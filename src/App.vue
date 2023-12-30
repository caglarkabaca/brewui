<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Installed from "./components/Installed.vue";
import List from "./components/List.vue";
import Details from "./components/Details.vue";

import { ref } from "vue";
import { _Metadata } from "./components/List.vue";
import { Load_Installed, Load_Metadata } from "./utils";

let display = ref("");

let filter = ref("");
let installed = ref<string[]>([]);
let metadatas = ref<Array<_Metadata>>([]);

let installed_formulas = ref([""]);
let installed_casks = ref([""]);

await Load_Installed(installed_formulas, installed_casks);
await Load_Metadata(installed_formulas.value, installed_casks.value, metadatas.value);

</script>

<template>
  <div class="flex flex-row h-screen overflow-hidden">
    <div class="basis-1/4 lg:basis-1/5 py-2 mx-2 pr-2 border-r-4 border-stone-300 font-mono">
      <div class="flex pb-2">
        <button class="flex-none" @click="display = ''"><i class="fa-solid fa-bars"></i></button>
        <input type="text" v-model="filter" class="mx-auto w-10/12 rounded-md">
      </div>
      <div class="overflow-scroll h-screen leading-loose">
        <h1 class="text-center text-stone-400">Installed</h1>
        <Installed :filter="filter" :metadatas="metadatas" :formulas="installed_formulas" :casks="installed_casks"
          @emit-show="(s) => display = s" />
      </div>
    </div>
    <div class="basis-3/4 lg:basis-4/5">
      <List v-if="display === ''" :filter="filter" :datas="metadatas" :installed="installed"
        @emit-show="(s) => display = s" />
      <div v-else class="flex h-screen justify-center items-center">
        <div class="mx-auto h-5/6 w-4/5 bg-stone-300 shadow-2xl overflow-y-auto">
          <Suspense>
            <Details :name="display" :installed_formulas="installed_formulas" :installed_casks="installed_casks"
              :metadata="metadatas" />
            <template #fallback>
              <button type="button" disabled>
                <svg class="animate-spin h-15 w-15 text-stone-300" xmlns="http://www.w3.org/2000/svg" fill="none"
                  viewBox="0 0 24 24">
                  <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4">
                  </circle>
                  <path class="opacity-75" fill="currentColor"
                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
                  </path>
                </svg>
              </button>
            </template>
          </Suspense>
        </div>
      </div>
    </div>
  </div>
</template>
