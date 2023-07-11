<script setup lang="ts">
import { ref } from 'vue'

const pa = [22277,22727,22777,23227,23327,23357,23537,23557,25237,25357,25537,25577,27277,27337,27527,27737,32237,32257,32327,32377,32537,33377,33577,33757,35227,35257,35327,35527,35537,37277,37337,37357,37537,52237,52727,52757,53327,53377,53527,53777,55337,57527,57557,57727,57737,72227,72277,72337,72577,72727,73237,73277,73327,73727,73757,75227,75277,75337,75377,75527,75557,75577,77237,77377,77527,77557] 

const primosAdri = ref(pa)

const emit = defineEmits(["selectedPrimoAdri"])
 
function selectedPrimoAdri(primoAdri: number) {
    console.log(primoAdri);
    emit("selectedPrimoAdri", primoAdri);
}

</script>

<template>
    <div class="m-4 flex flex-col justify-center">
        <span class="w-32 mr-2">Primos de Adri -></span>
        <div class="flex flex-row items-center overflow-x-auto">
            <ul class="flex flex-row last:mr-0" v-for="(p, i) in primosAdri">
                <li :id="i.toString()" class="flex bg-red-300 px-2 py-1 rounded mr-2 hover:cursor-pointer hover:bg-red-400" @click="selectedPrimoAdri(p)">
                    {{p}}
                </li>
            </ul>
        </div>
    </div>
</template>