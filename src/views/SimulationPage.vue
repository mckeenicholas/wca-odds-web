<script setup lang="ts">
import { useRouter } from "vue-router";
import { onMounted, ref, onUnmounted, computed } from "vue";
import { cloneDeep, isEqual } from "lodash-es";
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
import CompetitorList from "@/components/custom/CompetitorList.vue";
import ResultsSummary from "@/components/custom/ResultsSummary.vue";
import { Button } from "@/components/ui/button";
import { LoaderCircle } from "lucide-vue-next";

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
const selected = ref<boolean[]>(new Array(competitorsList?.length).fill(true));

const attemptsCount = computed(() => eventAttempts[event]);

const inputtedTimes = ref<number[][]>(
  Array.from({ length: competitorsList.length }, () =>
    Array.from({ length: attemptsCount.value }, () => 0),
  ),
);

const inputtedTimesPrev = ref<number[][]>(
  Array.from({ length: competitorsList.length }, () =>
    Array.from({ length: attemptsCount.value }, () => 0),
  ),
);

const inputtedTimesModified = computed(() => {
  return !isEqual(inputtedTimes.value, inputtedTimesPrev.value);
});

const hasNonZeroTimes = computed(() => {
  return inputtedTimes.value.some((competitor) =>
    competitor.some((time) => time !== 0),
  );
});

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
    }
  } finally {
    recalculateLoading.value = false;
  }
};

const reset = () => {
  const newTimes = Array.from({ length: competitorsList.length }, () =>
    Array.from({ length: attemptsCount.value }, () => 0),
  );

  inputtedTimes.value.splice(0, inputtedTimes.value.length, ...newTimes);
  recalculate();
};
</script>

<template>
  <div class="content-main flex flex-col items-center justify-center mx-2">
    <CompetitionHeader :name="name as string" />

    <div v-if="error">
      <ErrorDisplay :error />
    </div>

    <div
      v-else-if="simulation_results"
      class="lg:min-w-[1000px] md:min-w-full sm:min-w-full border-lg"
    >
      <ResultsSummary
        :simulation-results="simulation_results"
        :colors
        :num-simulations="numSimulations"
        :event
      />

      <ExpandableBox title="Results Histogram" class="mb-2">
        <FullHistogram
          :data="simulation_results"
          :event
          :colors
          :key="selected.filter(Boolean).length"
        />
      </ExpandableBox>

      <ExpandableBox title="Predicted Ranks">
        <RankHistogram
          :data="simulation_results"
          :colors
          :count="numSimulations"
          :key="selected.filter(Boolean).length"
        />
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
          <Button @click="reset" class="me-2" v-if="hasNonZeroTimes"
            >Reset</Button
          >
        </Transition>
        <Transition name="fade">
          <Button
            @click="recalculate"
            :disabled="recalculateLoading"
            v-if="inputtedTimesModified"
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

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
