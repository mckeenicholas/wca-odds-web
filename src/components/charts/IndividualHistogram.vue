<script setup lang="ts">
import { AreaChart } from "@/components/ui/chart-area";
import { totalSolves } from "@/lib/utils";

const { hist, color } = defineProps<{
  hist: Map<number, number>;
  color: string;
}>();

const solveCount = totalSolves(hist);

const data = [...hist]
  .map(([k, v]) => ({
    time: k / 10,
    probability: parseFloat((v / solveCount).toFixed(4)),
  }))
  .filter((item) => item.probability > 0.0001)
  .sort((a, b) => a.time - b.time);
</script>

<template>
  <div class="m-10">
    <AreaChart
      :data="data"
      index="time"
      :categories="['probability']"
      :colors="[color]"
    />
  </div>
</template>
