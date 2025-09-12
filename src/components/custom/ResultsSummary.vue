<script setup lang="ts">
import PieChart from "@/components/charts/PieChart.vue";
import { SimulationResultProps, eventNames } from "@/lib/types";
import { formatPercentage, toClockFormat } from "@/lib/utils";
import { computed } from "vue";

const { data, colors, numSimulations, event } =
  defineProps<SimulationResultProps>();

const topCompetitor = computed(() =>
  data.reduce(
    (max, competitor) =>
      competitor.win_count > max.win_count ? competitor : max,
    data[0],
  ),
);

const avgRank = computed(() =>
  (topCompetitor.value.total_rank / numSimulations).toFixed(2),
);

const winChance = computed(() =>
  formatPercentage(topCompetitor.value.win_count / numSimulations, true),
);

const podiumChance = computed(() =>
  formatPercentage(topCompetitor.value.pod_count / numSimulations, true),
);

const expectedAvg = computed(() =>
  toClockFormat(topCompetitor.value.mean_no_dnf),
);
</script>

<template>
  <div class="mb-2 flex h-full flex-col gap-2 md:flex-row">
    <div class="flex-grow">
      <div class="h-full rounded-md border p-4">
        <p class="mb-2 text-lg font-bold">{{ eventNames[event] }} Statistics</p>
        <div class="space-y-2">
          <p class="text-sm">
            <span class="font-semibold">{{ topCompetitor.name }}</span> has the
            highest odds of winning with:
          </p>
          <ul class="ml-4 list-inside list-disc text-sm">
            <li>{{ winChance }} chance of winning</li>
            <li>{{ podiumChance }} chance of podium finish</li>
            <li>Average rank of {{ avgRank }}</li>
            <li>Expected average of {{ expectedAvg }}</li>
          </ul>
        </div>
      </div>
    </div>
    <div class="rounded-md border p-2">
      <PieChart :data :num-simulations :colors />
    </div>
  </div>
</template>
