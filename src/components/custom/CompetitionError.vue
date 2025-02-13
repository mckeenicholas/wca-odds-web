<script setup lang="ts">
import { Button } from "@/components/ui/button";

const { error } = defineProps<{
  error: string;
}>();

const goBack = () => {
  window.history.back();
};
</script>

<template>
  <div class="flex flex-col items-center justify-center">
    <p>Error: {{ error }}</p>
    <div class="mt-2">
      <Button :onclick="goBack"> Back </Button>
    </div>
  </div>
</template>
