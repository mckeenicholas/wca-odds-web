<script setup lang="ts">
import { ref, watch } from "vue";
import { Input } from "../components/ui/input";
import { Button } from "../components/ui/button";
import { Skeleton } from "../components/ui/skeleton";
import { fetchWCAInfo } from "../lib/utils";
import { useQuery } from "@tanstack/vue-query";

interface Competition {
  start_date: string;
  id: string;
  name: string;
}

const input = ref<string>("");

const { isFetching, isError, data, error, refetch } = useQuery({
  queryKey: ["competitionSearch", input.value],
  queryFn: () =>
    fetchWCAInfo<Competition[]>(
      `https://api.worldcubeassociation.org/competitions?q=${input.value}`,
    ),
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
</script>

<template>
  {{ isFetching }}
  {{ input }}
  <div class="flex flex-col items-center justify-center">
    <div>
      <h1 class="text-center text-xl m-2">Find a competition</h1>
      <div class="flex flex-row space-x-4 min-w-[70vw]">
        <Input
          v-model="input"
          @keyup.enter="handleSearch"
          placeholder="Competition Name..."
          class="-me-2"
        />
        <Button @click="handleSearch">Search</Button>
      </div>
      <div v-if="isFetching && input" class="mt-2">
        <div class="border rounded-md px-3 pt-1 max-h-[75vh] overflow-y-scroll">
          <div v-for="index in 12">
            <Skeleton class="h-6 my-2" :key="index" />
            <Skeleton class="h-5 w-24 my-2" :key="index" />
          </div>
        </div>
      </div>
      <div v-else-if="isError">Error fetching Data: {{ error }}</div>
      <div
        v-else-if="data?.length"
        class="mt-4 border rounded-md max-h-[75vh] overflow-y-scroll"
      >
        <ol class="mx-1 py-1">
          <li
            v-for="result in data"
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
      </div>
      <div v-else class="text-center m-4">No competitions found</div>
    </div>
    <div class="mt-4 items-center justify-center">
      <a href="/custom">
        <Button> Or select competitors manually </Button>
      </a>
    </div>
  </div>
</template>
