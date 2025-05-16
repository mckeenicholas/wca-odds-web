<script setup lang="ts">
import { ref, watch } from "vue";
import { fetchWCAInfo } from "@/lib/utils";
import { useQuery } from "@tanstack/vue-query";
import { Input } from "@/components/ui/input";
import { Skeleton } from "@/components/ui/skeleton";
import { Search, X } from "lucide-vue-next";
import ControlPanel from "@/components/custom/ControlPanel.vue";
import { SimulationRouteQuery, supportedWCAEvents } from "@/lib/types";
import FlagIcon from "@/components/custom/FlagIcon.vue";
import { useRouter } from "vue-router";
import { useDebounceFn } from "@vueuse/core";

interface Person {
  name: string;
  wca_id: string;
  country: { iso2: string };
}

interface SearchResult {
  result: Person[];
}

const router = useRouter();
const input = ref<string>("");
const competitors = ref<Person[]>(
  JSON.parse(localStorage.getItem("competitors") || "[]"),
);

const selectedEventId = ref<string>("333");
const simCount = ref<number>(10000);
const monthCount = ref<number>(12);
const includeDnf = ref<boolean>(true);
const decayHalfLife = ref<number>(180);

const searchPersons = async (): Promise<Person[]> => {
  if (!input.value.trim()) return [];

  const response = await fetchWCAInfo<SearchResult>(
    `https://api.worldcubeassociation.org/search/users?q=${input.value}`,
  );

  return response.result.filter((person) => person.wca_id);
};

const { isFetching, isError, data, error, refetch } = useQuery({
  queryKey: ["userSearch", input.value],
  queryFn: searchPersons,
  enabled: false,
});

watch(
  competitors,
  (newCompetitors) => {
    localStorage.setItem("competitors", JSON.stringify(newCompetitors));
  },
  { deep: true },
);

const debouncedInput = useDebounceFn(refetch, 250);

const addCompetitor = (competitor: Person) => {
  if (!competitors.value.some((c) => c.wca_id === competitor.wca_id)) {
    competitors.value.push(competitor);
    input.value = "";
  }
};

const removeCompetitor = (competitorId: string) => {
  competitors.value = competitors.value.filter(
    (p) => p.wca_id !== competitorId,
  );
};

const runSimulation = () => {
  const query: SimulationRouteQuery = {
    name: "Custom Simulation",
    eventId: selectedEventId.value,
    simCount: simCount.value.toString(),
    monthCutoff: monthCount.value.toString(),
    includeDnf: includeDnf.value.toString(),
    decayRate: decayHalfLife.value.toString(),
    competitors: competitors.value.map((c: Person) => c.wca_id).join(","),
  };

  router.push({
    path: "/custom/results",
    query: query as Record<string, string>,
  });
};
</script>

<template>
  <div class="flex flex-col items-center justify-center">
    <div>
      <h1 class="text-center text-xl m-4">Add a competitor</h1>
      <div class="flex flex-row min-w-[70vw] relative">
        <Input
          v-model="input"
          @keyup.enter="refetch()"
          @input="debouncedInput"
          placeholder="Competitor Name..."
          :class="{ 'rounded-b-none': input }"
          class="-me-2"
          aria-label="Search for competitors"
        />
        <span
          class="absolute end-0 inset-y-0 flex items-center justify-center px-2"
        >
          <Search class="size-6 text-muted-foreground" />
        </span>

        <!-- Popover Content -->
        <div
          v-if="input"
          class="absolute top-full left-0 z-50 w-full bg-[hsl(var(--popover))] shadow-md border border-t-0 rounded-b-md mt-0 overflow-y-scroll max-h-[40vh]"
        >
          <div v-if="isFetching" class="px-3">
            <Skeleton v-for="index in 3" class="h-6 my-4" :key="index" />
          </div>
          <div v-else-if="isError" class="text-center m-4">
            Error fetching data:
            {{ error?.message || "Unknown error occurred" }}
          </div>
          <div v-else-if="!data?.length" class="text-center m-4">
            No Results found
          </div>
          <div v-else>
            <ol class="mx-1 py-1">
              <li
                v-for="person in data"
                :key="person.wca_id"
                class="p-2 hover:bg-secondary hover:cursor-pointer rounded-md"
                @click="() => addCompetitor(person)"
              >
                <div class="flex justify-between items-center">
                  <div class="flex items-center gap-2">
                    <FlagIcon :code="person.country.iso2" />
                    <span>{{ person.name }}</span>
                  </div>
                  <span class="text-muted-foreground">{{ person.wca_id }}</span>
                </div>
              </li>
            </ol>
          </div>
        </div>
      </div>

      <div class="mt-4">
        <ControlPanel
          v-bind:event-ids="[...supportedWCAEvents]"
          v-model:month-count="monthCount"
          v-model:selected-event-id="selectedEventId"
          v-model:sim-count="simCount"
          v-model="includeDnf"
          v-bind:decay-rate="decayHalfLife"
          :disableRun="competitors.length < 2"
          @run-simulation="runSimulation"
        />
      </div>

      <div
        v-if="competitors.length"
        class="mt-4 border rounded-md max-h-[75vh]"
      >
        <ol>
          <li
            v-for="competitor of competitors"
            :key="competitor.wca_id"
            class="flex justify-between items-center"
          >
            <a
              :href="`https://worldcubeassociation.org/persons/${competitor.wca_id}`"
              class="hover:underline p-2"
            >
              {{ competitor.name }}
            </a>
            <a
              class="rounded-md hover:bg-secondary p-1 me-1 hover:cursor-pointer"
              @click="() => removeCompetitor(competitor.wca_id)"
            >
              <X :size="24" />
            </a>
          </li>
        </ol>
      </div>
    </div>
  </div>
</template>
