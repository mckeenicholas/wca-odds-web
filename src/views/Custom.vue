<script setup lang="ts">
import { ref, watch } from "vue";
import { fetchWCAInfo } from "@/lib/utils";
import { useQuery } from "@tanstack/vue-query";
import { Input } from "@/components/ui/input";
import { Skeleton } from "@/components/ui/skeleton";
import { Search, X } from "lucide-vue-next";
import ControlPanel from "@/components/custom/ControlPanel.vue";
import { supportedWCAEvents } from "@/lib/types";

interface Person {
  name: string;
  wca_id: string;
}

interface SearchResult {
  result: Person[];
}

const input = ref<string>("");
const competitors = ref<Person[]>([]);

const selectedEventId = ref<string>("333");
const simCount = ref<number>(10000);
const monthCount = ref<number>(12);

const fetchPersonsResult = async (): Promise<Person[]> => {
  if (input.value === "") {
    return [];
  }

  const response = await fetchWCAInfo<SearchResult>(
    `https://api.worldcubeassociation.org/search/users?q=${input.value}`,
  );
  return response.result.filter(({ wca_id }) => wca_id != null);
};

const { isFetching, isError, data, error, refetch } = useQuery({
  queryKey: ["userSearch", input.value],
  queryFn: fetchPersonsResult,
  enabled: false,
});

const debounce = (fn: () => void, delay: number) => {
  let timeoutId: number;
  return () => {
    clearTimeout(timeoutId);
    timeoutId = window.setTimeout(fn, delay);
  };
};

const debouncedRefetch = debounce(() => refetch(), 250);

watch(input, () => {
  debouncedRefetch();
});

const handleSearch = () => {
  refetch();
};

const addCompetitor = (competitor: Person) => {
  competitors.value.push(competitor);
  input.value = "";
  handleSearch();
};

const removeCompetitor = (competitorId: string) => {
  competitors.value = competitors.value.filter(
    (person) => person.wca_id != competitorId,
  );
};

const runSimulation = () => {
  if (data.value) {
    const queryParams = new URLSearchParams({
      name: "Custom Simulation",
      eventId: selectedEventId.value,
      simCount: simCount.value.toString(),
      monthCutoff: monthCount.value.toString(),
      competitors: competitors.value.map((c: Person) => c.wca_id).join(","),
    });
    const url = `/simulation?${queryParams.toString()}`;
    window.location.href = url;
  }
};
</script>
<template>
  <div class="flex flex-col items-center justify-center">
    <div>
      <h1 class="text-center text-xl m-2">Add a competitor</h1>
      <div class="flex flex-row min-w-[70vw] relative">
        <Input
          v-model="input"
          @keyup.enter="handleSearch"
          placeholder="Competitor Name..."
          :class="`-me-2 ${input ? 'rounded-b-none' : ''}`"
        />
        <span
          class="absolute end-0 inset-y-0 flex items-center justify-center px-2"
        >
          <Search class="size-6 text-muted-foreground" />
        </span>

        <!-- Popover Content -->
        <div
          v-if="input"
          class="absolute top-full left-0 z-50 w-full bg-white shadow-md border border-t-0 rounded-b-md mt-0 overflow-y-scroll max-h-[40vh]"
        >
          <div v-if="isFetching" class="px-3">
            <Skeleton v-for="index in 3" class="h-6 my-4" :key="index" />
          </div>
          <div v-else-if="isError" class="text-center m-4">
            Error fetching Data: {{ error }}
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
                :onclick="() => addCompetitor(person)"
              >
                <div class="flex justify-between items-center">
                  <p>
                    {{ person.name }}
                  </p>
                  <p class="text-muted-foreground">
                    {{ person.wca_id }}
                  </p>
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
          v-on:run-simulation="runSimulation"
        />
      </div>

      <div
        v-if="competitors.length"
        class="mt-4 border rounded-md max-h-[75vh]"
      >
        <ol>
          <li
            v-for="competitor of competitors"
            class="flex justify-between items-center"
          >
            <a
              :key="competitor.wca_id"
              :href="`https://worldcubeassociation.org/persons/${competitor.wca_id}`"
              class="hover:underline p-2"
            >
              {{ competitor.name }}
            </a>
            <a
              class="rounded-md hover:bg-secondary p-1 me-1 hover:cursor-pointer"
              :onclick="() => removeCompetitor(competitor.wca_id)"
            >
              <X :size="24" />
            </a>
          </li>
        </ol>
      </div>
    </div>
  </div>
</template>
