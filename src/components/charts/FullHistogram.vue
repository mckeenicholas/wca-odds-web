<script setup lang="ts">
import { AreaChart } from "@/components/ui/chart-area";
import { SimulationResult } from "@/lib/types";

const { data, names, colors, simulations } = defineProps<{
  data: SimulationResult[];
  names: string[];
  colors: string[];
  simulations: number;
}>();

const resultTimes = new Map<number, Map<string, number>>();

data.forEach((person, idx) => {
  [...person.histValues].forEach(([k, v]) => {
    const key = k / 10;
    const name = names[idx];

    if (!resultTimes.has(key)) {
      resultTimes.set(key, new Map<string, number>());
    }

    const timesMap = resultTimes.get(key)!;
    timesMap.set(name, (timesMap.get(name) || 0) + (v / simulations));
  });
});

const chartData = [...resultTimes.entries()].map(([time, nameMap]) => ({
  time,
  ...Object.fromEntries(nameMap.entries())
}));

</script>

<template>
  <div class="my-10 mx-4">
    <AreaChart
      :data="chartData"
      index="time"
      :categories="names"
      :colors="colors"
      :showLegend="false"
    />
  </div>
</template>
