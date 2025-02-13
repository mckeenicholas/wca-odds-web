<script setup lang="ts">
import PieChart from "@/components/charts/PieChart.vue";

const { simulationResults, colors } = defineProps<{
  simulationResults: any;
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
