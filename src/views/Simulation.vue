<script setup lang="ts">
import { useRouter } from "vue-router";
import { onMounted, ref } from "vue";
import init, { run_odds_simulation } from "../../wasm/odds_web.js";

import IndividualHistogram from "@/components/charts/IndividualHistogram.vue";
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
import { eventNames, SimulationResult, SupportedWCAEvent } from "@/lib/types";
import { Button } from "@/components/ui/button";
import { generateColors } from "@/lib/histogram";

const router = useRouter();

const { competitors, eventId, name, simCount, monthCutoff } =
  router.currentRoute.value.query;

const error = ref<string>("");

if (!competitors || !eventId || !name || !simCount || !monthCutoff) {
  throw new Error("One or more required parameters are null or undefined.");
}

const competitorsList = competitors?.toString().split(",");
const numSimulations = parseInt(simCount as string);
const colors = generateColors(competitorsList.length)

const simulation_results = ref<SimulationResult[] | null>(null);
const loading = ref<boolean>(true);
const selected = ref<boolean[]>(new Array(competitorsList?.length).fill(true));

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

  const event = eventId as SupportedWCAEvent;

  await init();
  simulation_results.value = await run_odds_simulation(competitorsList, event, parseInt(monthCutoff as string), numSimulations);

  console.log(simulation_results);

  loading.value = false;
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
          <!-- <PieChart :data="simulation_results" :colors="colors" /> -->
        </div>
      </div>
      <Expandable title="Results Histogram" class="mb-2">
        <FullHistogram
          :data="simulation_results"
          :colors="colors"
          :simulations="numSimulations"
          :names="competitorsList"
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
            :key="idx"
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
                        :href="`https://worldcubeassociation.org/persons/${competitorsList[idx]}`"
                        @click.stop
                        class="hover:underline"
                      >
                        {{ competitorsList[idx] }}
                      </a>
                    </div>
                  </div>
                  <div class="flex-1 text-center">{{ result.win_count * 100 / numSimulations }}%</div>
                  <div class="flex-1 text-center">{{ result.pod_count * 100 / numSimulations }}%</div>
                  <div class="flex-1 text-center">{{ result.total_rank / numSimulations }}</div>
                </div>
              </CollapsibleTrigger>
              <CollapsibleContent class="space-y-2">
                <IndividualHistogram
                  :color="colors[idx]"
                  :hist="result.hist_values"
                  :simulations="numSimulations"
                  class="border rounded-md m-2 p-2"
                />
                <!-- Will probably add this back later -->
                <!-- <ResultInfo v-bind="result" /> -->
                <hr class="mx-2" />
              </CollapsibleContent>
            </Collapsible>
          </li>
        </ol>
      </div>
    </div>
    <div v-else class="mt-4">
      <LoadingMessage message="Calculating" />
    </div>
  </div>
</template>
