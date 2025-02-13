<script setup lang="ts">
import { onMounted } from "vue";
import init, { run_odds_simulation } from "../../wasm/odds_web.js";
import { fetchData } from "@/lib/utils.js";
// import init from "../../pkg/odds_web_bg.wasm?init"

interface Comp {
  id: string;
  date: number;
}

const processComps = async () => {
  const years = [2023, 2024, 2025];

  let comps: Record<string, number> = {};

  const parse = await Promise.all(
    years.map(async (year) => {
      const res = await fetch(
        `https://raw.githubusercontent.com/robiningelbrecht/wca-rest-api/master/api/competitions/${year}.json`,
      );
      const json = await res.json();

      const competitions = json.items.map(
        (item: { id: any; date: { from: string | number | Date } }) => {
          const id = item.id;
          const date = new Date(item.date.from).getTime();

          comps[id] = date;
        },
      );
    }),
  );

  return comps;
};

const getPersonData = async (persons: string[], event: string) => {
  const parse = await Promise.all(
    persons.map(async (person) => {
      const res = await fetch(
        `https://raw.githubusercontent.com/robiningelbrecht/wca-rest-api/master/api/persons/${person}.json`,
      );
      const json = await res.json();

      const results = Object.entries(json.results).map(([id, events]) => {
        if (events[event]) {
          let results = events[event].flatMap((result) => result.solves);
          return [{ id, solves: results }];
        } else {
          return [];
        }
      });
      return results.flat();
    }),
  );

  return parse;
};

const joinData = async (
  persons: string[],
  event: string,
  monthCutoff: number,
) => {
  const startDate = new Date();
  startDate.setMonth(startDate.getMonth() - monthCutoff);

  const comps = await processComps();
  const solves = await getPersonData(persons, event);

  console.time("join time");
  let joined = solves.map((competitions) => {
    const dated = competitions.flatMap((competition) => {
      const id = competition.id;
      if (comps[id]) {
        const compData = comps[id];
        return [{ solves: competition.solves, date: compData }];
      }
      return [];
    });
    return dated;
  });

  console.timeEnd("join time");
  return joined;
};

const testFn = async () => {
  const persons = [
    "2016LINB01",
    "2017YEHM01",
    "2017WURY01",
    "2017KWAN05",
    "2017WANS05",
    "2017SARE03",
    "2019LUYI03",
    "2015RAME02",
    "2022BHAV01",
    "2019HUAN05",
    "2016PATT02",
    "2015ZHEN17",
    "2023LOPE21",
    "2023ZHAO16",
    "2015WONG02",
    "2022WANG23",
  ];
  // const persons = ["2017YEHM01"]
  const monthCutoff = 12;

  const startDate = new Date();
  startDate.setMonth(startDate.getMonth() - monthCutoff);

  await init();
  console.time("asdf");

  const results = await run_odds_simulation(persons, "333", monthCutoff, 10000);

  console.timeEnd("asdf");

  console.log(results);
};

onMounted(() => {
  testFn();
});
</script>

<template>nothing to say here</template>
