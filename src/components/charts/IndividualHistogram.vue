<script setup lang="ts">
import { AreaChart } from "@/components/ui/chart-area";
import { totalSolves, renderTime } from "@/lib/utils";
import { computed, h } from "vue";
import HistogramCustomTooltip from "./HistogramCustomTooltip.vue";
import { ChartTooltipProps, SupportedWCAEvent } from "@/lib/types";

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

const histogramTooltip = computed(() => {
  return (props: ChartTooltipProps) =>
    h(HistogramCustomTooltip, {
      ...props,
      isFmc: event === "333fm",
    });
});

const solveCount = computed(() => totalSolves(histSingle));
const avgCount = computed(() => totalSolves(histAverage));

const padChartData = (data: DataPoint[]): DataPoint[] => {
  if (data.length === 0) return [];

  const startTime = data[0].time;
  const endTime = data[data.length - 1].time;

  const padPoint = (time: number) => ({ time, single: 0, average: 0 });

  return [
    padPoint(startTime - 20),
    padPoint(startTime - 10),
    ...data,
    padPoint(endTime + 10),
    padPoint(endTime + 20),
  ];
};

const data = computed(() => {
  const dataFormatted = [
    ...new Set([...histSingle.keys(), ...histAverage.keys()]),
  ]
    .reduce((acc: DataPoint[], time) => {
      const single = parseFloat(
        ((histSingle.get(time) || 0) / (solveCount.value / 100)).toFixed(4),
      );
      const average = parseFloat(
        ((histAverage.get(time) || 0) / (avgCount.value / 100)).toFixed(4),
      );

      if (single > 0.0001 || average > 0.0001) {
        acc.push({ time, single, average });
      }

      return acc;
    }, [])
    .sort((a, b) => a.time - b.time);

  if (dataFormatted.length >= 5) {
    return dataFormatted;
  }

  // It appears that there is undefined behavior if the chart has fewer than 5 data points
  // Pad the beginning and end with 0 values to ensure this
  return padChartData(dataFormatted);
});

console.log(data);
</script>

<template>
  <div class="m-10">
    <AreaChart
      class="mb-2"
      :data
      index="time"
      :categories="['single', 'average']"
      :colors="[color, `${color}88`]"
      :custom-tooltip="histogramTooltip"
      :showXAxis="true"
      :yFormatter="(value) => `${value}%`"
      :xFormatter="
        (value) =>
          renderTime(data[value as number].time * 10, event === '333fm')
      "
    />
  </div>
</template>
