<script setup lang="ts">
import { AreaChart } from "@/components/ui/chart-area";
import getHistValues from "@/lib/histogram";

const {
  mu,
  sigma,
  tau,
  color,
  max = 10,
} = defineProps<{
  mu: number;
  sigma: number;
  tau: number;
  color: string;
  max: string;
}>();

const data = getHistValues(mu, sigma, tau, 0, max);
console.log(color, data);
</script>

<template>
  <div class="m-10">
    <AreaChart
      :data="data"
      index="name"
      :categories="['probability']"
      :colors="[color]"
    />
  </div>
</template>
