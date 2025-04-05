<script setup lang="ts">
import { eventNames, SupportedWCAEvent } from "@/lib/types";
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
import { Switch } from "@/components/ui/switch";

const selectedEventId = defineModel<string>("selectedEventId");
const simCount = defineModel<number>("simCount");
const monthCount = defineModel<number>("monthCount");
const includeDnf = defineModel<boolean>("includeDnf");
const { eventIds, disableRun = false } = defineProps<{
  eventIds: SupportedWCAEvent[];
  disableRun?: boolean;
}>();

const emit = defineEmits<{
  (event: "runSimulation"): void;
}>();
</script>

<template>
  <Select v-model="selectedEventId">
    <SelectTrigger class="ms-0">
      <SelectValue />
    </SelectTrigger>
    <SelectContent>
      <SelectItem v-for="event of eventIds" :key="event" :value="event">
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
      v-model.number="simCount"
    />
    <Label for="resultCutoff">Using results from past</Label>
    <NumberField
      :default-value="18"
      :min="1"
      id="resultCutoff"
      class="w-24"
      v-model="monthCount"
    >
      <NumberFieldContent>
        <NumberFieldDecrement />
        <NumberFieldInput />
        <NumberFieldIncrement />
      </NumberFieldContent>
    </NumberField>
    <Label for="resultCutoff">{{
      monthCount === 1 ? "month" : "months"
    }}</Label>

    <Label for="includeDNF">Include DNFs in Calculation</Label>
    <Switch id="includeDNF" v-model="includeDnf" />

    <div class="flex flex-grow justify-end">
      <Button @click="() => emit('runSimulation')" :disabled="disableRun"
        >Run Simulation</Button
      >
    </div>
  </div>
</template>
