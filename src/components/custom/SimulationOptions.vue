<script setup lang="ts">
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import DateRangePicker from "./DateRangePicker.vue";

const simCount = defineModel<number>("simCount");
const includeDnf = defineModel<boolean>("includeDnf");
const decayHalfLife = defineModel<number>("decayRate");
const startDate = defineModel<Date>("startDate");
const endDate = defineModel<Date>("endDate");
</script>

<template>
  <div class="flex items-center min-[1255px]:pe-4 min-[1255px]:border-r">
    <Label for="simCount">Simulation Count:</Label>
    <Input class="max-w-24 ms-2 h-9" id="simCount" v-model.number="simCount" />
  </div>

  <div class="flex items-center min-[1255px]:pe-4 min-[1255px]:border-r">
    <Label for="resultCutoff" class="me-2">Use results from:</Label>
    <DateRangePicker
      id="resultCutoff"
      v-model:start-date="startDate"
      v-model:end-date="endDate"
    />
  </div>
  <div class="flex items-center min-[1255px]:pe-4 min-[1255px]:border-r">
    <Label for="decayRate">Result decay half-life</Label>
    <Input
      class="max-w-16 mx-2 h-9"
      id="decayRate"
      v-model.number="decayHalfLife"
    />
    <Label for="decayRate">days</Label>
  </div>
  <div class="flex items-center">
    <Label for="includeDNF" class="me-2">Include DNFs</Label>
    <Switch id="includeDNF" v-model:checked="includeDnf" />
  </div>
</template>
