<script setup lang="ts">
import { AreaChart } from "@/components/ui/chart-area";
import { SimulationResult } from "@/lib/types";
import { totalSolves } from "@/lib/utils";
import { Switch } from "@/components/ui/switch";
import { ref, computed } from "vue";
import { Label } from "../ui/label";
import HistogramCustomTooltip from "./HistogramCustomTooltip.vue";

const { data, colors } = defineProps<{
  data: SimulationResult[];
  colors: string[];
}>();

const isAverage = ref<boolean>(false);

const reduceDataPoints = (
  values: Array<Record<string, number>>,
): Array<Record<string, number>> => {
  const reduceBy = Math.ceil(Math.log2(values.length)) - 8;

  if (reduceBy <= 0) {
    return values;
  }

  const mergeFactor = Math.pow(2, reduceBy);

  const chunkIndices = Array.from(
    { length: Math.ceil(values.length / mergeFactor) },
    (_, i) => i * mergeFactor,
  );

  return chunkIndices.map((startIndex) => {
    const chunk = values.slice(startIndex, startIndex + mergeFactor);

    const base = { time: chunk[0].time };

    const allKeys = [
      ...new Set(
        chunk.flatMap((item) =>
          Object.keys(item).filter((key) => key !== "time"),
        ),
      ),
    ];

    const averages = allKeys.reduce(
      (acc, key) => ({
        ...acc,
        [key]:
          chunk.reduce((sum, item) => sum + (item[key] || 0), 0) / mergeFactor,
      }),
      {},
    );

    return { ...base, ...averages };
  });
};

const chartData = computed(() => {
  const resultTimes = new Map<number, Map<string, number>>();

  data.forEach((person) => {
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

  return reduceDataPoints(values);
});

const names = data.map((person) => person.name) as unknown as "time"[];
</script>

<template>
  <div class="my-4 mx-4">
    <AreaChart
      class="-ms-6"
      :data="chartData"
      index="time"
      :categories="names"
      :colors
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
