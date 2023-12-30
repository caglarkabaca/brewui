<script setup lang="ts">

import App from './App.vue';

</script>


<template>
    <Suspense>
        <App />
        <template #fallback>
            <div class="flex h-screen justify-center items-center">
                <div class="mx-auto items-center">
                    <button type="button" disabled>
                        <svg class="block w-full mx-auto animate-spin h-10 text-stone-300"
                            xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4">
                            </circle>
                            <path class="opacity-75" fill="currentColor"
                                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
                            </path>
                        </svg>
                    </button>
                    <h1 class="text-stone-400 text-2xl text-center">Loading..</h1>
                    <h1 class="text-stone-300 text-lg text-center">thanks for using my app</h1>
                    <h1 class="text-stone-300 text-md italic text-center">@caglarkabaca</h1>
                </div>
            </div>
        </template>
    </Suspense>
</template>