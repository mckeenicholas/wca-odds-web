<script setup lang="ts">
import {
  eventAttempts,
  SimulationResult,
  SupportedWCAEvent,
} from "@/lib/types";
import { computed, ref } from "vue";
import {
  Collapsible,
  CollapsibleTrigger,
  CollapsibleContent,
} from "../ui/collapsible";
import ColoredCircle from "./ColoredCircle.vue";
import {
  TooltipProvider,
  Tooltip,
  TooltipTrigger,
  TooltipContent,
} from "@/components/ui/tooltip";
import { CircleAlert, ChevronUp } from "lucide-vue-next";
import IndividualHistogram from "@/components/charts/IndividualHistogram.vue";
import ResultEntryField from "./ResultEntryField.vue";
import { formatPercentage } from "@/lib/utils";

const lowDataWarningThreshold = 12 as const;

const { result, color, numSimulations, event, wcaId } = defineProps<{
  result: SimulationResult;
  color: string;
  numSimulations: number;
  event: SupportedWCAEvent;
  wcaId: string;
}>();

const model = defineModel<number[]>({ required: true });

const isOpen = ref<boolean>(false);

const winPercentage = computed(() =>
  formatPercentage((result.win_count * 100) / numSimulations),
);
const podiumPercentage = computed(() =>
  formatPercentage((result.pod_count * 100) / numSimulations),
);
const expectedRank = computed(() => result.total_rank / numSimulations);
</script>

<template>
  <Collapsible v-model:open="isOpen">
    <CollapsibleTrigger as-child>
      <div
        class="flex justify-between p-2 ps-1 cursor-pointer hover:bg-secondary rounded-md"
      >
        <div class="flex-1 text-left">
          <div class="flex flex-row">
            <div class="flex flex-col justify-center mx-2">
              <ColoredCircle :color />
            </div>
            <a
              :href="`https://worldcubeassociation.org/persons/${wcaId}`"
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
          {{ winPercentage }}
        </div>
        <div class="flex-1 text-center">
          {{ podiumPercentage }}
        </div>
        <div class="flex-1 text-center">
          {{ expectedRank }}
        </div>
        <ChevronUp
          class="scale-75 transition-transform duration-450"
          :class="{ '-rotate-180': isOpen }"
        />
      </div>
    </CollapsibleTrigger>
    <CollapsibleContent class="space-y-2">
      <IndividualHistogram
        :color="color"
        :histSingle="result.hist_values_single"
        :histAverage="result.hist_values_average"
        :simulations="numSimulations * eventAttempts[event]"
        :event
      />
      <div class="flex flex-col lg:flex-row items-center px-2 lg:gap-4 lg:ms-2">
        <div
          v-for="attemptIdx in eventAttempts[event]"
          v-bind:key="attemptIdx"
          class="flex mb-2 lg:mb-0 items-center gap-2"
        >
          <span class="whitespace-nowrap">Attempt {{ attemptIdx }}:</span>
          <div class="lg:max-w-24">
            <ResultEntryField :event v-model="model[attemptIdx - 1]" />
          </div>
        </div>
      </div>
      <hr class="mx-2" />
    </CollapsibleContent>
  </Collapsible>
</template>
