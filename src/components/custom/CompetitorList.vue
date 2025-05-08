<script setup lang="ts">
import { SimulationResult, SupportedWCAEvent } from "@/lib/types";
import CompetitorDropdown from "./CompetitorDropdown.vue";
import Chevron from "./RotatableChevron.vue";
import { ref, computed } from "vue";

type sortCol = "name" | "win" | "pod" | "rank";

interface groupedResults {
  idx: number;
  results: SimulationResult;
  color: string;
  id: string;
}

const { simulationResults, colors, competitorsList, numSimulations, event } =
  defineProps<{
    simulationResults: SimulationResult[];
    colors: string[];
    competitorsList: string[];
    numSimulations: number;
    event: SupportedWCAEvent;
  }>();

const sortBy = ref<sortCol>("win");
const sortAsc = ref<boolean>(false);

const setSortBy = (col: sortCol) => {
  if (sortBy.value === col) {
    sortAsc.value = !sortAsc.value;
  } else {
    sortBy.value = col;
    sortAsc.value = false;
  }
};

const groupedProps = computed(() => {
  const sortFn = (a: groupedResults, b: groupedResults) => {
    const comparison =
      sortBy.value === "name"
        ? -a.results.name.localeCompare(b.results.name) // This is inverted to be consistent as we sort by descending for stats
        : sortBy.value === "win"
          ? a.results.win_count - b.results.win_count
          : sortBy.value === "pod"
            ? a.results.pod_count - b.results.pod_count
            : -(a.results.total_rank - b.results.total_rank); // This is also inverted for the same reason as above

    return sortAsc.value ? comparison : -comparison;
  };

  return simulationResults
    .map((results, idx) => ({
      idx,
      results,
      color: colors[idx],
      id: competitorsList[idx],
    }))
    .sort(sortFn);
});

const model = defineModel<number[][]>({ required: true });
</script>

<template>
  <div class="border rounded-md mt-2">
    <div class="flex justify-between p-1 me-8">
      <button
        @click="setSortBy('name')"
        class="flex-1 hover:bg-secondary rounded-md py-1 px-2 ps-3"
      >
        <div class="flex items-center justify-start">
          <span>Name</span>
          <Chevron
            v-show="sortBy === 'name'"
            class="ms-2"
            :up="!sortAsc"
            :animate="false"
          />
        </div>
      </button>
      <button
        @click="setSortBy('win')"
        class="flex-1 hover:bg-secondary rounded-md py-1 px-2"
      >
        <div class="flex items-center justify-center">
          <span>Chance of winning</span>
          <Chevron
            v-show="sortBy === 'win'"
            class="ms-2"
            :up="!sortAsc"
            :animate="false"
          />
        </div>
      </button>
      <button
        @click="setSortBy('pod')"
        class="flex-1 hover:bg-secondary rounded-md py-1 px-2"
      >
        <div class="flex items-center justify-center">
          <span>Chance of podiuming</span>
          <Chevron
            v-show="sortBy === 'pod'"
            class="ms-2"
            :up="!sortAsc"
            :animate="false"
          />
        </div>
      </button>
      <button
        @click="setSortBy('rank')"
        class="flex-1 hover:bg-secondary rounded-md py-1 px-2"
      >
        <div class="flex items-center justify-center">
          <span>Expected rank</span>
          <Chevron
            v-show="sortBy === 'rank'"
            class="ms-2"
            :up="!sortAsc"
            :animate="false"
          />
        </div>
      </button>
    </div>
    <hr class="mx-2" />
    <ol>
      <li
        v-for="(person, idx) in groupedProps"
        :key="idx"
        class="p-1 rounded-md"
      >
        <CompetitorDropdown
          :result="person.results"
          :event
          :num-simulations
          :color="person.color"
          :wca-id="person.color"
          v-model="model[person.idx]"
        />
      </li>
    </ol>
  </div>
</template>
