<script setup lang="ts">
import { onMounted, ref, watch, nextTick, useTemplateRef } from "vue";
import { RouterLink, useRoute, useRouter } from "vue-router";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { fetchWCAInfo, formatDate } from "@/lib/utils";
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
const selectedResult = ref<number>(-1);
const listContainerRef = useTemplateRef<HTMLDivElement>("listContainer");

onMounted(() => {
  refetch();
  const inputField = document.getElementById("input-field");
  inputField?.focus();
});

watch(
  input,
  useDebounceFn((newInput) => {
    const trimmedInput = newInput?.trim();
    router.push({
      query: trimmedInput ? { q: trimmedInput } : {},
    });
    if (trimmedInput) {
      refetch();
    } else {
      selectedResult.value = -1;
    }
  }, 250),
);

const { isFetching, isError, data, error, refetch } = useQuery({
  queryKey: ["competitionSearch", input.value],
  queryFn: () => {
    if (!input.value.trim()) return Promise.resolve([]);
    return fetchWCAInfo<Competition[]>(
      `https://api.worldcubeassociation.org/competitions?q=${input.value.trim()}`,
    );
  },
  enabled: false,
});

watch(data, () => {
  selectedResult.value = -1;
});

const scrollToSelected = () => {
  nextTick(() => {
    if (listContainerRef.value && selectedResult.value !== -1) {
      const olElement = listContainerRef.value.querySelector("ol");
      if (
        olElement &&
        data.value &&
        selectedResult.value >= 0 &&
        selectedResult.value < data.value.length &&
        olElement.children[selectedResult.value]
      ) {
        const selectedItemEl = olElement.children[
          selectedResult.value
        ] as HTMLElement;
        selectedItemEl.scrollIntoView({ block: "nearest", behavior: "smooth" });
      }
    }
  });
};

const handleKeydown = (event: KeyboardEvent) => {
  if (!data.value || data.value.length === 0) {
    if (event.key === "ArrowUp" || event.key === "ArrowDown") {
      event.preventDefault();
    }
    return;
  }

  switch (event.key) {
    case "ArrowDown":
      event.preventDefault();
      selectedResult.value =
        selectedResult.value === -1
          ? 0
          : (selectedResult.value + 1) % data.value.length;
      scrollToSelected();
      break;
    case "ArrowUp":
      event.preventDefault();
      selectedResult.value =
        selectedResult.value === -1
          ? data.value.length - 1
          : selectedResult.value === 0
            ? data.value.length - 1
            : selectedResult.value - 1;
      scrollToSelected();
      break;
    case "Enter":
      if (
        selectedResult.value !== -1 &&
        selectedResult.value < data.value.length
      ) {
        event.preventDefault();
        const competition = data.value[selectedResult.value];
        router.push(`/competition/${competition.id}`);
      }
      break;
    case "Escape":
      event.preventDefault();
      selectedResult.value = -1;
      break;
  }
};
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
          id="input-field"
          v-model="input"
          @keyup.enter="refetch()"
          @keydown="handleKeydown"
          placeholder="Competition Name..."
          class="-me-2"
          aria-label="Competition search"
          aria-describedby="search-instructions"
        />
        <Button
          @click="refetch()"
          class="h-[40px]"
          :disabled="isFetching || !input.trim()"
          aria-label="Search for competitions"
        >
          {{ isFetching ? "Searching..." : "Search" }}
        </Button>
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
        ref="listContainer"
        role="listbox"
        aria-label="Competition search results"
      >
        <ol class="mx-1 py-1">
          <li
            v-for="(result, index) in data"
            :key="result.id"
            class="rounded-md"
            role="option"
            :aria-selected="index === selectedResult"
          >
            <RouterLink
              :to="`/competition/${result.id}`"
              class="block p-2 rounded-md"
              :class="[
                index === selectedResult ? 'bg-muted' : 'hover:bg-secondary',
              ]"
            >
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
