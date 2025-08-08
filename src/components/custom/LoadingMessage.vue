<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";

const { message } = defineProps<{ message: string }>();

const loadingText = ref(message);
const dots = ref("");
let intervalId: number;

const updateDots = () => {
  dots.value = dots.value.length < 3 ? dots.value + "." : "";
  loadingText.value = message + dots.value;
};

onMounted(() => {
  intervalId = window.setInterval(updateDots, 333);
});

onUnmounted(() => {
  clearInterval(intervalId);
});
</script>

<template>
  <div class="text-center text-2xl">
    {{ loadingText }}
  </div>
</template>
