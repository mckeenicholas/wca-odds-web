<script setup lang="ts">
import { ChevronUp } from "lucide-vue-next";

const { up, animate = true } = defineProps<{
  up: boolean;
  animate?: boolean;
}>();
</script>

<template>
  <ChevronUp
    class="duration-450 scale-75"
    :class="{ '-rotate-180': up, 'transition-transform': animate }"
  />
</template>
