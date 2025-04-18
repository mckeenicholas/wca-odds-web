<script setup lang="ts">
import { ref, computed, watchEffect } from "vue";
import { useRoute, useRouter } from "vue-router";
import { fetchWCAInfo } from "@/lib/utils";
import { wcif, SupportedWCAEvent, Person } from "@/lib/types";
import { Checkbox } from "@/components/ui/checkbox";
import { useQuery } from "@tanstack/vue-query";
import LoadingMessage from "@/components/custom/LoadingMessage.vue";
import ControlPanel from "@/components/custom/ControlPanel.vue";
import FlagIcon from "@/components/custom/FlagIcon.vue";

interface Competitor {
  id: string;
  country: string;
  name: string;
  rank: number;
  selected: boolean;
}

type EventRegistration = Partial<Record<SupportedWCAEvent, Competitor[]>>;

const MAX_COMPETITORS = 64 as const;
const DEFAULT_SELECTED = 16 as const;

const route = useRoute();
const router = useRouter();

const competitorsByEvent = ref<
  Partial<Record<SupportedWCAEvent, Competitor[]>>
>({});
const selectedEventId = ref<SupportedWCAEvent>("333");
const simCount = ref<number>(10000);
const monthCount = ref<number>(12);
const includeDnf = ref<boolean>(true);

const { isPending, isError, data, error } = useQuery({
  queryKey: ["competition", route.params.id],
  queryFn: () =>
    fetchWCAInfo<wcif>(
      `https://api.worldcubeassociation.org/competitions/${route.params.id}/wcif/public`,
    ),
});

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
          competitorAcc[event].push(competitor);
        }
      });
    });

  Object.values(competitorAcc).forEach((competitors) => {
    competitors.sort((a, b) => a.rank - b.rank);
    competitors.forEach((c, i) => (c.selected = i < DEFAULT_SELECTED));
    competitors.splice(MAX_COMPETITORS);
  });

  selectedEventId.value = Object.keys(competitorAcc)[0] as SupportedWCAEvent;

  return competitorAcc;
};

watchEffect(() => {
  competitorsByEvent.value = getCompetitorData();
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

  router.push({
    path: "/simulation",
    query: {
      name: data.value.name,
      eventId: selectedEventId.value,
      simCount: simCount.value.toString(),
      monthCutoff: monthCount.value.toString(),
      includeDnf: includeDnf.value.toString(),
      competitors: selectedIds.join(","),
    },
  });
};

const toggleSelection = (person: Competitor) => {
  person.selected = !person.selected;
};
</script>

<template>
  <div class="flex flex-col items-center justify-center">
    <div v-if="isPending">
      <LoadingMessage message="Loading WCA Data" class="text-2xl m-4" />
    </div>

    <div v-else-if="isError || !data?.name" class="text-red-500">
      Error fetching data: {{ error?.message || "Unknown error occurred" }}
    </div>

    <template v-else>
      <h1 class="text-center text-2xl font-bold m-4">{{ data.name }}</h1>

      <div>
        <ControlPanel
          :event-ids="eventIds"
          v-model:month-count="monthCount"
          v-model:selected-event-id="selectedEventId"
          v-model:sim-count="simCount"
          v-model:include-dnf="includeDnf"
          :disable-run="currentSelectedCompetitors.length < 2"
          @run-simulation="runSimulation"
        />

        <div
          v-if="!competitorsByEvent[selectedEventId]?.length"
          class="text-center m-6 text-lg"
        >
          No one is registered for this event
        </div>

        <ol
          v-else
          class="max-h-[75vh] rounded-md border min-w-[70vw] overflow-y-scroll"
        >
          <li
            v-for="person in competitorsByEvent[selectedEventId]"
            :key="person.id"
            @click="toggleSelection(person)"
            class="p-2 hover:bg-secondary rounded-md flex justify-between items-center"
          >
            <span :class="{ 'text-muted-foreground': !person.selected }">
              <FlagIcon :code="person.country" :muted="!person.selected" />
              <a
                :href="`https://worldcubeassociation.org/persons/${person.id}`"
                class="hover:underline ms-2"
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
