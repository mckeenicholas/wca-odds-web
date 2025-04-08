<script setup lang="ts">
import { ref, watch, onMounted, onBeforeUnmount } from "vue";
import { RouterLink } from "vue-router";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { fetchWCAInfo } from "@/lib/utils";
import { useQuery } from "@tanstack/vue-query";
import debounce from "debounce";

interface Competition {
  start_date: string;
  id: string;
  name: string;
}

const input = ref<string>("");

const syncInputWithURL = () => {
  const searchParam =
    new URLSearchParams(window.location.search).get("q") || "";
  input.value = searchParam;
};

const updateURL = (value: string) => {
  const url = new URL(window.location.href);
  if (value) {
    url.searchParams.set("q", value);
  } else {
    url.searchParams.delete("q");
  }
  window.history.replaceState({}, "", url);
};

const { isFetching, isError, data, error, refetch } = useQuery({
  queryKey: ["competitionSearch", input.value],
  queryFn: () =>
    fetchWCAInfo<Competition[]>(
      `https://api.worldcubeassociation.org/competitions?q=${input.value}`,
    ),
  enabled: false,
});

const debouncedRefetch = debounce(() => refetch(), 250);
const debouncedUpdateURL = debounce((value: string) => updateURL(value), 250);

watch(input, (value) => {
  debouncedRefetch();
  debouncedUpdateURL(value);
});

const handleSearch = () => {
  refetch();
};

onMounted(() => {
  syncInputWithURL();
  window.addEventListener("popstate", syncInputWithURL);
});

onBeforeUnmount(() => {
  window.removeEventListener("popstate", syncInputWithURL);
});
</script>

<template>
  <div class="flex flex-col items-center justify-center">
    <div>
      <h1 class="text-center text-3xl font-bold m-6">
        WCA Competition Simulator
      </h1>
      <h1 class="text-center text-xl m-4">Find a competition</h1>
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
          <div v-for="index in 12" :key="index">
            <Skeleton class="h-6 my-2" />
            <Skeleton class="h-5 w-24 my-2" />
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
            <RouterLink :to="`/competition/${result.id}`">
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
            </RouterLink>
          </li>
        </ol>
      </div>
      <div v-else-if="input" class="text-center m-4">No competitions found</div>
    </div>
    <div class="mt-8 items-center justify-center">
      <RouterLink to="/custom">
        <Button> Or select competitors manually </Button>
      </RouterLink>
    </div>
  </div>
</template>
