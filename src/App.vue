<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Installed from "./components/Installed.vue";
import List from "./components/List.vue";
import Details from "./components/Details.vue";
import FakeApp from "./FakeApp.vue";

import { ref } from "vue";
import { _Metadata } from "./components/List.vue";
import { Load_Installed, Load_Metadata } from "./utils";

let display = ref("");

let filter = ref("");
let installed = ref<string[]>([]);
let metadatas = ref<Array<_Metadata>>([]);

let installed_formulas = ref([""]);
let installed_casks = ref([""]);
let reloading = ref(false);

await Load_Installed(installed_formulas, installed_casks);
await Load_Metadata(installed_formulas.value, installed_casks.value, metadatas.value);

async function Reload_Installed() {
  reloading.value = true;
  installed_formulas.value = [""];
  installed_casks.value = [""];
  await Load_Installed(installed_formulas, installed_casks);
  metadatas.value.forEach((metadata) => {
    let _installed = installed_formulas.value.concat(installed_casks.value).filter((p) => p.split(" ")[0] == metadata.data.name!).length > 0;
    let _deprecated = installed_formulas.value.concat(installed_casks.value).filter((p) => installed && p.split(" ")[1] === metadata.data.versions!.stable).length > 0;
    metadata.installed = _installed;
    metadata.deprecated = !_deprecated;
  })
  reloading.value = false;
}

</script>

<template>
  <template v-if="reloading">
    <FakeApp />
  </template>
  <template v-else>
    <div class="flex flex-row h-screen overflow-hidden">
      <div class="basis-1/4 lg:basis-1/5 py-2 mx-2 pr-2 border-r-4 border-stone-300 font-mono">
        <div class="flex pb-2">
          <button class="flex-none" @click="display = ''; filter = ''"><i class="fa-solid fa-bars"></i></button>
          <input type="text" v-model="filter" class="mx-auto w-10/12 rounded-md">
        </div>
        <div class="overflow-scroll h-screen leading-loose">
          <h1 class="text-center text-stone-400">Installed</h1>
          <Installed :filter="filter" :metadatas="metadatas" :formulas="installed_formulas" :casks="installed_casks"
            @emit-show="(s) => display = s" />
        </div>
      </div>
      <div class="basis-3/4 lg:basis-4/5 overflow-auto leading-loose h-screen mr-2 root">
        <List v-if="display === ''" :filter="filter" :datas="metadatas" :installed="installed"
          @emit-show="(s) => display = s" class="" />
        <div v-else class="flex h-screen justify-center items-center">
          <Suspense>
            <div class="mx-auto h-5/6 w-4/5 bg-stone-300 shadow-2xl overflow-y-auto">
              <Details :name="display" :installed_formulas="installed_formulas" :installed_casks="installed_casks"
                :metadata="metadatas" @refresh="Reload_Installed();" />
            </div>
            <template #fallback>
              <div class="mx-auto items-center">
                <button class="mx-auto">
                  <svg class="block w-full mx-auto animate-spin h-10 text-stone-300" xmlns="http://www.w3.org/2000/svg"
                    fill="none" viewBox="0 0 24 24">
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4">
                    </circle>
                    <path class="opacity-75" fill="currentColor"
                      d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
                    </path>
                  </svg>
                </button>
                <h1 class="text-stone-400 text-2xl text-center">Loading..</h1>
                <h1 class="text-stone-300 text-md italic text-center">@caglarkabaca</h1>
              </div>
            </template>
          </Suspense>
        </div>
      </div>
    </div>
  </template>
</template>

<style>
.root {
  background-image: url('./brew.svg');
  background-repeat: no-repeat;
  background-position: center center;
  background-size: 90% 40%;
}
</style>