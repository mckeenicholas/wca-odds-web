<script setup lang="ts">
import { SimulationResult, SupportedWCAEvent } from "@/lib/types";
import CompetitorDropdown from "./CompetitorDropdown.vue";

const { simulationResults, colors, competitorsList, numSimulations, event } =
  defineProps<{
    simulationResults: SimulationResult[];
    colors: string[];
    competitorsList: string[];
    numSimulations: number;
    event: SupportedWCAEvent;
  }>();

const model = defineModel<number[][]>({ required: true });
</script>

<template>
  <div class="border rounded-md mt-2">
    <div class="flex justify-between py-2 px-4 me-5">
      <div class="flex-1 text-left">Name</div>
      <div class="flex-1 text-center">Chance of winning</div>
      <div class="flex-1 text-center">Chance of podiuming</div>
      <div class="flex-1 text-center">Expected rank</div>
    </div>
    <hr class="mx-2" />
    <ol>
      <li
        v-for="(result, personIdx) in simulationResults"
        :key="personIdx"
        class="p-1 rounded-md"
      >
        <CompetitorDropdown
          :result
          :event
          :num-simulations
          :color="colors[personIdx]"
          :wca-id="competitorsList[personIdx]"
          v-model="model[personIdx]"
        />
      </li>
    </ol>
  </div>
</template>
