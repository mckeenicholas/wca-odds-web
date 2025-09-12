<script setup lang="ts">
import { Button } from "@/components/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { eventNames, SupportedWCAEvent } from "@/lib/types";
import { BREAKPOINT } from "@/lib/utils";
import { useWindowSize } from "@vueuse/core";
import ExpandableBox from "./ExpandableBox.vue";
import SimulationOptions from "./SimulationOptions.vue";

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
    <SelectTrigger class="ms-0 min-h-[42px]" aria-label="Event select dropdown">
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
    class="my-2 flex items-center space-x-4 rounded-md border p-2"
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
      <div class="flex flex-col items-stretch space-y-4 p-4">
        <SimulationOptions
          v-model:simCount="simCount"
          v-model:includeDnf="includeDnf"
          v-model:decay-rate="decayHalfLife"
          v-model:start-date="startDate"
          v-model:end-date="endDate"
        />
      </div>
    </ExpandableBox>
    <div class="mb-2 flex flex-col">
      <Button @click="() => emit('runSimulation')" :disabled="disableRun"
        >Run Simulation</Button
      >
    </div>
  </div>
</template>
