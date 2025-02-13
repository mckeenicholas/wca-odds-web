<script setup lang="ts">
import { BarChart } from "@/components/ui/chart-bar";
import { SimulationResult } from "@/lib/types";

const { data, colors, count } = defineProps<{
  data: SimulationResult[];
  colors: string[];
  count: number;
}>();

const chartData = Array.from({ length: data.length }, (_, idx) => ({
  name: idx + 1,
  ...Object.fromEntries(
    data.map((person) => [
      person.name,
      parseFloat(((person.results.rank_dist[idx] / count) * 100).toFixed(2)),
    ]),
  ),
}));

const names = data.map((person) => person.name) as "name"[];
</script>

<template>
  <div class="my-10 mx-4">
    <BarChart
      :data="chartData"
      index="name"
      :categories="names"
      :colors="colors"
      :type="'stacked'"
      :showLegend="false"
    />
  </div>
</template>
