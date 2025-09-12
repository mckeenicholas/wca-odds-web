<script setup lang="ts">
import IndividualHistogram from "@/components/charts/IndividualHistogram.vue";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import {
  eventAttempts,
  SimulationResult,
  SupportedWCAEvent,
} from "@/lib/types";
import { formatPercentage } from "@/lib/utils";
import { CircleAlert } from "lucide-vue-next";
import { computed, ref } from "vue";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "../ui/collapsible";
import ColoredCircle from "./ColoredCircle.vue";
import FMCEntryField from "./FMCEntryField.vue";
import Chevron from "./RotatableChevron.vue";
import ResultEntryField from "./TimeEntryField.vue";

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

const ariaId = computed(() => `dropdown-${wcaId}`);
</script>

<template>
  <Collapsible v-model:open="isOpen">
    <CollapsibleTrigger as-child :aria-controls="ariaId">
      <button
        type="button"
        :aria-label="`Details for ${wcaId}`"
        class="flex w-full cursor-pointer justify-between rounded-md border-0 bg-transparent p-2 ps-1 text-left hover:bg-secondary focus:outline-none focus-visible:bg-secondary"
      >
        <div class="flex-1 text-left">
          <div class="flex flex-row">
            <div class="mx-2 flex flex-col justify-center">
              <ColoredCircle :color />
            </div>
            <a
              :href="`https://worldcubeassociation.org/persons/${wcaId}?event=${event}`"
              @click.stop
              class="hover:underline"
            >
              {{ result.name }}
            </a>
            <TooltipProvider
              :delayDuration="250"
              v-if="result.sample_size < lowDataWarningThreshold"
            >
              <Tooltip>
                <TooltipTrigger aria-label="Low result warning tooltip">
                  <CircleAlert class="ms-1 scale-75 text-red-600" />
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
        <Chevron :up="isOpen" />
      </button>
    </CollapsibleTrigger>
    <CollapsibleContent class="space-y-2" :id="ariaId">
      <IndividualHistogram
        :color="color"
        :histSingle="result.hist_values_single"
        :histAverage="result.hist_values_average"
        :simulations="numSimulations * eventAttempts[event]"
        :event
      />
      <div class="flex flex-col items-center px-2 lg:ms-2 lg:flex-row lg:gap-4">
        <div
          v-for="attemptIdx in eventAttempts[event]"
          v-bind:key="attemptIdx"
          class="mb-2 flex items-center gap-2 lg:mb-0"
        >
          <span class="whitespace-nowrap">Attempt {{ attemptIdx }}:</span>
          <div class="lg:max-w-24">
            <ResultEntryField
              v-if="event != '333fm'"
              v-model="model[attemptIdx - 1]"
            />
            <FMCEntryField v-else v-model="model[attemptIdx - 1]" />
          </div>
        </div>
      </div>
      <hr class="mx-2" />
    </CollapsibleContent>
  </Collapsible>
</template>
