<script setup lang="ts">
import { useRouter } from "vue-router";
import { Button } from "@/components/ui/button";

const router = useRouter();
</script>

<template>
  <Button variant="outline" class="absolute m-2" @click="router.back()"
    >Back</Button
  >
</template>
