// filepath:
/home/nicholas/wca-odds-web/src/components/custom/SimulationOptions.vue
<script setup lang="ts">
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  NumberField,
  NumberFieldContent,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput,
} from "@/components/ui/number-field";
import { Switch } from "@/components/ui/switch";

const simCount = defineModel<number>("simCount");
const monthCount = defineModel<number>("monthCount");
const includeDnf = defineModel<boolean>("includeDnf");
const decayHalfLife = defineModel<number>("decayRate");
</script>

<template>
  <div class="flex items-center min-[1200px]:pe-4 min-[1200px]:border-r">
    <Label for="simCount">Simulation Count:</Label>
    <Input class="max-w-24 ms-2" id="simCount" v-model.number="simCount" />
  </div>

  <div class="flex items-center min-[1200px]:pe-4 min-[1200px]:border-r">
    <Label for="resultCutoff">Using results from the past</Label>
    <NumberField
      :default-value="18"
      :min="1"
      id="resultCutoff"
      class="w-24 mx-2"
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
  <div class="flex items-center min-[1200px]:pe-4 min-[1200px]:border-r">
    <Label for="decayRate"> Result decay half-life </Label>
    <Input
      class="max-w-16 mx-2"
      id="decayRate"
      v-model.number="decayHalfLife"
    />
    <Label for="decayRate"> days </Label>
  </div>
  <div class="flex items-center">
    <Label for="includeDNF" class="me-2">Include DNFs</Label>
    <Switch id="includeDNF" v-model:checked="includeDnf" />
  </div>
</template>
