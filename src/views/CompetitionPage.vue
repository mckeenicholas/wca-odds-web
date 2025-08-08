<script setup lang="ts">
import ControlPanel from "@/components/custom/ControlPanel.vue";
import FlagIcon from "@/components/custom/FlagIcon.vue";
import LoadingMessage from "@/components/custom/LoadingMessage.vue";
import { Checkbox } from "@/components/ui/checkbox";
import { useCompSettingsStore } from "@/lib/stores/compSettings";
import { Competitor, Person, SupportedWCAEvent } from "@/lib/types";
import { BREAKPOINT, buildSimulationQuery, fetchWCIF } from "@/lib/utils";
import { useQuery } from "@tanstack/vue-query";
import { useWindowSize } from "@vueuse/core";
import { storeToRefs } from "pinia";
import { computed, watch, watchEffect } from "vue";
import { useRoute, useRouter } from "vue-router";

type EventRegistration = Partial<Record<SupportedWCAEvent, Competitor[]>>;

const MAX_COMPETITORS = 64 as const;
const DEFAULT_SELECTED = 16 as const;

const route = useRoute();
const router = useRouter();

const store = useCompSettingsStore();
const {
  compId,
  competitorsByEvent,
  selectedEventId,
  simCount,
  includeDnf,
  decayHalfLife,
  startDate,
  endDate,
} = storeToRefs(store);

const { width } = useWindowSize();

const { isPending, isError, data, error } = useQuery({
  queryKey: ["competition", route.params.id],
  queryFn: () => fetchWCIF(route.params.id as string),
  staleTime: Infinity,
});

watch(
  () => route.params.id,
  (newId) => {
    if (newId && newId !== compId.value) {
      store.reset();
      compId.value = newId as string;
    }
  },
  { immediate: true },
);

const eventIds = computed(
  () => data.value?.events.map((event) => event.id) ?? [],
);

const processCompetitor = (
  person: Person,
  event: SupportedWCAEvent,
): Competitor | null => {
  const worldRank = person.personalBests.find(
    (pb) => pb.eventId === event,
  )?.worldRanking;
  if (!worldRank) return null;

  return {
    id: person.wcaId,
    country: person.countryIso2,
    name: person.name,
    rank: worldRank,
    selected: false,
  };
};

const getCompetitorData = () => {
  if (isError.value || !data.value) return {};

  const competitorAcc: EventRegistration = {};

  data.value.persons
    .filter(
      (person) =>
        person.registration?.status === "accepted" &&
        person.registration?.isCompeting &&
        person.wcaId,
    )
    .forEach((person) => {
      person.registration.eventIds.forEach((event: SupportedWCAEvent) => {
        if (!competitorAcc[event]) {
          competitorAcc[event] = [];
        }

        const competitor = processCompetitor(person, event);
        if (competitor) {
          competitorAcc[event]!.push(competitor);
        }
      });
    });

  Object.keys(competitorAcc).forEach((eventId) => {
    const competitors = competitorAcc[eventId as SupportedWCAEvent];
    if (competitors && competitors.length > 0) {
      competitors.sort((a, b) => a.rank - b.rank);
      competitors.forEach((c, i) => (c.selected = i < DEFAULT_SELECTED));

      if (competitors.length > MAX_COMPETITORS) {
        competitorAcc[eventId as SupportedWCAEvent] = competitors.slice(
          0,
          MAX_COMPETITORS,
        );
      }
    }
  });

  if (!("333" in competitorAcc)) {
    selectedEventId.value = Object.keys(competitorAcc)[0] as SupportedWCAEvent;
  }

  return competitorAcc;
};

watchEffect(() => {
  // Only populate competitor data if we have data from the API
  // and the store hasn't been populated for this competition yet.
  if (data.value && Object.keys(competitorsByEvent.value).length === 0) {
    competitorsByEvent.value = getCompetitorData();
  }
});

const currentSelectedCompetitors = computed(
  () =>
    competitorsByEvent.value[selectedEventId.value]?.filter(
      (competitor) => competitor.selected,
    ) ?? [],
);

const runSimulation = () => {
  if (!data.value) return;

  const selectedIds = currentSelectedCompetitors.value.map((item) => item.id);

  const query = buildSimulationQuery({
    name: data.value.name,
    eventId: selectedEventId.value,
    simCount: simCount.value,
    startDate: startDate.value,
    endDate: endDate.value,
    includeDnf: includeDnf.value,
    decayRate: decayHalfLife.value,
    competitors: selectedIds,
    competitionId: data.value.id,
    date: data.value.schedule.startDate,
  });

  router.push({
    path: `./${data.value.id}/results`,
    query: query as Record<string, string>,
  });
};

const toggleSelection = (person: Competitor) => {
  person.selected = !person.selected;
};
</script>

<template>
  <div class="flex flex-col items-center justify-center">
    <div v-if="isPending">
      <LoadingMessage message="Loading WCA Data" class="m-4 text-2xl" />
    </div>

    <div v-else-if="isError || !data?.name" class="text-red-500">
      Error fetching data: {{ error?.message || "Unknown error occurred" }}
    </div>

    <template v-else>
      <h1 class="m-4 text-center text-2xl font-bold">{{ data.name }}</h1>

      <div>
        <ControlPanel
          :event-ids="eventIds"
          v-model:selected-event-id="selectedEventId"
          v-model:sim-count="simCount"
          v-model:include-dnf="includeDnf"
          v-model:decay-rate="decayHalfLife"
          v-model:start-date="startDate"
          v-model:end-date="endDate"
          :disable-run="currentSelectedCompetitors.length < 2"
          @run-simulation="runSimulation"
        />

        <div
          v-if="!competitorsByEvent[selectedEventId]?.length"
          class="m-6 text-center text-lg"
        >
          No one is registered for this event
        </div>

        <ol
          v-else
          class="flex-1 overflow-y-auto rounded-md border"
          :class="{
            'max-h-[72vh]': width > BREAKPOINT,
            'max-h-[64vh]': width <= BREAKPOINT,
          }"
        >
          <li
            v-for="person in competitorsByEvent[selectedEventId]"
            :key="person.id"
            @click="toggleSelection(person)"
            class="flex items-center justify-between rounded-md p-2 hover:bg-secondary"
          >
            <span :class="{ 'text-muted-foreground': !person.selected }">
              <FlagIcon :code="person.country" :muted="!person.selected" />
              <a
                :href="`https://worldcubeassociation.org/persons/${person.id}?event=${selectedEventId}`"
                class="ms-2 hover:underline"
                @click.stop
              >
                {{ person.name }}
              </a>
            </span>
            <Checkbox
              v-model:checked="person.selected"
              @click.stop
              class="me-2"
              aria-label="Select competitor {{ person.name }}"
            />
          </li>
        </ol>
      </div>
    </template>
  </div>
</template>
