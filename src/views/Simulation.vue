<script setup lang="ts">
import { useRouter } from "vue-router";
import { onMounted, ref } from "vue";
import init, { simulate } from "../../build/odds_web.js";
import { fetchWCAInfo } from "@/lib/utils.js";
import { eventInfo, WCAevent } from "@/lib/types.js";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { defaultColors } from "../components/ui/chart/index.js";
import ResultHistogram from "@/components/charts/ResultHistogram.vue";

const router = useRouter();
const simulation_results = ref<any>(null);
const loading = ref<boolean>(true);
const colors = ref<string[]>([""]);

const { competitors, event, name, simCount, monthCutoff } =
  router.currentRoute.value.query;

if (!competitors || !event || !name || !simCount || !monthCutoff) {
  throw new Error("One or more required parameters are null or undefined.");
}

const competitorsList = competitors?.toString().split(",");

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
        const response = await fetchWCAInfo(
          `https://raw.githubusercontent.com/robiningelbrecht/wca-rest-api/master/api/competitions/${year}.json`,
        );
        return response.items.map((comp: { id: any; date: { from: any } }) => {
          return { id: comp.id, date: comp.date.from };
        });
      }),
    )
  ).flat();

  loading.value = false;
  return filterCompetitions(competitions, results, event, startDate);
};

const runSimulation = (
  results: { id: string; name: string; results: number[] }[],
  simCount: number,
  event: WCAevent,
) => {
  init().then(() => {
    const result_times = results.map((result) => result.results);
    const raw_results = simulate(
      { results: result_times },
      simCount,
      eventInfo[event].format,
    );
    simulation_results.value = raw_results.map((item: any, idx: number) => {
      return {
        name: results[idx].name,
        id: results[idx].id,
        wins: parseFloat(((item.wins / simCount) * 100).toFixed(2)),
        podiums: parseFloat(((item.podiums / simCount) * 100).toFixed(2)),
        mu: parseFloat((item.mu / 100).toFixed(4)),
        sigma: parseFloat((item.sigma / 100).toFixed(4)),
        tau: parseFloat((item.tau / 100).toFixed(4)),
      };
    });

    colors.value = defaultColors(raw_results.length);
  });
};

onMounted(async () => {
  let startDate = new Date();
  startDate.setMonth(new Date().getMonth() - parseInt(monthCutoff.toString()));
  const data = await fetchData(competitorsList, event as WCAevent, startDate);
  runSimulation(data, parseInt(simCount.toString()), event);
});
</script>

<template>
  <div class="flex flex-col items-center justify-center my-10">
    <div v-if="simulation_results" class="border rounded-md min-w-[70vw]">
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
          class="p-1 hover:bg-secondary rounded-md"
        >
          <Collapsible>
            <CollapsibleTrigger as-child>
              <div class="flex justify-between p-2 cursor-pointer">
                <div class="flex-1 text-left">
                  {{ result.name }}
                </div>
                <div class="flex-1 text-center">{{ result.wins }}%</div>
                <div class="flex-1 text-center">{{ result.podiums }}%</div>
                <div class="flex-1 text-center">{{ result.podiums }}%</div>
              </div>
            </CollapsibleTrigger>
            <CollapsibleContent class="space-y-2">
              <ResultHistogram
                :mu="result.mu"
                :sigma="result.sigma"
                :tau="result.tau"
                :color="colors[idx]"
              />
            </CollapsibleContent>
          </Collapsible>
        </li>
      </ol>
    </div>
    <div v-else-if="loading">Fetching data...</div>
    <div v-else>Calculating...</div>
  </div>
</template>
