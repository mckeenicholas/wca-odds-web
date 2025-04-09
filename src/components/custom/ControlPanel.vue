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
import { useWindowSize } from "@vueuse/core";
import ExpandableBox from "./ExpandableBox.vue";

const breakpoint = 1100;

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

const { width } = useWindowSize();
</script>

<template>
  <Select v-model="selectedEventId">
    <SelectTrigger class="ms-0 min-h-[42px]">
      <SelectValue />
    </SelectTrigger>
    <SelectContent>
      <SelectItem v-for="event of eventIds" :key="event" :value="event">
        {{ eventNames[event] }}
      </SelectItem>
    </SelectContent>
  </Select>
  <div
    v-if="width >= breakpoint"
    class="border rounded-md my-2 p-2 flex items-center space-x-4"
  >
    <Label for="simCount">Number of simulations:</Label>
    <Input
      placeholder="100000"
      class="min-w-16 max-w-36"
      id="simCount"
      v-model.number="simCount"
    />
    <div class="flex items-center space-x-2">
      <Label for="resultCutoff">Using results from the past</Label>
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
    </div>

    <Label for="includeDNF">Include DNFs in Calculation</Label>
    <Switch id="includeDNF" v-model="includeDnf" />

    <div class="flex flex-grow justify-end">
      <Button @click="() => emit('runSimulation')" :disabled="disableRun"
        >Run Simulation</Button
      >
    </div>
  </div>
  <div v-else>
    <ExpandableBox title="Options" class="my-2">
      <hr class="mx-2 mb-2" />
      <div class="flex flex-col items-center">
        <div class="flex items-center space-x-2 mb-4">
          <Label for="simCount">Number of simulations:</Label>
          <Input
            placeholder="100000"
            class="min-w-16 max-w-36"
            id="simCount"
            v-model.number="simCount"
          />
        </div>
        <div class="flex items-center space-x-2 mb-4">
          <Label for="resultCutoff">Using results from the past</Label>
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
        </div>

        <div class="flex items-center space-x-2 mb-4">
          <Label for="includeDNF">Include DNFs in Calculation</Label>
          <Switch id="includeDNF" v-model="includeDnf" />
        </div>
      </div>
    </ExpandableBox>
    <div class="flex flex-col mb-2">
      <Button @click="() => emit('runSimulation')" :disabled="disableRun"
        >Run Simulation</Button
      >
    </div>
  </div>
</template>
