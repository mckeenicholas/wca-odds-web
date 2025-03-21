<script setup lang="ts">
import { useRouter } from "vue-router";
import { onMounted, ref } from "vue";
import init, { load_data, run_odds_simulation } from "../../wasm/odds_web.js";
import {
  eventAttempts,
  eventNames,
  SimulationResult,
  SupportedWCAEvent,
} from "@/lib/types";
import { generateColors } from "@/lib/utils";
import ExpandableBox from "@/components/custom/ExpandableBox.vue";
import FullHistogram from "@/components/charts/FullHistogram.vue";
import RankHistogram from "@/components/charts/RankHistogram.vue";
import LoadingMessage from "@/components/custom/LoadingMessage.vue";
import CompetitionHeader from "@/components/custom/CompetitionHeader.vue";
import ErrorDisplay from "@/components/custom/CompetitionError.vue";
import ResultsSummary from "@/components/custom/ResultsSummary.vue";
import CompetitorList from "@/components/custom/CompetitorList.vue";

const router = useRouter();

const { competitors, eventId, name, simCount, monthCutoff, includeDNFFlag } =
  router.currentRoute.value.query;

const includeDNF = (includeDNFFlag ?? false) as boolean;

const error = ref<string>("");

if (!competitors || !eventId || !name || !simCount || !monthCutoff) {
  throw new Error("One or more required parameters are null or undefined.");
}

const competitorsList = competitors?.toString().split(",");
const numSimulations = parseInt(simCount as string);
const colors = generateColors(competitorsList.length);

const simulation_results = ref<SimulationResult[] | null>(null);
const loading = ref<boolean>(true);
const selected = ref<boolean[]>(new Array(competitorsList?.length).fill(true));

const inputtedTimes = ref<number[][]>(
  new Array(competitorsList?.length).fill(
    new Array(eventAttempts[eventId as SupportedWCAEvent]).fill(0),
  ),
);

onMounted(async () => {
  if (!((eventId as string) in eventNames)) {
    error.value = "Invalid event ID";
    return;
  }

  const startDate = new Date();
  startDate.setMonth(startDate.getMonth() - parseInt(monthCutoff.toString()));

  const event = eventId as SupportedWCAEvent;

  await init();
  const result = await load_data(
    competitorsList,
    event,
    parseInt(monthCutoff.toString()),
  );

  if (!result) {
    console.error("Error fetching data");
    error.value = "Error fetching data";
    return;
  }

  simulation_results.value = run_odds_simulation(numSimulations, includeDNF);

  loading.value = false;
});

const recalculate = () => {
  console.log("lol");
  simulation_results.value = run_odds_simulation(numSimulations, includeDNF);
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
        :event="eventId as SupportedWCAEvent"
      />

      <ExpandableBox title="Results Histogram" class="mb-2">
        <FullHistogram
          :data="simulation_results"
          :colors="colors"
          :simulations="numSimulations"
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
        :event="eventId as SupportedWCAEvent"
        v-model="inputtedTimes"
      />

      <button @click="recalculate">Recalculate</button>
    </div>

    <div v-else class="mt-4">
      <LoadingMessage message="Calculating" />
    </div>
  </div>
</template>
