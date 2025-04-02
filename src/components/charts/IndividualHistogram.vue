<script setup lang="ts">
import { AreaChart } from "@/components/ui/chart-area";
import { totalSolves } from "@/lib/utils";
import { computed } from "vue";
import HistogramCustomTooltip from "./HistogramCustomTooltip.vue";

const props = defineProps<{
  histAverage: Map<number, number>;
  histSingle: Map<number, number>;
  color: string;
}>();

const solveCount = computed(() => totalSolves(props.histSingle));
const avgCount = computed(() => totalSolves(props.histAverage));

const data = computed(() =>
  [...new Set([...props.histSingle.keys(), ...props.histAverage.keys()])]
    .map((time) => ({
      time,
      single: parseFloat(
        ((props.histSingle.get(time) || 0) / (solveCount.value / 100)).toFixed(
          4,
        ),
      ),
      average: parseFloat(
        ((props.histAverage.get(time) || 0) / (avgCount.value / 100)).toFixed(
          4,
        ),
      ),
    }))
    .filter((item) => item.single > 0.0001 || item.average > 0.0001)
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
