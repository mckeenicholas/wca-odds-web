<script setup lang="ts">
const { color } = defineProps<{ color: string }>();
</script>

<template>
  <svg width="10" height="10" viewBox="0 0 30 30">
    <path
      d="M 15 15 m -14, 0 a 14,14 0 1,1 28,0 a 14,14 0 1,1 -28,0"
      :stroke="color"
      :fill="color"
      stroke-width="1"
    />
  </svg>
</template>
