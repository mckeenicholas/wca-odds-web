<script setup lang="ts">
import { ChevronUp } from "lucide-vue-next";

const { up, animate = true } = defineProps<{ up: boolean; animate?: boolean }>();
</script>

<template>
  <ChevronUp
    class="scale-75 duration-450"
    :class="{ '-rotate-180': up, 'transition-transform': animate }"
  />
</template>
