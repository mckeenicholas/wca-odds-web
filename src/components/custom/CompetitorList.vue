<script setup lang="ts">
import { Icon } from "@iconify/vue";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import IndividualHistogram from "@/components/charts/IndividualHistogram.vue";
import {
  eventAttempts,
  SimulationResult,
  SupportedWCAEvent,
} from "@/lib/types";
import ResultEntryField from "@/components/custom/ResultEntryField.vue";

const { simulationResults, colors, competitorsList, numSimulations, event } =
  defineProps<{
    simulationResults: SimulationResult[];
    colors: string[];
    competitorsList: string[];
    numSimulations: number;
    event: SupportedWCAEvent;
  }>();

const model = defineModel<number[][]>({ required: true });
</script>

<template>
  <div class="border rounded-md mt-2">
    <div class="flex justify-between py-2 px-4">
      <div class="flex-1 text-left">Name</div>
      <div class="flex-1 text-center">Chance of winning</div>
      <div class="flex-1 text-center">Chance of podiuming</div>
      <div class="flex-1 text-center">Expected rank</div>
    </div>
    <hr class="mx-2" />
    <ol>
      <li
        v-for="(result, personIdx) in simulationResults"
        :key="personIdx"
        class="p-1 rounded-md"
      >
        <Collapsible>
          <CollapsibleTrigger as-child>
            <div
              class="flex justify-between p-2 cursor-pointer hover:bg-secondary rounded-md"
            >
              <div class="flex-1 text-left">
                <div class="flex flex-row">
                  <div class="flex flex-col justify-center">
                    <Icon
                      icon="radix-icons:dot-filled"
                      class="scale-150"
                      :style="{ color: colors[personIdx] }"
                    />
                  </div>
                  <a
                    :href="`https://worldcubeassociation.org/persons/${competitorsList[personIdx]}`"
                    @click.stop
                    class="hover:underline"
                  >
                    {{ result.name }}
                  </a>
                </div>
              </div>
              <div class="flex-1 text-center">
                {{ (result.results.win_count * 100) / numSimulations }}%
              </div>
              <div class="flex-1 text-center">
                {{ (result.results.pod_count * 100) / numSimulations }}%
              </div>
              <div class="flex-1 text-center">
                {{ result.results.total_rank / numSimulations }}
              </div>
            </div>
          </CollapsibleTrigger>
          <CollapsibleContent class="space-y-2">
            <IndividualHistogram
              :color="colors[personIdx]"
              :histSingle="result.results.hist_values_single"
              :histAverage="result.results.hist_values_average"
              :simulations="numSimulations * eventAttempts[event]"
              class="border rounded-md m-2 p-2"
            />
            <div class="flex gap-4 items-center px-2">
              <div
                v-for="attemptIdx in eventAttempts[event]"
                v-bind:key="attemptIdx"
                class="flex items-center gap-2"
              >
                <span class="whitespace-nowrap">Attempt {{ attemptIdx }}:</span>
                <div class="max-w-24">
                  <ResultEntryField v-model="model[personIdx][attemptIdx]" />
                </div>
              </div>
            </div>
            <hr class="mx-2" />
          </CollapsibleContent>
        </Collapsible>
      </li>
    </ol>
  </div>
</template>
