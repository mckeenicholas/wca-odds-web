<script setup lang="ts">
import { BarChart } from "@/components/ui/chart-bar";

const { data, colors, count } = defineProps<{
  data: any;
  colors: string[];
  count: number;
}>();

type ChartRow = {
  name: number;
  [key: string]: any;
};

let chartData: ChartRow[] = [];

data[0].ranks.forEach((_: number, itemIndex: number) => {
  let row: ChartRow = { name: itemIndex + 1 };
  data.forEach((_: any, idx: number) => {
    row[data[idx].name] = (100 * data[idx].ranks[itemIndex]) / count;
  });
  chartData.push(row);
});

const categories = data.map((item: { name: any }) => item.name);
</script>

<template>
  <div class="my-10 mx-4">
    <BarChart
      :data="chartData"
      index="name"
      :categories="categories"
      :colors="colors"
      :type="'stacked'"
      :showLegend="false"
    />
  </div>
</template>
