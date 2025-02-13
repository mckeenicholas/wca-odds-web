<script setup lang="ts">
import PieChart from "@/components/charts/PieChart.vue";
import { SimulationResult } from "@/lib/types";

const { simulationResults, colors } = defineProps<{
  simulationResults: SimulationResult[];
  colors: string[];
}>();
</script>

<template>
  <div class="flex">
    <p class="border rounded-md my-2 py-2 px-4 me-2 flex-grow">
      winner info here
    </p>
    <div class="border rounded-md my-2 py-2 px-4">
      <PieChart :data="simulationResults" :colors="colors" />
    </div>
  </div>
</template>
