<script setup lang="ts">
import { DonutChart } from "@/components/ui/chart-donut";
import { SimulationResult } from "@/lib/types";
import { computed } from "vue";
import PercentageTooltip from "./PercentageTooltip.vue";

const { data, numSimulations, colors } = defineProps<{
  data: SimulationResult[];
  numSimulations: number;
  colors: string[];
}>();

const chartData = computed(() =>
  data.map((item) => ({
    name: item.name,
    wins: (item.results.win_count / numSimulations) * 100,
  })),
);
</script>

<template>
  <div class="max-w-96">
    <DonutChart
      index="name"
      :category="'wins'"
      :data="chartData"
      :type="'pie'"
      :colors
      :custom-tooltip="PercentageTooltip"
    />
  </div>
</template>
