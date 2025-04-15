<script setup lang="ts">
import { useRouter, RouteParams } from "vue-router";
import { onMounted, ref, onUnmounted, computed } from "vue";
import { cloneDeep, isEqual } from "lodash-es";
import {
  eventAttempts,
  eventNames,
  SimulationResult,
  SimulationRouteQuery,
  SupportedWCAEvent,
} from "@/lib/types";
import { generateColors, generateDefaultTimesArray } from "@/lib/utils";
import {
  runSimulationInWorker,
  terminateSimulationWorker,
} from "@/lib/simulationWorkerService";
import ExpandableBox from "@/components/custom/ExpandableBox.vue";
import FullHistogram from "@/components/charts/FullHistogram.vue";
import RankHistogram from "@/components/charts/RankHistogram.vue";
import LoadingMessage from "@/components/custom/LoadingMessage.vue";
import ErrorDisplay from "@/components/custom/CompetitionError.vue";
import CompetitorList from "@/components/custom/CompetitorList.vue";
import ResultsSummary from "@/components/custom/ResultsSummary.vue";
import { Button } from "@/components/ui/button";
import { LoaderCircle } from "lucide-vue-next";

const route = useRouter().currentRoute.value as {
  query: SimulationRouteQuery & RouteParams;
};
const { competitors, eventId, name, simCount, monthCutoff, includeDnf } =
  route.query;

if (!competitors || !eventId || !name || !simCount || !monthCutoff) {
  throw new Error("One or more required parameters are null or undefined.");
}

if (!(eventId in eventNames)) {
  throw new Error("Invalid event ID");
}

const includeDNF = includeDnf === "true";
const competitorsList = competitors.toString().split(",");
const numSimulations = parseInt(simCount as string);
const monthCutoffNum = parseInt(monthCutoff.toString());
const event = eventId as SupportedWCAEvent;
const colors = generateColors(competitorsList.length);

const attemptsCount = eventAttempts[event];
const defaultTimesArray = generateDefaultTimesArray(
  competitorsList.length,
  attemptsCount,
);

const error = ref<string>("");
const simulation_results = ref<SimulationResult[] | null>(null);
const loading = ref<boolean>(true);
const recalculateLoading = ref<boolean>(false);
const inputtedTimes = ref<number[][]>(cloneDeep(defaultTimesArray));
const inputtedTimesPrev = ref<number[][]>(cloneDeep(defaultTimesArray));

const inputtedTimesState = computed(() => {
  const hasNonZero = inputtedTimes.value.some((competitor) =>
    competitor.some((time) => time !== 0),
  );
  const isModified = !isEqual(inputtedTimes.value, inputtedTimesPrev.value);

  return { hasNonZero, isModified };
});

const sharedProps = computed(() => ({
  data: simulation_results.value ?? [],
  colors,
  numSimulations,
  event,
}));

const runSimulation = async () => {
  try {
    const results = await runSimulationInWorker(
      competitorsList,
      event,
      monthCutoffNum,
      numSimulations,
      includeDNF,
      inputtedTimes.value,
    );

    if (results) {
      inputtedTimesPrev.value = cloneDeep(inputtedTimes.value);
    }

    return results;
  } catch (err) {
    terminateSimulationWorker();
    console.error("Error in simulation:", err);
    error.value = err instanceof Error ? err.message : "Unknown error occurred";
    return null;
  }
};

const handleSimulation = async () => {
  try {
    const results = await runSimulation();
    if (results) {
      simulation_results.value = results;
      inputtedTimesPrev.value = cloneDeep(inputtedTimes.value);
    }
  } catch (err) {
    error.value = err instanceof Error ? err.message : "Unknown error occurred";
  }
};

onMounted(async () => {
  loading.value = true;
  await handleSimulation();
  loading.value = false;
});

onUnmounted(() => {
  terminateSimulationWorker();
});

const recalculate = async () => {
  recalculateLoading.value = true;
  error.value = "";
  await handleSimulation();
  recalculateLoading.value = false;
};

const reset = () => {
  inputtedTimes.value = cloneDeep(defaultTimesArray);
  recalculate();
};
</script>

<template>
  <div class="content-main flex flex-col items-center justify-center mx-2">
    <h1 class="text-center text-2xl font-bold mt-4 mb-4">
      Results for {{ name }}
    </h1>

    <div v-if="!!error">
      <ErrorDisplay :error />
    </div>

    <div
      v-else-if="simulation_results"
      class="lg:min-w-[1000px] md:min-w-full border-lg"
    >
      <ResultsSummary
        :data="simulation_results || []"
        :colors="colors"
        :numSimulations="numSimulations"
        :event="event"
      />

      <ExpandableBox title="Results Histogram" class="mb-2">
        <FullHistogram v-bind="sharedProps" />
      </ExpandableBox>

      <ExpandableBox title="Predicted Ranks">
        <RankHistogram v-bind="sharedProps" />
      </ExpandableBox>
      <CompetitorList
        :simulation-results="simulation_results"
        :colors
        :competitors-list="competitorsList"
        :num-simulations="numSimulations"
        :event
        v-model="inputtedTimes"
      />

      <div class="fixed bottom-4 right-4 z-50 flex">
        <Transition name="fade">
          <Button
            @click="reset"
            class="me-2"
            v-if="inputtedTimesState.hasNonZero"
          >
            Reset
          </Button>
        </Transition>
        <Transition name="fade">
          <Button
            @click="recalculate"
            :disabled="recalculateLoading"
            v-if="inputtedTimesState.isModified"
          >
            {{ recalculateLoading ? "Recalculating..." : "Recalculate" }}
            <LoaderCircle v-show="recalculateLoading" class="animate-spin" />
          </Button>
        </Transition>
      </div>
    </div>

    <div v-else class="mt-4">
      <LoadingMessage message="Calculating" />
    </div>
  </div>
</template>
