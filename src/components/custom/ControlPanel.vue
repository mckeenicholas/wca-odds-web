<script setup lang="ts">
import { eventNames, SupportedWCAEvent } from "@/lib/types";
import { defineProps } from "vue";
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from "@/components/ui/select";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import {
  NumberField,
  NumberFieldContent,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput,
} from "@/components/ui/number-field";

const props = defineProps<{
  selectedEventId: string;
  eventIds: SupportedWCAEvent[];
  simCount: number;
  monthCount: number;
}>();

const emit = defineEmits<{
  (event: "update:selectedEventId", value: string): void;
  (event: "update:simCount", value: number): void;
  (event: "update:monthCount", value: number): void;
  (event: "runSimulation"): void;
}>();
</script>

<template>
  <Select
    v-model="props.selectedEventId"
    @update:modelValue="
      (value) => emit('update:selectedEventId', value as SupportedWCAEvent)
    "
  >
    <SelectTrigger class="ms-0">
      <SelectValue />
    </SelectTrigger>
    <SelectContent>
      <SelectItem v-for="event of props.eventIds" :key="event" :value="event">
        {{ eventNames[event] }}
      </SelectItem>
    </SelectContent>
  </Select>
  <div class="border rounded-md my-2 p-2 flex items-center space-x-4">
    <Label for="simCount">Number of simulations:</Label>
    <Input
      placeholder="100000"
      class="min-w-16 max-w-36"
      id="simCount"
      v-model="props.simCount"
      @input="(value: number) => emit('update:simCount', value)"
    />
    <Label for="resultCutoff">Using results from past</Label>
    <NumberField
      v-model="props.monthCount"
      :default-value="18"
      :min="1"
      id="resultCutoff"
      class="w-24"
      @update:modelValue="(value) => emit('update:monthCount', value)"
    >
      <NumberFieldContent>
        <NumberFieldDecrement />
        <NumberFieldInput />
        <NumberFieldIncrement />
      </NumberFieldContent>
    </NumberField>
    <Label for="resultCutoff">{{
      props.monthCount === 1 ? "month" : "months"
    }}</Label>
    <div class="flex flex-grow justify-end">
      <Button @click="() => emit('runSimulation')">Run Simulation</Button>
    </div>
  </div>
</template>
