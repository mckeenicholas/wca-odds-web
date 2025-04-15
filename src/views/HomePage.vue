<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { RouterLink, useRoute, useRouter } from "vue-router";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { fetchWCAInfo } from "@/lib/utils";
import { useQuery } from "@tanstack/vue-query";
import { useDebounceFn } from "@vueuse/core";

interface Competition {
  start_date: string;
  id: string;
  name: string;
}

const route = useRoute();
const router = useRouter();
const input = ref<string>((route.query.q as string) || "");

watch(
  input,
  useDebounceFn((newInput) => {
    router.push({ query: { q: newInput || undefined } });
    refetch();
  }, 250),
);

onMounted(() => {
  refetch();
});

const { isFetching, isError, data, error, refetch } = useQuery({
  queryKey: ["competitionSearch", input.value],
  queryFn: () => {
    if (!input.value) return [];
    return fetchWCAInfo<Competition[]>(
      `https://api.worldcubeassociation.org/competitions?q=${input.value}`,
    );
  },
  enabled: false,
});

const formatDate = (date: string) =>
  new Date(date).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
</script>

<template>
  <div class="flex flex-col items-center justify-center">
    <div>
      <h1 class="text-center text-3xl font-bold m-6">
        WCA Competition Predictor
      </h1>
      <h1 class="text-center text-xl m-4">Find a competition</h1>
      <div class="flex flex-row space-x-4 min-w-[70vw]">
        <Input
          v-model="input"
          @keyup.enter="refetch()"
          placeholder="Competition Name..."
          class="-me-2"
        />
        <Button @click="refetch()" :disabled="isFetching">Search</Button>
      </div>
      <div v-if="isFetching && input" class="mt-2">
        <div class="border rounded-md px-3 pt-1 max-h-[75vh] overflow-y-scroll">
          <div v-for="index in 12" :key="index">
            <Skeleton class="h-6 my-2" />
            <Skeleton class="h-5 w-24 my-2" />
          </div>
        </div>
      </div>
      <div v-else-if="isError">
        Error fetching data: {{ error?.message || "Unknown error occurred" }}
      </div>
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
                {{ formatDate(result.start_date) }}
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
