<script setup lang="ts">
import { useRouter } from "vue-router";
import { onMounted, ref, watch, onUnmounted, computed } from "vue";
import {
  eventAttempts,
  eventNames,
  SimulationResult,
  SupportedWCAEvent,
} from "@/lib/types";
import { generateColors } from "@/lib/utils";
import {
  runSimulationInWorker,
  terminateSimulationWorker,
} from "@/lib/simulationWorkerService";
import ExpandableBox from "@/components/custom/ExpandableBox.vue";
import FullHistogram from "@/components/charts/FullHistogram.vue";
import RankHistogram from "@/components/charts/RankHistogram.vue";
import LoadingMessage from "@/components/custom/LoadingMessage.vue";
import CompetitionHeader from "@/components/custom/CompetitionHeader.vue";
import ErrorDisplay from "@/components/custom/CompetitionError.vue";
import ResultsSummary from "@/components/custom/ResultsSummary.vue";
import CompetitorList from "@/components/custom/CompetitorList.vue";
import { Button } from "@/components/ui/button";
import { Icon } from "@iconify/vue";

const router = useRouter();
const { competitors, eventId, name, simCount, monthCutoff, includeDNFFlag } =
  router.currentRoute.value.query;

if (!competitors || !eventId || !name || !simCount || !monthCutoff) {
  throw new Error("One or more required parameters are null or undefined.");
}

const includeDNF = (includeDNFFlag ?? false) as boolean;
const competitorsList = competitors.toString().split(",");
const numSimulations = parseInt(simCount as string);
const monthCutoffNum = parseInt(monthCutoff.toString());
const event = eventId as SupportedWCAEvent;
const colors = generateColors(competitorsList.length);

const error = ref<string>("");
const simulation_results = ref<SimulationResult[] | null>(null);
const loading = ref<boolean>(true);
const recalculateLoading = ref<boolean>(false);
const inputtedTimesModified = ref<boolean>(false);
const selected = ref<boolean[]>(new Array(competitorsList?.length).fill(true));

const attemptsCount = computed(() => eventAttempts[event]);

const inputtedTimes = ref<number[][]>(
  Array.from({ length: competitorsList.length }, () =>
    Array.from({ length: attemptsCount.value + 1 }, () => 0),
  ),
);

watch(
  inputtedTimes,
  () => {
    inputtedTimesModified.value = true;
  },
  { deep: true },
);

const runSimulation = async () => {
  try {
    return await runSimulationInWorker(
      competitorsList,
      event,
      monthCutoffNum,
      numSimulations,
      includeDNF,
      inputtedTimes.value,
    );
  } catch (err) {
    console.error("Error in simulation:", err);
    error.value = err instanceof Error ? err.message : "Unknown error occurred";
    return null;
  }
};

onMounted(async () => {
  try {
    if (!(event in eventNames)) {
      error.value = "Invalid event ID";
      return;
    }

    simulation_results.value = await runSimulation();
  } finally {
    loading.value = false;
  }
});

onUnmounted(() => {
  terminateSimulationWorker();
});

const recalculate = async () => {
  recalculateLoading.value = true;

  try {
    const results = await runSimulation();
    if (results) {
      simulation_results.value = results;
      inputtedTimesModified.value = false;
    }
  } finally {
    recalculateLoading.value = false;
  }
};
</script>

<template>
  <div class="flex flex-col items-center justify-center mb-2">
    <CompetitionHeader :name="name as string" />

    <div v-if="error">
      <ErrorDisplay :error="error" />
    </div>

    <div v-else-if="simulation_results" class="min-w-[70vw]">
      <ResultsSummary
        :simulation-results="simulation_results"
        :colors="colors"
        :num-simulations="numSimulations"
        :event="event"
      />

      <ExpandableBox title="Results Histogram" class="mb-2">
        <FullHistogram
          :data="simulation_results"
          :colors="colors"
          :simulations="numSimulations * attemptsCount"
          :key="selected.filter(Boolean).length"
        />
      </ExpandableBox>

      <ExpandableBox title="Predicted Ranks">
        <RankHistogram
          :data="simulation_results"
          :colors="colors"
          :count="numSimulations"
          :key="selected.filter(Boolean).length"
        />
      </ExpandableBox>

      <CompetitorList
        :simulation-results="simulation_results"
        :colors="colors"
        :competitors-list="competitorsList"
        :num-simulations="numSimulations"
        :event="event"
        v-model="inputtedTimes"
      />

      <div
        class="fixed bottom-4 right-4 z-50 transition-opacity duration-300 ease-in-out"
        :class="{
          'opacity-0': !inputtedTimesModified,
          'opacity-100': inputtedTimesModified,
        }"
      >
        <Button
          @click="recalculate"
          class="shadow-lg"
          :disabled="recalculateLoading"
        >
          {{ recalculateLoading ? "Calculating..." : "Recalculate" }}
          <Icon
            v-if="recalculateLoading"
            icon="svg-spinners:180-ring"
            class="ml-2"
            width="24"
            height="24"
          />
        </Button>
      </div>
    </div>

    <div v-else class="mt-4">
      <LoadingMessage message="Calculating" />
    </div>
  </div>
</template>
