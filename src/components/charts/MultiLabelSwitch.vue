<script setup lang="ts">
import { Switch } from "../ui/switch";
import { Label } from "../ui/label";

const { left, right } = defineProps<{ left: string; right: string }>();
const model = defineModel<boolean>();
</script>

<template>
  <div class="ms-8 mt-2 flex items-center">
    <Label for="isSingle">{{ left }}</Label>
    <Switch v-model:checked="model" id="isSingle" class="mx-3" />
    <Label for="isSingle">{{ right }}</Label>
  </div>
</template>
