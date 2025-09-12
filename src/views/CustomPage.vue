<script setup lang="ts">
import ControlPanel from "@/components/custom/ControlPanel.vue";
import FlagIcon from "@/components/custom/FlagIcon.vue";
import { Input } from "@/components/ui/input";
import { Skeleton } from "@/components/ui/skeleton";
import { supportedWCAEvents } from "@/lib/types";
import { buildSimulationQuery, fetchWCAInfo } from "@/lib/utils";
import { useQuery } from "@tanstack/vue-query";
import { useDebounceFn } from "@vueuse/core";
import { Search, X } from "lucide-vue-next";
import { ref, watch } from "vue";
import { useRouter } from "vue-router";

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
const includeDnf = ref<boolean>(true);
const decayHalfLife = ref<number>(180);
const startDate = ref<Date>(
  new Date(new Date().setFullYear(new Date().getFullYear() - 1)),
);
const endDate = ref<Date>(new Date());

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
  const query = buildSimulationQuery({
    name: "Custom Simulation",
    eventId: selectedEventId.value,
    simCount: simCount.value,
    startDate: startDate.value,
    endDate: endDate.value,
    includeDnf: includeDnf.value,
    decayRate: decayHalfLife.value,
    competitors: competitors.value.map((c: Person) => c.wca_id),
  });

  router.push({
    path: "/custom/results",
    query: query as Record<string, string>,
  });
};
</script>

<template>
  <div class="flex flex-col items-center justify-center">
    <div>
      <h1 class="m-4 text-center text-xl">Add a competitor</h1>
      <div class="relative flex min-w-[70vw] flex-row">
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
          class="absolute inset-y-0 end-0 flex items-center justify-center px-2"
        >
          <Search class="size-6 text-muted-foreground" />
        </span>

        <!-- Popover Content -->
        <div
          v-if="input"
          class="absolute left-0 top-full z-50 mt-0 max-h-[40vh] w-full overflow-y-scroll rounded-b-md border border-t-0 bg-[hsl(var(--popover))] shadow-md"
        >
          <div v-if="isFetching" class="px-3">
            <Skeleton v-for="index in 3" class="my-4 h-6" :key="index" />
          </div>
          <div v-else-if="isError" class="m-4 text-center">
            Error fetching data:
            {{ error?.message || "Unknown error occurred" }}
          </div>
          <div v-else-if="!data?.length" class="m-4 text-center">
            No Results found
          </div>
          <div v-else>
            <ol class="mx-1 py-1">
              <li
                v-for="person in data"
                :key="person.wca_id"
                class="rounded-md p-2 hover:cursor-pointer hover:bg-secondary"
                @click="() => addCompetitor(person)"
              >
                <div class="flex items-center justify-between">
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
          v-model:selected-event-id="selectedEventId"
          v-model:sim-count="simCount"
          v-model:include-dnf="includeDnf"
          v-model:decay-rate="decayHalfLife"
          v-model:start-date="startDate"
          v-model:end-date="endDate"
          :disableRun="competitors.length < 2"
          @run-simulation="runSimulation"
        />
      </div>

      <div
        v-if="competitors.length"
        class="mt-4 max-h-[75vh] rounded-md border"
      >
        <ol>
          <li
            v-for="competitor of competitors"
            :key="competitor.wca_id"
            class="flex items-center justify-between"
          >
            <a
              :href="`https://worldcubeassociation.org/persons/${competitor.wca_id}`"
              class="p-2 hover:underline"
            >
              {{ competitor.name }}
            </a>
            <button
              class="me-1 rounded-md p-1 hover:cursor-pointer hover:bg-secondary"
              aria-label="remove competitor"
              @click="() => removeCompetitor(competitor.wca_id)"
            >
              <X :size="24" />
            </button>
          </li>
        </ol>
      </div>
    </div>
  </div>
</template>
