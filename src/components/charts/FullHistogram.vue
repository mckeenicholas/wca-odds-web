<script setup lang="ts">
import { AreaChart } from "../ui/chart-area";
import getHistValues from "../../lib/histogram";

const { min, max, data, colors } = defineProps<{
  min: number;
  max: number;
  data: { name: string; mu: number; sigma: number; tau: number }[];
  colors: string[];
}>();
const allData = data.map((item) =>
  getHistValues(item.mu, item.sigma, item.tau, min, max),
);

type ChartRow = {
  name: number;
  [key: string]: any;
};

let chartData: ChartRow[] = [];

allData[0].forEach((item: { name: any }, itemIndex: number) => {
  let row: ChartRow = { name: item.name };
  allData.forEach((_, idx) => {
    row[data[idx].name] = allData[idx][itemIndex].probability;
  });
  chartData.push(row);
});

const categories = data.map((item) => item.name);
</script>

<template>
  <div class="my-10 mx-4">
    <AreaChart
      :data="chartData"
      index="name"
      :categories="categories"
      :colors="colors"
      :showLegend="false"
    />
  </div>
</template>
