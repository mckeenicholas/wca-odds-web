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
  <div class="flex flex-col md:flex-row gap-2 mb-2 h-full">
    <div class="flex-grow">
      <div class="border rounded-md p-4 h-full">
        <h3 class="font-bold text-lg mb-2">
          {{ eventNames[event] }} Statistics
        </h3>
        <div class="space-y-2">
          <p class="text-sm">
            <span class="font-semibold">{{ topCompetitor.name }}</span> has the
            highest odds of winning with:
          </p>
          <ul class="list-disc list-inside text-sm ml-4">
            <li>{{ winChance }} chance of winning</li>
            <li>{{ podiumChance }} chance of podium finish</li>
            <li>Average rank of {{ avgRank }}</li>
            <li>Expected average of {{ expectedAvg }}</li>
          </ul>
        </div>
      </div>
    </div>
    <div class="border rounded-md p-2">
      <PieChart :data :num-simulations :colors />
    </div>
  </div>
</template>
