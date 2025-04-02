<script setup lang="ts">
import { AreaChart } from "@/components/ui/chart-area";
import { SimulationResult } from "@/lib/types";
import { totalSolves } from "@/lib/utils";
import { Switch } from "@/components/ui/switch";
import { ref, computed } from "vue";
import { Label } from "../ui/label";
import HistogramCustomTooltip from "./HistogramCustomTooltip.vue";

const props = defineProps<{
  data: SimulationResult[];
  colors: string[];
}>();

const isAverage = ref<boolean>(false);

const chartData = computed(() => {
  const resultTimes = new Map<number, Map<string, number>>();

  props.data.forEach((person) => {
    const results = isAverage.value
      ? person.results.hist_values_average
      : person.results.hist_values_single;

    [...results].forEach(([key, time]) => {
      const name = person.name;

      if (!resultTimes.has(key)) {
        resultTimes.set(key, new Map<string, number>());
      }

      const timesMap = resultTimes.get(key)!;
      const solveCount = totalSolves(results);

      timesMap.set(
        name,
        solveCount == 0
          ? 0
          : (timesMap.get(name) || 0) +
              parseFloat((time / (solveCount / 100)).toFixed(4)),
      );
    });
  });

  const values = [...resultTimes.entries()]
    .map(([time, nameMap]) => ({
      time,
      ...Object.fromEntries(nameMap.entries()),
    }))
    .sort((a, b) => a.time - b.time);

  const reduceBy = Math.ceil(Math.log2(values.length)) - 8;

  if (reduceBy < 0) {
    return values;
  }

  const mergeFactor = Math.pow(2, reduceBy);

  let output = [];

  for (let i = 0; i < values.length; i += mergeFactor) {
    const items = values.slice(i, i + mergeFactor);

    let merged: Record<string, number> = {
      time: items[0].time,
    };

    for (const item of items) {
      for (const [key, value] of Object.entries(item)) {
        if (key !== "time") {
          merged[key] = (merged[key] || 0) + value / mergeFactor;
        }
      }
    }

    output.push(merged);
  }

  return output;
});

const names = computed(() =>
  props.data.map((person) => person.name),
) as unknown as "time"[];
</script>

<template>
  <div class="my-4 mx-4">
    <AreaChart
      class="-ms-6"
      :data="chartData"
      index="time"
      :categories="names"
      :colors="colors"
      :showLegend="false"
      :customTooltip="HistogramCustomTooltip"
      :showXAxis="false"
      :yFormatter="(value) => `${value}%`"
    />
    <div class="ms-8 mt-2 flex items-center">
      <Label for="isSingle">Single</Label>
      <Switch v-model="isAverage" id="isSingle" class="mx-3" />
      <Label for="isSingle">Average</Label>
    </div>
  </div>
</template>
