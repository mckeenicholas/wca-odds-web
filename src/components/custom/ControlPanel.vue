<script setup lang="ts">
import { eventNames, SupportedWCAEvent } from "@/lib/types";
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { useWindowSize } from "@vueuse/core";
import ExpandableBox from "./ExpandableBox.vue";
import SimulationOptions from "./SimulationOptions.vue";
import { BREAKPOINT } from "@/lib/utils";

const selectedEventId = defineModel<string>("selectedEventId");
const simCount = defineModel<number>("simCount");
const includeDnf = defineModel<boolean>("includeDnf");
const decayHalfLife = defineModel<number>("decayRate");
const startDate = defineModel<Date>("startDate");
const endDate = defineModel<Date>("endDate");

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
    v-if="width >= BREAKPOINT"
    class="border rounded-md my-2 p-2 flex items-center space-x-4"
  >
    <SimulationOptions
      v-model:simCount="simCount"
      v-model:includeDnf="includeDnf"
      v-model:decay-rate="decayHalfLife"
      v-model:start-date="startDate"
      v-model:end-date="endDate"
    />
    <div class="flex flex-grow justify-end">
      <Button @click="() => emit('runSimulation')" :disabled="disableRun"
        >Run Simulation</Button
      >
    </div>
  </div>
  <div v-else>
    <ExpandableBox title="Options" class="my-2">
      <hr class="mx-2" />
      <div class="flex flex-col items-stretch p-4 space-y-4">
        <SimulationOptions
          v-model:simCount="simCount"
          v-model:includeDnf="includeDnf"
          v-model:decay-rate="decayHalfLife"
          v-model:start-date="startDate"
          v-model:end-date="endDate"
        />
      </div>
    </ExpandableBox>
    <div class="flex flex-col mb-2">
      <Button @click="() => emit('runSimulation')" :disabled="disableRun"
        >Run Simulation</Button
      >
    </div>
  </div>
</template>
