<script setup lang="ts">

let prop = defineProps({
    cmd: String,
    name: String,
})

const _args = prop.cmd?.split(" ");

</script>

<template>
    <div class="flex pt-[45%] justify-center items-center ">
        <div class="mx-auto items-center">
            <button type="button" disabled>
                <svg class="block w-full mx-auto animate-spin h-10 text-stone-500" xmlns="http://www.w3.org/2000/svg"
                    fill="none" viewBox="0 0 24 24">
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4">
                    </circle>
                    <path class="opacity-75" fill="currentColor"
                        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z">
                    </path>
                </svg>
                <h1 class="text-stone-500 font-thin text-2xl text-center">{{ _args![0] }}ing {{ prop.name }}</h1>
                <h1 class="text-stone-400 font-thin text-lg text-center">executing...
                    $ <span class="font-normal italic">brew {{ prop.cmd }}</span></h1>
            </button>
        </div>
    </div>
</template>