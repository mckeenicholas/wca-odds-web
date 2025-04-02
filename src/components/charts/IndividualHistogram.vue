<script setup lang="ts">
import { AreaChart } from "@/components/ui/chart-area";
import { totalSolves } from "@/lib/utils";
import { computed } from "vue";
import HistogramCustomTooltip from "./HistogramCustomTooltip.vue";

interface DataPoint {
  time: number;
  single: number;
  average: number;
}

const props = defineProps<{
  histAverage: Map<number, number>;
  histSingle: Map<number, number>;
  color: string;
}>();

const solveCount = computed(() => totalSolves(props.histSingle));
const avgCount = computed(() => totalSolves(props.histAverage));

const data = computed(() =>
  [...new Set([...props.histSingle.keys(), ...props.histAverage.keys()])]
    .reduce((acc: DataPoint[], time) => {
      const single = parseFloat(
        ((props.histSingle.get(time) || 0) / (solveCount.value / 100)).toFixed(
          4,
        ),
      );
      const average = parseFloat(
        ((props.histAverage.get(time) || 0) / (avgCount.value / 100)).toFixed(
          4,
        ),
      );

      if (single > 0.0001 || average > 0.0001) {
        acc.push({ time, single, average });
      }

      return acc;
    }, [])
    .sort((a, b) => a.time - b.time),
);
</script>

<template>
  <div class="m-10">
    <AreaChart
      class="mb-2"
      :data="data"
      index="time"
      :categories="['single', 'average']"
      :colors="[color, `${color}88`]"
      :custom-tooltip="HistogramCustomTooltip"
      :showXAxis="false"
      :yFormatter="(value) => `${value}%`"
    />
  </div>
</template>
