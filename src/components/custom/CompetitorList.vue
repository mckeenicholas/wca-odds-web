<script setup lang="ts">
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
import { ChevronUp, CircleAlert } from "lucide-vue-next";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { ref } from "vue";
import ColoredCircle from "./ColoredCircle.vue";

const lowDataWarningThreshold = 12 as const;

const { simulationResults, colors, competitorsList, numSimulations, event } =
  defineProps<{
    simulationResults: SimulationResult[];
    colors: string[];
    competitorsList: string[];
    numSimulations: number;
    event: SupportedWCAEvent;
  }>();

const isOpen = ref<boolean[]>(Array(simulationResults.length).fill(false));

const model = defineModel<number[][]>({ required: true });
</script>

<template>
  <div class="border rounded-md mt-2">
    <div class="flex justify-between py-2 px-4 me-5">
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
        <Collapsible v-model:open="isOpen[personIdx]">
          <CollapsibleTrigger as-child>
            <div
              class="flex justify-between p-2 ps-1 cursor-pointer hover:bg-secondary rounded-md"
            >
              <div class="flex-1 text-left">
                <div class="flex flex-row">
                  <div class="flex flex-col justify-center">
                    <ColoredCircle :color="colors[personIdx]" />
                  </div>
                  <a
                    :href="`https://worldcubeassociation.org/persons/${competitorsList[personIdx]}`"
                    @click.stop
                    class="hover:underline"
                  >
                    {{ result.name }}
                  </a>
                  <TooltipProvider :delayDuration="250">
                    <Tooltip>
                      <TooltipTrigger>
                        <CircleAlert
                          v-show="result.sample_size < lowDataWarningThreshold"
                          class="scale-75 ms-1 text-red-600"
                        />
                      </TooltipTrigger>
                      <TooltipContent>
                        Competitor only has performed
                        {{ result.sample_size }} solves since date cutoff.
                      </TooltipContent>
                    </Tooltip>
                  </TooltipProvider>
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
              <ChevronUp
                class="scale-75 transition-transform duration-450"
                :class="{ '-rotate-180': isOpen[personIdx] }"
              />
            </div>
          </CollapsibleTrigger>
          <CollapsibleContent class="space-y-2">
            <IndividualHistogram
              :color="colors[personIdx]"
              :histSingle="result.results.hist_values_single"
              :histAverage="result.results.hist_values_average"
              :simulations="numSimulations * eventAttempts[event]"
              :event
            />
            <div
              class="flex flex-col lg:flex-row items-center px-2 lg:gap-4 lg:ms-2"
            >
              <div
                v-for="attemptIdx in eventAttempts[event]"
                v-bind:key="attemptIdx"
                class="flex mb-2 lg:mb-0 items-center gap-2"
              >
                <span class="whitespace-nowrap">Attempt {{ attemptIdx }}:</span>
                <div class="lg:max-w-24">
                  <ResultEntryField
                    :event
                    v-model="model[personIdx][attemptIdx - 1]"
                  />
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
