<script setup lang="ts">
import { useRouter } from "vue-router";
import { onMounted, ref } from "vue";
import init, { run_odds_simulation } from "../../wasm/odds_web.js";
import { eventNames, SimulationResult, SupportedWCAEvent } from "@/lib/types";
import { generateColors } from "@/lib/histogram";
import Expandable from "@/components/custom/Expandable.vue";
import FullHistogram from "@/components/charts/FullHistogram.vue";
import RankHistogram from "@/components/charts/RankHistogram.vue";
import LoadingMessage from "@/components/custom/LoadingMessage.vue";
import CompetitionHeader from "@/components/custom/CompetitionHeader.vue";
import ErrorDisplay from "@/components/custom/CompetitionError.vue";
import ResultsSummary from "@/components/custom/ResultsSummary.vue";
import CompetitorList from "@/components/custom/CompetitorList.vue";

const router = useRouter();

const { competitors, eventId, name, simCount, monthCutoff } =
  router.currentRoute.value.query;

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

onMounted(async () => {
  if (!((eventId as string) in eventNames)) {
    error.value = "Invalid event ID";
    return;
  }

  const startDate = new Date();
  startDate.setMonth(startDate.getMonth() - parseInt(monthCutoff.toString()));

  const event = eventId as SupportedWCAEvent;

  await init();
  simulation_results.value = await run_odds_simulation(
    competitorsList,
    event,
    parseInt(monthCutoff.toString()),
    numSimulations,
  );

  loading.value = false;
});
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
      />

      <Expandable title="Results Histogram" class="mb-2">
        <FullHistogram
          :data="simulation_results"
          :colors="colors"
          :simulations="numSimulations"
          :key="selected.filter(Boolean).length"
        />
      </Expandable>
      <Expandable title="Predicted Ranks">
        <RankHistogram
          :data="simulation_results"
          :colors="colors"
          :count="numSimulations"
          :key="selected.filter(Boolean).length"
        />
      </Expandable>

      <CompetitorList
        :simulation-results="simulation_results"
        :colors="colors"
        :competitors-list="competitorsList"
        :num-simulations="numSimulations"
      />
    </div>

    <div v-else class="mt-4">
      <LoadingMessage message="Calculating" />
    </div>
  </div>
</template>
