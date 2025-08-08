<script setup lang="ts">
const { mean, stdDev, gamma, mu, sigma, tau, dnfRate } = defineProps<{
  mean: number;
  stdDev: number;
  gamma: number;
  mu: number;
  sigma: number;
  tau: number;
  dnfRate: number;
}>();
</script>

<template>
  <div class="mx-6 my-10 flex flex-row space-x-4 text-lg">
    <div>m = {{ mean }}</div>
    <div>s = {{ stdDev }}</div>
    <div>&gamma; = {{ gamma }}</div>
    <div>&mu; = {{ mu }}</div>
    <div>&sigma; = {{ sigma }}</div>
    <div>&tau; = {{ tau }}</div>
    <div>DNF rate = {{ dnfRate }}%</div>
  </div>
</template>
