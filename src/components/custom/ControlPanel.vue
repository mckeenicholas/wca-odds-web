<script setup lang="ts">
import { eventNames, SupportedWCAEvent } from "@/lib/types";
import { ref, watch } from "vue";
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

const props = defineProps<{
  selectedEventId: string;
  eventIds: SupportedWCAEvent[];
  simCount: number;
  monthCount: number;
  includeDnf: boolean;
}>();

const emit = defineEmits<{
  (event: "update:selectedEventId", value: string): void;
  (event: "update:simCount", value: number): void;
  (event: "update:monthCount", value: number): void;
  (event: "update:includeDnf", value: boolean): void;
  (event: "runSimulation"): void;
}>();

const simCountValue = ref(props.simCount);
const selectedEventValue = ref(props.selectedEventId);
const monthCountValue = ref(props.monthCount);
const includeDNFValue = ref(props.includeDnf);

watch(simCountValue, (newValue) => {
  emit("update:simCount", newValue);
});

watch(selectedEventValue, (newValue) => {
  emit("update:selectedEventId", newValue);
});

watch(monthCountValue, (newValue) => {
  emit("update:monthCount", newValue);
});

watch(includeDNFValue, (newValue) => {
  emit("update:includeDnf", newValue);
});
</script>

<template>
  <Select v-model="selectedEventValue">
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
      v-model.number="simCountValue"
    />
    <Label for="resultCutoff">Using results from past</Label>
    <NumberField
      :default-value="18"
      :min="1"
      id="resultCutoff"
      class="w-24"
      v-model="monthCountValue"
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

    <Switch id="includeDNF" v-model="includeDNFValue" />
    <Label for="includeDNF">Include DNFs in Calculation</Label>

    <div class="flex flex-grow justify-end">
      <Button @click="() => emit('runSimulation')">Run Simulation</Button>
    </div>
  </div>
</template>
