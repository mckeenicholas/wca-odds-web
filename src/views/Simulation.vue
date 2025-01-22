<script setup lang="ts">
import { useRouter } from "vue-router";
import { onMounted, ref } from "vue";
import init, { run_odds_simulation } from "../../wasm/odds_web.js";

import { fetchData } from "@/lib/utils.js";
import { eventInfo, SupportedWCAEvent } from "@/lib/types.js";
import IndividualHistogram from "@/components/charts/IndividualHistogram.vue";
import { generateColors } from "@/lib/histogram.js";
import FullHistogram from "@/components/charts/FullHistogram.vue";
import RankHistogram from "@/components/charts/RankHistogram.vue";
import PieChart from "@/components/charts/PieChart.vue";
import { Icon } from "@iconify/vue";
import Expandable from "@/components/custom/Expandable.vue";
import ResultInfo from "@/components/custom/ResultInfo.vue";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import LoadingMessage from "@/components/custom/LoadingMessage.vue";
import { eventNames } from "@/lib/types";
import { Button } from "@/components/ui/button";

const router = useRouter();

const { competitors, eventId, name, simCount, monthCutoff } =
  router.currentRoute.value.query;

const error = ref<string>("");

if (!competitors || !eventId || !name || !simCount || !monthCutoff) {
  throw new Error("One or more required parameters are null or undefined.");
}

const competitorsList = competitors?.toString().split(",");

const simulation_results = ref<any>(null);
const loading = ref<boolean>(true);
const colors = ref<string[]>([""]);
const bounds = ref<{ max: number; min: number }>({ max: 0, min: 100 });
const selected = ref<boolean[]>(new Array(competitorsList?.length).fill(true));

const runSimulation = async (
  results: { id: string; name: string; results: number[] }[],
  simCount: number,
  event: SupportedWCAEvent,
) => {

  const resultTimes = results.map((result) => result.results);

  await init()

  const rawResults: any[] = [] 
  // simulate(
  //   { results: resultTimes },
  //   simCount,
  //   eventInfo[event].format,
  // );

  updateSimulationResults(rawResults, results, simCount);
  colors.value = generateColors(rawResults.length);
};

const updateSimulationResults = (
  rawResults: any[],
  results: { id: string; name: string }[],
  simCount: number,
) => {
  simulation_results.value = rawResults
    .map((item, idx) => transformResult(item, results[idx], simCount))
    .sort((a, b) => b.wins - a.wins);
};

const transformResult = (
  item: any,
  result: { id: string; name: string },
  simCount: number,
) => {
  const maxBound = (item.mu + 6 * item.sigma) / 100;
  const minBound = Math.max((item.mu - 4 * item.sigma) / 100, 0);

  if (!isNaN(maxBound)) bounds.value.max = Math.max(maxBound, bounds.value.max);
  if (!isNaN(minBound)) bounds.value.min = Math.min(minBound, bounds.value.min);

  return {
    name: result.name,
    id: result.id,
    wins: toPercentage(item.wins, simCount),
    podiums: toPercentage(item.podiums, simCount),
    mean: toDecimal(item.mean / 100),
    stdev: toDecimal(item.stdev / 100),
    gamma: toDecimal(Math.max(item.gamma / 100, 0)),
    mu: toDecimal(item.mu / 100),
    sigma: toDecimal(item.sigma / 100),
    tau: toDecimal(Math.max(item.tau / 100, 0)),
    dnfRate: toDecimal(item.dnf_rate * 100),
    avgRank: toDecimal(item.avg_rank),
    ranks: item.ranks,
  };
};

const toPercentage = (value: number, total: number) =>
  parseFloat(((value / total) * 100).toFixed(2));

const toDecimal = (value: number) => parseFloat(value.toFixed(4));

const goBack = () => {
  window.history.back();
};

onMounted(async () => {
  if (!((eventId as string) in eventNames)) {
    error.value = "Invalid event ID";
    return;
  }

  const startDate = new Date();
  startDate.setMonth(startDate.getMonth() - parseInt(monthCutoff.toString()));

  const data = await fetchData(
    competitorsList,
    eventId as SupportedWCAEvent,
    startDate,
  );
  loading.value = false;

  if (data.length == 0) {
    error.value = `Nobody has results in ${eventNames[eventId as keyof typeof eventNames]}`;
  }

  await runSimulation(
    data,
    parseInt(simCount.toString()),
    eventId as SupportedWCAEvent,
  );
});
</script>

<template>
  <div class="flex flex-col items-center justify-center mb-2">
    <h1 class="text-center text-2xl font-bold mt-4 mb-2">
      Results for {{ name }}
    </h1>
    <div v-if="error" class="flex flex-col items-center justify-center">
      <p>Error: {{ error }}</p>
      <div class="mt-2">
        <Button :onclick="goBack"> Back </Button>
      </div>
    </div>
    <div v-else-if="simulation_results" class="min-w-[70vw]">
      <div class="flex">
        <p class="border rounded-md my-2 py-2 px-4 me-2 flex-grow">
          winner info here
        </p>
        <div class="border rounded-md my-2 py-2 px-4">
          <PieChart :data="simulation_results" :colors="colors" />
        </div>
      </div>
      <Expandable title="Results Histogram" class="mb-2">
        <FullHistogram
          :data="simulation_results"
          :min="bounds.min"
          :max="bounds.max"
          :colors="colors"
          :key="selected.filter(Boolean).length"
        />
      </Expandable>
      <Expandable title="Predicted Ranks">
        <RankHistogram
          :data="simulation_results"
          :colors="colors"
          :count="parseInt(simCount as string)"
          :key="selected.filter(Boolean).length"
        />
      </Expandable>
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
            v-for="(result, idx) in simulation_results"
            :key="result.id"
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
                          :style="{ color: colors[idx] }"
                        />
                      </div>
                      <a
                        :href="`https://worldcubeassociation.org/persons/${result.id}`"
                        @click.stop
                        class="hover:underline"
                      >
                        {{ result.name }}
                      </a>
                    </div>
                  </div>
                  <div class="flex-1 text-center">{{ result.wins }}%</div>
                  <div class="flex-1 text-center">{{ result.podiums }}%</div>
                  <div class="flex-1 text-center">{{ result.avgRank }}</div>
                </div>
              </CollapsibleTrigger>
              <CollapsibleContent class="space-y-2">
                <IndividualHistogram
                  :mu="result.mu"
                  :sigma="result.sigma"
                  :tau="result.tau"
                  :color="colors[idx]"
                  :min="bounds.min"
                  :max="bounds.max"
                  class="border rounded-md m-2 p-2"
                />
                <ResultInfo v-bind="result" />
                <hr class="mx-2" />
              </CollapsibleContent>
            </Collapsible>
          </li>
        </ol>
      </div>
    </div>
    <div v-else-if="loading" class="mt-4">
      <LoadingMessage message="Fetching data" />
    </div>
    <div v-else class="mt-4">
      <LoadingMessage message="Calculating results" />
    </div>
  </div>
</template>
