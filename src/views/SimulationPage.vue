<script setup lang="ts">
import { useRouter, RouteParams, useRoute } from "vue-router";
import { onMounted, ref, onUnmounted, computed } from "vue";
import {
  eventAttempts,
  eventNames,
  SimulationResult,
  SimulationRouteQuery,
  SupportedWCAEvent,
} from "@/lib/types";
import {
  generateColors,
  generateDefaultTimesArray,
  getParentPath,
  ArrEq2D,
  clone2DArr,
  createJSONExport,
} from "@/lib/utils";
import {
  runSimulationInWorker,
  recalculateSimulationInWorker,
  terminateSimulationWorker,
} from "@/lib/simulationWorkerService";
import ExpandableBox from "@/components/custom/ExpandableBox.vue";
import FullHistogram from "@/components/charts/FullHistogram.vue";
import RankHistogram from "@/components/charts/RankHistogram.vue";
import LoadingMessage from "@/components/custom/LoadingMessage.vue";
import ErrorDisplay from "@/components/custom/ErrorPanel.vue";
import CompetitorList from "@/components/custom/CompetitorList.vue";
import ResultsSummary from "@/components/custom/ResultsSummary.vue";
import { Button } from "@/components/ui/button";
import { LoaderCircle } from "lucide-vue-next";
import fetchWCALiveResults from "@/lib/wcaLive";

const router = useRouter();
const currentVueRoute = useRoute();
const path = currentVueRoute.path;

const queryParams = currentVueRoute.query as SimulationRouteQuery & RouteParams;

const {
  competitors: competitorsParam,
  eventId: eventIdParam,
  name: nameParam,
  simCount: simCountParam,
  startDate: startDateParam,
  endDate: endDateParam,
  includeDnf: includeDnfParam,
  decayRate: decayRateParam,
  competitionId: competitionIdParam,
  date: competitionDateParam,
} = queryParams;

if (
  !competitorsParam ||
  !eventIdParam ||
  !nameParam ||
  !simCountParam ||
  !startDateParam ||
  !includeDnfParam ||
  !decayRateParam ||
  !(eventIdParam in eventNames)
) {
  router.push(getParentPath(path));
}

const name = nameParam!;
const numSimulations = parseInt(simCountParam!);
const startDate = new Date(startDateParam!);
const endDate = endDateParam ? new Date(endDateParam) : new Date();
const decayHalfLife = parseInt(decayRateParam!);
const competitorsList = competitorsParam!.split(",");
const includeDNF = includeDnfParam === "true";
const event = eventIdParam! as SupportedWCAEvent;
const colors = generateColors(competitorsList.length);

const attemptsCount = eventAttempts[event];
const defaultTimesArray = generateDefaultTimesArray(
  competitorsList.length,
  attemptsCount,
);

const error = ref<string>("");
const simulationResults = ref<SimulationResult[] | null>(null);
const loading = ref<boolean>(true);
const recalculateLoading = ref<boolean>(false);
const wcaLiveLoading = ref<boolean>(false);
const inputtedTimes = ref<number[][]>(clone2DArr(defaultTimesArray));
const inputtedTimesPrev = ref<number[][]>(clone2DArr(defaultTimesArray));

const inputtedTimesState = computed(() => {
  const hasNonZero = inputtedTimes.value.some((competitor: number[]) =>
    competitor.some((time) => time !== 0),
  );
  const isModified = !ArrEq2D(inputtedTimes.value, inputtedTimesPrev.value);

  return { hasNonZero, isModified };
});

const sharedProps = computed(() => ({
  data: simulationResults.value ?? [],
  colors,
  numSimulations,
  event,
}));

const runInitialSimulation = async () => {
  try {
    const results = await runSimulationInWorker(
      competitorsList,
      event,
      startDate,
      endDate,
      numSimulations,
      includeDNF,
      decayHalfLife,
      inputtedTimes.value,
    );

    if (results) {
      simulationResults.value = results;
      inputtedTimesPrev.value = clone2DArr(inputtedTimes.value);
    }
  } catch (err) {
    terminateSimulationWorker();
    console.error("Error in initial simulation:", err);
    error.value =
      err instanceof Error
        ? err.message
        : "Unknown error occurred during initial simulation";
  }
};

const handleRecalculation = async () => {
  try {
    const results = await recalculateSimulationInWorker(
      numSimulations,
      includeDNF,
      inputtedTimes.value,
    );

    if (results) {
      simulationResults.value = results;
      inputtedTimesPrev.value = clone2DArr(inputtedTimes.value);
    }
  } catch (err) {
    console.error("Error in recalculation:", err);
    error.value =
      err instanceof Error
        ? err.message
        : "Unknown error occurred during recalculation";
  }
};

onMounted(async () => {
  loading.value = true;
  await runInitialSimulation();
  loading.value = false;
});

onUnmounted(() => {
  terminateSimulationWorker();
});

const recalculate = async () => {
  recalculateLoading.value = true;
  error.value = "";
  await handleRecalculation();
  recalculateLoading.value = false;
};

const reset = async () => {
  inputtedTimes.value = clone2DArr(defaultTimesArray);
  await recalculate();
};

const syncResultsWithWCALive = async () => {
  if (!showWCALiveImport()) {
    return;
  }

  wcaLiveLoading.value = true;
  try {
    const results = await fetchWCALiveResults(
      competitionIdParam!,
      event,
      competitorsList,
    );
    inputtedTimes.value = results;
    await recalculate();
  } catch (err) {
    console.error(err);
    // For now we don't set the error ref as it doesn't prevent the simulation from running
    // TODO: add a Toast component for this
  } finally {
    wcaLiveLoading.value = false;
  }
};

const showWCALiveImport = () => {
  if (!competitionIdParam || !competitionDateParam) {
    return false;
  }

  const today = new Date();
  const competitionDate = new Date(competitionDateParam);

  if (competitionDate > today) {
    return false;
  }

  // WCA Live competitions are archived after 90 days.
  const removalCutoff = new Date();
  removalCutoff.setDate(today.getDate() - 90);

  return competitionDate > removalCutoff;
};

const exportJson = () => {
  const jsonText = createJSONExport({
    competitionName: name,
    results: simulationResults.value!,
    ids: competitorsList,
    currentTimes: inputtedTimes.value,
    startDate,
    endDate,
    simCount: numSimulations,
    decayRate: decayHalfLife,
    includeDnf: includeDNF,
    event,
  });

  const blob = new Blob([jsonText], { type: "application/json" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = `${name}_results.json`;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
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
      v-else-if="simulationResults"
      class="lg:min-w-[1000px] md:min-w-full border-lg"
    >
      <ResultsSummary
        :data="simulationResults || []"
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
        :simulation-results="simulationResults"
        :colors
        :competitors-list="competitorsList"
        :num-simulations="numSimulations"
        :event
        v-model="inputtedTimes"
      />

      <p class="text-muted-foreground m-2">
        Export as:
        <a
          role="button"
          class="underline hover:text-gray-300 me-2"
          @click="exportJson()"
          >json</a
        >
        <a role="button" class="underline hover:text-gray-300">csv</a>
      </p>

      <div class="fixed bottom-4 right-2 z-50 flex">
        <Transition name="fade">
          <Button
            @click="syncResultsWithWCALive"
            class="me-2"
            v-if="showWCALiveImport()"
            :disabled="wcaLiveLoading || recalculateLoading"
          >
            Import Current Results From WCA Live
            <LoaderCircle v-show="wcaLiveLoading" class="animate-spin" />
          </Button>
        </Transition>
        <Transition name="fade">
          <Button
            @click="recalculate"
            class="me-2"
            :disabled="recalculateLoading || !inputtedTimesState.isModified"
            v-if="inputtedTimesState.isModified"
          >
            {{ recalculateLoading ? "Recalculating..." : "Recalculate" }}
            <LoaderCircle v-show="recalculateLoading" class="animate-spin" />
          </Button>
        </Transition>
        <Transition name="fade">
          <Button
            @click="reset"
            v-if="inputtedTimesState.hasNonZero"
            :disabled="recalculateLoading"
          >
            Reset
          </Button>
        </Transition>
      </div>
    </div>

    <div v-else class="mt-4">
      <LoadingMessage message="Calculating" />
    </div>
  </div>
</template>
