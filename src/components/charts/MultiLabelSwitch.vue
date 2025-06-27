<script setup lang="ts">
import { Switch } from "../ui/switch";
import { Label } from "../ui/label";
import { useId } from "vue";

const { left, right } = defineProps<{ left: string; right: string }>();
const model = defineModel<boolean>();

const inputId = useId();
</script>

<template>
  <div class="ms-8 mt-2 flex items-center">
    <Label :for="inputId">{{ left }}</Label>
    <Switch v-model:checked="model" :id="inputId" class="mx-3" />
    <Label :for="inputId">{{ right }}</Label>
  </div>
</template>
