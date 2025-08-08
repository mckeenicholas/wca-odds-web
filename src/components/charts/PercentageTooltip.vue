<script setup lang="ts">
import { formatPercentage } from "@/lib/utils";
import ColoredCircle from "../custom/ColoredCircle.vue";

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
  <div class="match-background rounded-md border-2 p-2">
    <p v-if="title" class="font-bold">{{ toPlaceString(parseInt(title)) }}</p>
    <div v-for="(item, key) in data" :key class="flex justify-between text-sm">
      <div class="flex items-center">
        <span class="mr-2 h-2.5 w-2.5">
          <ColoredCircle :color="item.color" />
        </span>
        <span>{{ item.name }}</span>
      </div>
      <span class="ml-4 font-semibold">{{ formatPercentage(item.value) }}</span>
    </div>
  </div>
</template>

<style lang="css">
.match-background {
  background-color: hsl(var(--background));
  color: hsl(var(--foreground));
}
</style>
