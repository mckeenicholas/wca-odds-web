<script setup lang="ts">
import PieChart from "@/components/charts/PieChart.vue";
import { SimulationResult, SupportedWCAEvent, eventNames } from "@/lib/types";

const { simulationResults, colors, numSimulations, event } = defineProps<{
  simulationResults: SimulationResult[];
  colors: string[];
  numSimulations: number;
  event: SupportedWCAEvent;
}>();

const resultsSorted = [...simulationResults].sort(
  (a, b) => b.results.win_count - a.results.win_count,
);

const avgRank = (resultsSorted[0].results.total_rank / numSimulations).toFixed(
  2,
);

const winChance = (
  (resultsSorted[0].results.win_count / numSimulations) *
  100
).toFixed(2);

const podiumChance = (
  (resultsSorted[0].results.pod_count / numSimulations) *
  100
).toFixed(2);

const expectedAvg = (resultsSorted[0].results.mean_no_dnf / 100).toFixed(2);
</script>

<template>
  <div class="flex flex-col md:flex-row gap-2 mb-2 h-full">
    <div class="flex-grow">
      <div class="border rounded-md p-4 h-full">
        <h3 class="font-bold text-lg mb-2">
          {{ eventNames[event] }} Statistics
        </h3>
        <div class="space-y-2">
          <p class="text-sm">
            <span class="font-semibold">{{ resultsSorted[0].name }}</span> has
            the highest odds of winning with:
          </p>
          <ul class="list-disc list-inside text-sm ml-4">
            <li>{{ winChance }}% chance of winning</li>
            <li>{{ podiumChance }}% chance of podium finish</li>
            <li>Average rank of {{ avgRank }}</li>
            <li>Expected average of {{ expectedAvg }} seconds</li>
          </ul>
        </div>
      </div>
    </div>
    <div class="border rounded-md p-2">
      <PieChart :data="simulationResults" :colors="colors" />
    </div>
  </div>
</template>
