<script setup lang="ts">
import ColoredCircle from "@/components/custom/ColoredCircle.vue";
import { ChartTooltipProps } from "@/lib/types";
import { renderTime } from "@/lib/utils";

const { title, data, isFmc = false } = defineProps<ChartTooltipProps>();

const timeRawValue = parseInt(title ?? "0") * 10;
const timeDisplayValue = renderTime(timeRawValue, isFmc);
</script>

<template>
  <div class="match-background rounded-md border-2 p-2">
    <p class="font-bold">{{ timeDisplayValue }}</p>
    <div v-for="(item, key) in data" :key class="flex justify-between text-sm">
      <div class="flex items-center">
        <span class="mr-2 h-2.5 w-2.5">
          <ColoredCircle :color="item.color" />
        </span>
        <span>{{ item.name }}</span>
      </div>
      <span class="ml-4 font-semibold"
        >{{ item.value >= 0.01 ? item.value.toFixed(2) : "<0.01" }}%</span
      >
    </div>
  </div>
</template>

<style lang="css">
.match-background {
  background-color: hsl(var(--background));
  color: hsl(var(--foreground));
}
</style>
