<script setup lang="ts">
import { AreaChart } from "@/components/ui/chart-area";
import { SimulationResult } from "@/lib/types";

const { data, colors, simulations } = defineProps<{
  data: SimulationResult[];
  colors: string[];
  simulations: number;
}>();

const resultTimes = new Map<number, Map<string, number>>();

data.forEach((person) => {
  [...person.results.hist_values_single].forEach(([k, v]) => {
    const key = k / 10;
    const name = person.name;

    if (!resultTimes.has(key)) {
      resultTimes.set(key, new Map<string, number>());
    }

    const timesMap = resultTimes.get(key)!;
    timesMap.set(
      name,
      (timesMap.get(name) || 0) + parseFloat((v / simulations).toFixed(4)),
    );
  });
});

const chartData = [...resultTimes.entries()]
  .map(([time, nameMap]) => ({
    time,
    ...Object.fromEntries(nameMap.entries()),
  }))
  .sort((a, b) => a.time - b.time);

const names = data.map((person) => person.name) as "time"[];
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
