<script setup lang="ts">
import { AreaChart } from "@/components/ui/chart-area";
import {
  SimulationResult,
  SupportedWCAEvent,
  SimulationResultProps,
} from "@/lib/types";
import { totalSolves, renderTime, createFMCTooltip } from "@/lib/utils";
import { ref, computed } from "vue";
import MultiLabelSwitch from "./MultiLabelSwitch.vue";
import Checkbox from "../ui/checkbox/Checkbox.vue";
import { Select, SelectTrigger, SelectContent } from "@/components/ui/select";
import { Label } from "../ui/label";
import ColoredCircle from "../custom/ColoredCircle.vue";
import { useMemoize } from "@vueuse/core";

const { data, event, colors } =
  defineProps<Omit<SimulationResultProps, "numSimulations">>();

const histogramTooltip = createFMCTooltip(event);

const enabled = ref<boolean[]>(Array(data.length).fill(true));
const isAverage = ref<boolean>(false);
const isCDF = ref<boolean>(false);

const getPrevFMCIndex = (idx: number, isAverage: boolean) => {
  if (!isAverage) return idx - 10;

  const mod = idx % 10;
  if (mod == 0) return idx - 4;
  if (mod == 3 || mod == 6) return idx - 3;

  return 0;
};

const getNextIndex = (
  currentIndex: number,
  isFMC: boolean,
  isAverage: boolean,
): number => {
  if (!isFMC) return currentIndex + 1;
  if (!isAverage) return currentIndex + 10;

  const mod = currentIndex % 10;
  if (mod === 0) return currentIndex + 3;
  if (mod === 3) return currentIndex + 3;
  if (mod === 6) return currentIndex + 4;
  return currentIndex + 1;
};

const reduceDataPoints = (
  values: Array<Record<string, number>>,
): Array<Record<string, number>> => {
  const reduceBy = Math.ceil(Math.log2(values.length)) - 8;

  if (reduceBy <= 0) {
    return values;
  }

  const mergeFactor = Math.pow(2, reduceBy);

  const chunkIndices = Array.from(
    { length: Math.ceil(values.length / mergeFactor) },
    (_, i) => i * mergeFactor,
  );

  return chunkIndices.map((startIndex) => {
    const chunk = values.slice(startIndex, startIndex + mergeFactor);

    const base = { time: chunk[0].time };

    const allKeys = [
      ...new Set(
        chunk.flatMap((item) =>
          Object.keys(item).filter((key) => key !== "time"),
        ),
      ),
    ];

    const averages = allKeys.reduce(
      (acc, key) => ({
        ...acc,
        [key]:
          chunk.reduce((sum, item) => sum + (item[key] || 0), 0) / mergeFactor,
      }),
      {},
    );

    return { ...base, ...averages };
  });
};

const findTimeRange = (
  data: SimulationResult[],
  isAverage: boolean,
): [number, number] => {
  return data.reduce(
    ([minAccPerson, maxAccPerson], person) => {
      const results = isAverage
        ? person.results.hist_values_average
        : person.results.hist_values_single;

      const [minPerson, maxPerson] = [...results].reduce(
        ([minAcc, maxAcc], [time]) => [
          Math.min(time, minAcc),
          Math.max(time, maxAcc),
        ],
        [Number.MAX_SAFE_INTEGER, 0],
      );

      return [
        Math.min(minPerson, minAccPerson),
        Math.max(maxPerson, maxAccPerson),
      ];
    },
    [Number.MAX_SAFE_INTEGER, 0],
  );
};

const generateHistogramData = useMemoize(
  (
    data: SimulationResult[],
    minTime: number,
    maxTime: number,
    isAverage: boolean,
    isCDF: boolean,
    event: SupportedWCAEvent,
    enabledPersons: boolean[],
  ): Map<number, Map<string, number>> => {
    const resultTimes = new Map<number, Map<string, number>>();

    data.forEach((person, idx) => {
      if (!enabledPersons[idx]) {
        return;
      }

      const results = isAverage
        ? person.results.hist_values_average
        : person.results.hist_values_single;

      const solveCount = totalSolves(results);
      let i = minTime;

      while (i <= maxTime) {
        if (!resultTimes.has(i)) {
          resultTimes.set(i, new Map<string, number>());
        }

        const timesMap = resultTimes.get(i)!;
        const numOccurrences = results.get(i) ?? 0;

        let value: number;
        if (isCDF) {
          const prevIndex =
            event === "333fm" ? getPrevFMCIndex(i, isAverage) : i - 1;
          const prevValue = resultTimes.get(prevIndex)?.get(person.name) ?? 0;
          const currentValue = parseFloat(
            (numOccurrences / (solveCount / 100)).toFixed(4),
          );

          value = i === minTime ? currentValue : prevValue + currentValue;
        } else {
          value = parseFloat((numOccurrences / (solveCount / 100)).toFixed(4));
        }

        timesMap.set(person.name, value);
        i = getNextIndex(i, event === "333fm", isAverage);
      }
    });

    return resultTimes;
  },
);

const formatChartData = (
  resultTimes: Map<number, Map<string, number>>,
): Array<Record<string, number>> => {
  return [...resultTimes.entries()]
    .map(([time, nameMap]) => ({
      time,
      ...Object.fromEntries(nameMap.entries()),
    }))
    .sort((a, b) => a.time - b.time);
};

const chartData = computed(() => {
  const [min, max] = findTimeRange(data, isAverage.value);
  const histogramData = generateHistogramData(
    data,
    min,
    max,
    isAverage.value,
    isCDF.value,
    event,
    enabled.value,
  );
  const formattedData = formatChartData(histogramData);
  return reduceDataPoints(formattedData);
});

const xFormatter = (value: number | Date) =>
  renderTime(chartData.value[value as number].time * 10, event === "333fm");

const names = data.map((person) => person.name) as unknown as "time"[];
</script>

<template>
  <div class="mb-4 mt-2 ms-4 -me-6">
    <AreaChart
      class="-ms-6"
      :data="chartData"
      index="time"
      :categories="names"
      :colors
      :showLegend="false"
      :customTooltip="histogramTooltip"
      :showXAxis="true"
      :yFormatter="(value) => `${value}%`"
      :xFormatter
    />
    <div class="lg:flex lg:pe-10">
      <MultiLabelSwitch left="Single" right="Average" v-model="isAverage" />
      <MultiLabelSwitch left="Probability" right="Cumulative" v-model="isCDF" />
      <div class="ms-4 flex flex-grow justify-end">
        <Select>
          <SelectTrigger class="min-w-36 mt-2"> Competitors </SelectTrigger>
          <SelectContent>
            <ul class="min-w-7xl">
              <li
                v-for="(result, idx) in data"
                :key="idx"
                class="mx-2 flex items-center flex-grow"
              >
                <Checkbox
                  :id="`checkbox-${idx}`"
                  v-model:checked="enabled[idx]"
                />
                <Label
                  :for="`checkbox-${idx}`"
                  class="flex items-center text-md font-normal"
                >
                  <ColoredCircle class="mx-2" :color="colors[idx]" />
                  {{ result.name }}
                </Label>
              </li>
            </ul>
          </SelectContent>
        </Select>
      </div>
    </div>
  </div>
</template>
