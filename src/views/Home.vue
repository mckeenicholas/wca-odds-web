<script setup lang="ts">
import { ref, watch } from "vue";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { ScrollArea } from "@/components/ui/scroll-area";
import { fetchWCAInfo } from "@/lib/utils";

interface Competition {
  start_date: string;
  id: string;
  name: string;
}

const input = ref<string>("");
const results = ref<Competition[]>([]);
const loading = ref<boolean>(false);

function debounce<T extends (...args: any[]) => void>(
  fn: T,
  delay: number,
): (...args: Parameters<T>) => void {
  let timeoutId: NodeJS.Timeout;
  return (...args: Parameters<T>) => {
    if (timeoutId) clearTimeout(timeoutId);
    timeoutId = setTimeout(() => {
      fn(...args);
    }, delay);
  };
}

const fetchCompetitions = async (query: string) => {
  loading.value = true;
  if (query) {
    const response = await fetchWCAInfo<Competition[]>(
      `https://api.worldcubeassociation.org/competitions?q=${query}`,
    );
    results.value =
      response !== null
        ? response
            .filter(
              (competition: { start_date: string }) =>
                new Date(competition.start_date) > new Date(),
            )
            .sort(
              (a: { start_date: string }, b: { start_date: string }) =>
                new Date(a.start_date) - new Date(b.start_date),
            )
        : [];
  } else {
    results.value = [];
  }
  loading.value = false;
};

const debouncedFetchCompetitions = debounce(fetchCompetitions, 300);

const handleSearch = () => {
  fetchCompetitions(input.value);
};

watch(input, (newVal) => {
  debouncedFetchCompetitions(newVal);
});
</script>

<template>
  <div class="flex flex-col items-center justify-center">
    <div>
      <h1 class="text-center">Search for a competition</h1>
      <div class="flex flex-row space-x-4 min-w-[70vw]">
        <Input
          v-model="input"
          @keyup.enter="handleSearch"
          placeholder="Competition Name..."
          class="-me-2"
        />
        <Button @click="handleSearch">Search</Button>
      </div>
      <div v-if="loading" class="mt-2">
        <div class="border rounded-md px-3 pt-1">
          <div v-for="index in 16">
            <Skeleton class="h-6 my-2" :key="index" />
            <Skeleton class="h-5 w-24 my-2" :key="index" />
          </div>
        </div>
      </div>
      <div v-else>
        <div v-if="results.length" class="mt-4">
          <ScrollArea class="h-[70vh] rounded-md border">
            <ol class="mx-1 py-1">
              <li
                v-for="result in results"
                :key="result.id"
                class="p-2 hover:bg-secondary rounded-md"
              >
                <a :href="`/competition/${result.id}`">
                  <p>{{ result.name }}</p>
                  <p class="text-sm text-secondary-foreground">
                    {{
                      new Date(result.start_date).toLocaleDateString("en-US", {
                        month: "short",
                        day: "numeric",
                        year: "numeric",
                      })
                    }}
                  </p>
                </a>
              </li>
            </ol>
          </ScrollArea>
        </div>
        <div v-else-if="input">No results found.</div>
      </div>
    </div>
    <div class="mt-4 items-center justify-center">
      <a href="/custom">
        <Button> Add competitors manually </Button>
      </a>
    </div>
  </div>
</template>
