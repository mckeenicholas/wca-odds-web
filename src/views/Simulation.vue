<script setup lang="ts">
import { useRouter } from "vue-router";
import { onMounted } from "vue";
import init, { simulate } from "../../build/odds_web.js";
import { fetchWCAInfo } from "@/lib/utils.js";
import { eventInfo, WCAevent } from "@/lib/utils.js";

const router = useRouter();
const { competitors, event, name, simCount } = router.currentRoute.value.query;

if (!competitors || !event || !name || !simCount) {
  throw new Error("One or more required parameters are null or undefined.");
}

const competitorsList = competitors?.toString().split(",");

type Competition = {
  results: any;
  id: string;
  date: string;
};

type Competitor = {
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
    return results.length ? [{ id: person.id, results: results }] : [];
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

  return filterCompetitions(competitions, results, event, startDate);
};

const runSimulation = (
  results: { id: string; results: number[] }[],
  simCount: number,
  event: WCAevent,
) => {
  init().then(() => {
    const result_times = results.map((result) => result.results);
    const simulation_results = simulate(
      { results: result_times },
      simCount,
      eventInfo[event].format,
    );
    console.log(simulation_results);
  });
};

onMounted(async () => {
  const months = 12;
  let startDate = new Date();
  startDate.setMonth(new Date().getMonth() - months);
  const data = await fetchData(competitorsList, event as WCAevent, startDate);
  runSimulation(data, parseInt(simCount.toString()), event);
});
</script>

<template>test</template>
