<script setup lang="ts">
import { toClockFormat } from "@/lib/utils";

const props = defineProps<{
  title?: string;
  data: {
    name: string;
    color: string;
    value: number;
  }[];
}>();

const time = parseInt(props.title ?? "0") * 10;
</script>

<template>
  <div class="match-backround p-2 rounded-md border-2">
    {{ toClockFormat(time) }}
    <div
      v-for="(item, key) in data"
      :key="key"
      class="flex justify-between text-sm"
    >
      <div class="flex items-center">
        <span class="w-2.5 h-2.5 mr-2">
          <svg width="100%" height="100%" viewBox="0 0 30 30">
            <path
              d=" M 15 15 m -14, 0 a 14,14 0 1,1 28,0 a 14,14 0 1,1 -28,0"
              :stroke="item.color"
              :fill="item.color"
              stroke-width="1"
            />
          </svg>
        </span>
        <span>{{ item.name }}</span>
      </div>
      <span class="font-semibold ml-4"
        >{{ item.value >= 0.01 ? item.value.toFixed(2) : "<0.01" }}%</span
      >
    </div>
  </div>
</template>

<style lang="css">
.match-backround {
  background-color: hsl(var(--background));
  color: hsl(var(--foreground));
}
</style>
