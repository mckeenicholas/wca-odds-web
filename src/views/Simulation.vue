<script setup lang="ts">
import { useRouter } from "vue-router";
import { onMounted, ref } from "vue";
import init, { simulate } from "../../wasm/odds_web.js";
import { fetchWCAInfo } from "../lib/utils.js";
import { eventInfo, WCAevent } from "../lib/types.js";
import IndividualHistogram from "../components/charts/IndividualHistogram.vue";
import { generateColors } from "../lib/histogram.js";
import FullHistogram from "../components/charts/FullHistogram.vue";
import RankHistogram from "../components/charts/RankHistogram.vue";
import PieChart from "../components/charts/PieChart.vue";
import { Icon } from "@iconify/vue";
import Expandable from "../components/custom/Expandable.vue";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "../components/ui/collapsible";

const router = useRouter();

const { competitors, event, name, simCount, monthCutoff } =
  router.currentRoute.value.query;

if (!competitors || !event || !name || !simCount || !monthCutoff) {
  throw new Error("One or more required parameters are null or undefined.");
}

const competitorsList = competitors?.toString().split(",");

const simulation_results = ref<any>(null);
const loading = ref<boolean>(true);
const colors = ref<string[]>([""]);
const bounds = ref<{ max: number; min: number }>({ max: 0, min: 100 });
const selected = ref<boolean[]>(new Array(competitorsList.length).fill(true));

type Competition = {
  results: any;
  id: string;
  date: string;
};

type Competitor = {
  name: any;
  id: any;
  results: { [key: string]: any };
};

const getYearsFromDate = (startDate: Date) => {
  let today = new Date();

  let years = [];
  for (
    let year = startDate.getFullYear();
    year <= today.getFullYear();
    year++
  ) {
    years.push(year);
  }

  return years;
};

const filterCompetitions = (
  competitions: Competition[],
  competitors: Competitor[],
  event: WCAevent,
  startDate: Date,
) => {
  const numSolves = eventInfo[event].attempts;
  const compDate: { [key: string]: string } = {};

  competitions.forEach((comp) => {
    compDate[comp.id] = comp.date;
  });

  const results = competitors.flatMap((person) => {
    const results = Object.entries(person.results)
      .filter(
        ([key, values]) => new Date(compDate[key]) > startDate && values[event],
      )
      .flatMap(([_, values]) =>
        values[event]?.flatMap((round: { solves: number[] }) =>
          round.solves.slice(0, numSolves).filter((solve) => solve !== 0),
        ),
      );
    return results.length
      ? [{ id: person.id, name: person.name, results: results }]
      : [];
  });
  return results;
};

const fetchData = async (
  persons: string[],
  event: WCAevent,
  startDate: Date,
) => {
  const years = getYearsFromDate(startDate);

  const results = await Promise.all(
    persons.map(
      async (person) =>
        await fetchWCAInfo(
          `https://raw.githubusercontent.com/robiningelbrecht/wca-rest-api/master/api/persons/${person}.json`,
        ),
    ),
  );

  const competitions = (
    await Promise.all(
      years.map(async (year) => {
        const response = await fetchWCAInfo<{ items: any[] }>(
          `https://raw.githubusercontent.com/robiningelbrecht/wca-rest-api/master/api/competitions/${year}.json`,
        );
        return response?.items.map((comp: { id: any; date: { from: any } }) => {
          return { id: comp.id, date: comp.date.from };
        });
      }),
    )
  ).flat();

  loading.value = false;
  return filterCompetitions(
    competitions as Competition[],
    results as Competitor[],
    event,
    startDate,
  );
};

const runSimulation = (
  results: { id: string; name: string; results: number[] }[],
  simCount: number,
  event: WCAevent,
) => {
  console.log(results);
  init().then(() => {
    const result_times = results.map((result) => result.results);
    const raw_results = simulate(
      { results: result_times },
      simCount,
      eventInfo[event].format,
    );
    simulation_results.value = raw_results.map((item: any, idx: number) => {
      const maxBound = (item.mu + 6 * item.sigma) / 100;
      const minBound = Math.max((item.mu - 4 * item.sigma) / 100, 0);

      if (!isNaN(maxBound)) {
        bounds.value.max = Math.max(maxBound, bounds.value.max);
      }

      if (!isNaN(minBound)) {
        bounds.value.min = Math.min(minBound, bounds.value.min);
      }

      return {
        name: results[idx].name,
        id: results[idx].id,
        wins: parseFloat(((item.wins / simCount) * 100).toFixed(2)),
        podiums: parseFloat(((item.podiums / simCount) * 100).toFixed(2)),
        mean: parseFloat((item.mean / 100).toFixed(4)),
        stdev: parseFloat((item.stdev / 100).toFixed(4)),
        gamma: parseFloat((item.gamma / 100).toFixed(4)),
        mu: parseFloat((item.mu / 100).toFixed(4)),
        sigma: parseFloat((item.sigma / 100).toFixed(4)),
        tau: parseFloat((item.tau / 100).toFixed(4)),
        dnfRate: parseFloat(item.dnf_rate.toFixed(4)) * 100,
        avgRank: parseFloat(item.avg_rank.toFixed(4)),
        ranks: item.ranks,
      };
    });
    console.log(simulation_results.value);
    colors.value = generateColors(raw_results.length);
  });
};

onMounted(async () => {
  let startDate = new Date();
  startDate.setMonth(new Date().getMonth() - parseInt(monthCutoff.toString()));
  const data = await fetchData(competitorsList, event as WCAevent, startDate);
  runSimulation(data, parseInt(simCount.toString()), event as WCAevent);
});
</script>

<template>
  <div class="flex flex-col items-center justify-center mb-2">
    <h1 class="text-center text-2xl font-bold mt-4 mb-2">
      Results for {{ name }}
    </h1>
    <div v-if="simulation_results" class="min-w-[70vw]">
      <div class="flex">
        <div class="border rounded-md my-2 py-2 px-4 me-2 flex-grow">
          winner info here
        </div>
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
          :key="selected.filter((item) => item).length"
        />
      </Expandable>
      <Expandable title="Predicted Ranks">
        <RankHistogram
          :data="simulation_results"
          :colors="colors"
          :count="parseInt(simCount as string)"
          :key="selected.filter((item) => item).length"
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
                <div class="my-10 mx-6 text-lg flex flex-row space-x-4">
                  <div>m = {{ result.mean }}</div>
                  <div>s = {{ result.stdev }}</div>
                  <div>&gamma; = {{ result.gamma }}</div>
                  <div>&mu; = {{ result.mu }}</div>
                  <div>&sigma; = {{ result.sigma }}</div>
                  <div>&tau; = {{ result.tau }}</div>
                  <div>DNF rate = {{ result.dnfRate }}%</div>
                </div>
                <hr class="mx-2" />
              </CollapsibleContent>
            </Collapsible>
          </li>
        </ol>
      </div>
    </div>
    <div v-else-if="loading" class="mt-4">Fetching data...</div>
    <div v-else class="mt-4">Calculating...</div>
  </div>
</template>
