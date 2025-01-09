<script setup lang="ts">
import { DonutChart } from "@/components/ui/chart-donut";

const { data, colors } = defineProps<{
  data: { name: string; wins: number }[];
  colors: string[];
}>();
</script>

<template>
  <DonutChart
    index="name"
    :category="'wins'"
    :data="data"
    :type="'pie'"
    :colors="colors"
  />
</template>
