<script setup lang="ts">
import ColoredCircle from "../custom/ColoredCircle.vue";
import { formatPercentage } from "@/lib/utils";

const { title, data } = defineProps<{
  title?: string;
  data: {
    name: string;
    color: string;
    value: string | number;
  }[];
}>();

const toPlaceString = (place: number): string => {
  const suffixes = ["th", "st", "nd", "rd"];
  const mod100 = place % 100;

  if (mod100 >= 11 && mod100 <= 13) {
    return `${place}th`;
  }

  const suffix = suffixes[place % 10] || suffixes[0];
  return `${place}${suffix}`;
};
</script>

<template>
  <div class="match-background p-2 rounded-md border-2">
    <p v-if="title" class="font-bold">{{ toPlaceString(parseInt(title)) }}</p>
    <div v-for="(item, key) in data" :key class="flex justify-between text-sm">
      <div class="flex items-center">
        <span class="w-2.5 h-2.5 mr-2">
          <ColoredCircle :color="item.color" />
        </span>
        <span>{{ item.name }}</span>
      </div>
      <span class="font-semibold ml-4">{{ formatPercentage(item.value) }}</span>
    </div>
  </div>
</template>

<style lang="css">
.match-background {
  background-color: hsl(var(--background));
  color: hsl(var(--foreground));
}
</style>
