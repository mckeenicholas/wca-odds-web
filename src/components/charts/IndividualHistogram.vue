<script setup lang="ts">
import { AreaChart } from "@/components/ui/chart-area";

const { hist, color, simulations } = defineProps<{
  hist: Map<number, number>;
  color: string;
  simulations: number;
}>();

const data = [...hist]
  .map(([k, v]) => ({
    time: k / 10,
    probability: parseFloat(((v / simulations) * 100).toFixed(2)),
  }))

  .filter((item) => item.probability > 0.01)
  .sort((a, b) => a.time - b.time);

console.log(data);
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
