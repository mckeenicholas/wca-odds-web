<script setup lang="ts">
import { DonutChart } from "@/components/ui/chart-donut";
import { SimulationResultProps } from "@/lib/types";
import { computed } from "vue";
import PercentageTooltip from "./PercentageTooltip.vue";

const { data, numSimulations, colors } =
  defineProps<Omit<SimulationResultProps, "event">>();

const chartData = computed(() =>
  data.map((item) => ({
    name: item.name,
    wins: (item.win_count / numSimulations) * 100,
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
