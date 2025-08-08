<script setup lang="ts">
import { AreaChart } from "@/components/ui/chart-area";
import { SupportedWCAEvent } from "@/lib/types";
import { createFMCTooltip, renderTime, totalSolves } from "@/lib/utils";
import { computed, ref } from "vue";
import MultiLabelSwitch from "./MultiLabelSwitch.vue";

interface DataPoint {
  time: number;
  single: number;
  average: number;
}

const { histAverage, histSingle, color, event } = defineProps<{
  histAverage: Map<number, number>;
  histSingle: Map<number, number>;
  color: string;
  event: SupportedWCAEvent;
}>();

const histogramTooltip = createFMCTooltip(event);

const isCDF = ref<boolean>(false);

const solveCount = computed(() => totalSolves(histSingle));
const avgCount = computed(() => totalSolves(histAverage));

const padChartData = (data: DataPoint[]): DataPoint[] => {
  if (data.length === 0) return [];

  const startTime = data[0].time;
  const endTime = data[data.length - 1].time;

  const beforePadding = [
    { time: startTime - 20, single: 0, average: 0 },
    { time: startTime - 10, single: 0, average: 0 },
  ];

  const afterPadding = isCDF.value
    ? [
        {
          time: endTime + 10,
          single: data[data.length - 1].single,
          average: data[data.length - 1].average,
        },
        {
          time: endTime + 20,
          single: data[data.length - 1].single,
          average: data[data.length - 1].average,
        },
      ]
    : [
        { time: endTime + 10, single: 0, average: 0 },
        { time: endTime + 20, single: 0, average: 0 },
      ];

  return [...beforePadding, ...data, ...afterPadding];
};

const data = computed(() => {
  const dataFormatted = [
    ...new Set([...histSingle.keys(), ...histAverage.keys()]),
  ]
    .sort((a, b) => a - b)
    .reduce((acc: DataPoint[], time, idx) => {
      const prevTimeSingle = isCDF.value
        ? idx === 0
          ? 0
          : acc[idx - 1].single
        : 0;
      const prevTimeAverage = isCDF.value
        ? idx === 0
          ? 0
          : acc[idx - 1].average
        : 0;

      const single = parseFloat(
        (
          (histSingle.get(time) || 0) / (solveCount.value / 100) +
          prevTimeSingle
        ).toFixed(4),
      );
      const average = parseFloat(
        (
          (histAverage.get(time) || 0) / (avgCount.value / 100) +
          prevTimeAverage
        ).toFixed(4),
      );

      if (single > 0.0001 || average > 0.0001) {
        return [...acc, { time, single, average }];
      }

      return acc;
    }, []);

  if (dataFormatted.length >= 5) {
    return dataFormatted;
  }

  // It appears that there is undefined behavior if the chart has fewer than 5 data points
  // Pad the beginning and end with 0 values to ensure this
  return padChartData(dataFormatted);
});

const xFormatter = (value: number | Date) =>
  renderTime(data.value[value as number].time * 10, event === "333fm");
</script>

<template>
  <div class="-me-6 mb-4 ms-4 mt-2">
    <AreaChart
      class="-ms-6"
      :data
      index="time"
      :categories="['single', 'average']"
      :colors="[color, `${color}88`]"
      :custom-tooltip="histogramTooltip"
      :showXAxis="true"
      :yFormatter="(value) => `${value}%`"
      :xFormatter
    />
    <MultiLabelSwitch left="Probability" right="Cumulative" v-model="isCDF" />
  </div>
</template>
