<script setup lang="ts">
import { AreaChart } from "@/components/ui/chart-area";
import {
  SimulationResult,
  SupportedWCAEvent,
  ChartTooltipProps,
} from "@/lib/types";
import { totalSolves, renderTime } from "@/lib/utils";
import { ref, computed, h } from "vue";
import HistogramCustomTooltip from "./HistogramCustomTooltip.vue";
import MultiLabelSwitch from "./MultiLabelSwitch.vue";

const { data, event, colors } = defineProps<{
  data: SimulationResult[];
  event: SupportedWCAEvent;
  colors: string[];
}>();

const histogramTooltip = computed(() => {
  return (props: ChartTooltipProps) =>
    h(HistogramCustomTooltip, {
      ...props,
      isFmc: event === "333fm",
    });
});

const isAverage = ref<boolean>(false);
const isCDF = ref<boolean>(false);

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
  const [min, max] = data.reduce(
    ([minAccPerson, maxAccPerson], person) => {
      const results = isAverage.value
        ? person.results.hist_values_average
        : person.results.hist_values_single;

      const [minPerson, maxPerson] = [...results].reduce(
        ([minAcc, maxAcc], [time]) => [
          Math.min(time, minAcc),
          Math.max(time, maxAcc),
        ],
        [Number.MAX_SAFE_INTEGER, 0],
      );

      return [
        Math.min(minPerson, minAccPerson),
        Math.max(maxPerson, maxAccPerson),
      ];
    },
    [Number.MAX_SAFE_INTEGER, 0],
  );

  const resultTimes = new Map<number, Map<string, number>>();

  data.forEach((person) => {
    const results = isAverage.value
      ? person.results.hist_values_average
      : person.results.hist_values_single;

    const solveCount = totalSolves(results);

    for (let i = min; i <= max; i++) {
      const numOccurrences = results.get(i) ?? 0;

      if (!resultTimes.has(i)) {
        resultTimes.set(i, new Map<string, number>());
      }

      const timesMap = resultTimes.get(i)!;

      const curVal = parseFloat(
        (numOccurrences / (solveCount / 100)).toFixed(4),
      );

      if (isCDF.value) {
        const prevTime =
          i === min ? 0 : resultTimes.get(i - 1)!.get(person.name)!;
        timesMap.set(person.name, prevTime + curVal);
      } else {
        timesMap.set(person.name, curVal);
      }
    }
  });

  const values = [...resultTimes.entries()]
    .map(([time, nameMap]) => ({
      time,
      ...Object.fromEntries(nameMap.entries()),
    }))
    .sort((a, b) => a.time - b.time);

  return reduceDataPoints(values);
});

const xFormatter = (value: number | Date) =>
  renderTime(chartData.value[value as number].time * 10, event === "333fm");

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
      :customTooltip="histogramTooltip"
      :showXAxis="true"
      :yFormatter="(value) => `${value}%`"
      :xFormatter
    />
    <div class="flex">
      <MultiLabelSwitch left="Single" right="Average" v-model="isAverage" />
      <MultiLabelSwitch left="Probability" right="Cumulative" v-model="isCDF" />
    </div>
  </div>
</template>
