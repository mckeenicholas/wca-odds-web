<script setup lang="ts">
import { AreaChart } from "../ui/chart-area";
import getHistValues from "../../lib/histogram";

const { mu, sigma, tau, color, max, min } = defineProps<{
  mu: number;
  sigma: number;
  tau: number;
  color: string;
  min: number;
  max: number;
}>();

const data = getHistValues(mu, sigma, tau, min, max);
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
