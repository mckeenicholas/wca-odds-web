<script setup lang="ts">
import { DonutChart } from "@/components/ui/chart-donut";
import { SimulationResult } from "@/lib/types";

const { data, colors } = defineProps<{
  data: SimulationResult[];
  colors: string[];
}>();

const chartData = data.map((item) => ({
  name: item.name,
  wins: item.results.win_count,
}));
</script>

<template>
  <DonutChart
    index="name"
    :category="'wins'"
    :data="chartData"
    :type="'pie'"
    :colors="colors"
  />
</template>
