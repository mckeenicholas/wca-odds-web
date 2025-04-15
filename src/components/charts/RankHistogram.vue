<script setup lang="ts">
import { BarChart } from "@/components/ui/chart-bar";
import { SimulationResultProps } from "@/lib/types";
import { computed } from "vue";
import PercentageTooltip from "./PercentageTooltip.vue";

const { data, colors, numSimulations } =
  defineProps<Omit<SimulationResultProps, "event">>();

const chartData = computed(() =>
  Array.from({ length: data.length }, (_, idx) => ({
    name: idx + 1,
    ...Object.fromEntries(
      data.map((person) => [
        person.name,
        parseFloat(
          ((person.results.rank_dist[idx] / numSimulations) * 100).toFixed(2),
        ),
      ]),
    ),
  })),
);

const names = data.map((person) => person.name) as "name"[];
</script>

<template>
  <div class="mb-4 mt-2 mx-4">
    <BarChart
      :data="chartData"
      index="name"
      :categories="names"
      :colors
      :type="'stacked'"
      :showLegend="false"
      :customTooltip="PercentageTooltip"
    />
  </div>
</template>
