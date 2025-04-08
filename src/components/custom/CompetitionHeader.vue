<script setup lang="ts">
const { name } = defineProps<{
  name: string;
}>();
</script>

<template>
  <h1 class="text-center text-2xl font-bold mt-4 mb-4">
    Results for {{ name }}
  </h1>
</template>
