<script setup lang="ts">
const { title, data } = defineProps<{
  title?: string;
  data: {
    name: string;
    color: string;
    value: string | number;
  }[];
}>();

const toPlaceString = (place: number): string => {
  if (place >= 11 && place <= 13) {
    return `${place}th`;
  }

  switch (place % 10) {
    case 1:
      return `${place}st`;
    case 2:
      return `${place}nd`;
    case 3:
      return `${place}rd`;
    default:
      return `${place}th`;
  }
};

const getNumericValue = (val: string | number): number => {
  if (typeof val === "string") {
    return parseFloat(val);
  }
  return val;
};
</script>

<template>
  <div class="match-background p-2 rounded-md border-2">
    <p v-if="title" class="font-bold">{{ toPlaceString(parseInt(title)) }}</p>
    <div v-for="(item, key) in data" :key class="flex justify-between text-sm">
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
        >{{
          getNumericValue(item.value) >= 0.01
            ? getNumericValue(item.value).toFixed(2)
            : "<0.01"
        }}%</span
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
