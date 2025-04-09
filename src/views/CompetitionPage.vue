<script setup lang="ts">
import { watch, ref, computed, onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import { fetchWCAInfo } from "@/lib/utils";
import { wcif, SupportedWCAEvent, Person } from "@/lib/types";
import { Checkbox } from "@/components/ui/checkbox";
import { useQuery } from "@tanstack/vue-query";
import LoadingMessage from "@/components/custom/LoadingMessage.vue";
import ControlPanel from "@/components/custom/ControlPanel.vue";
import FlagIcon from "@/components/custom/FlagIcon.vue";
import BackButton from "@/components/custom/BackButton.vue";

interface Competitor {
  id: string;
  country: string;
  name: string;
  rank: number;
  selected: boolean;
}

interface EventRegistration {
  [key: string]: Competitor[];
}

const route = useRoute();
const router = useRouter();

const selectedCompetitors = ref<EventRegistration>({});
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

const numSelected = computed(
  () =>
    selectedCompetitors.value[selectedEventId.value]?.filter((p) => p.selected)
      .length ?? 0,
);

const currentCompetitors = computed(
  () => selectedCompetitors.value[selectedEventId.value] ?? [],
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

const runSimulation = () => {
  if (!data.value) return;

  const selectedIds = currentCompetitors.value
    .filter((item) => item.selected)
    .map((item) => item.id);

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

const updateSelected = () => {
  if (isError.value || !data.value) return;

  selectedEventId.value = data.value.events[0].id;
  const competitorsByEvent: EventRegistration = {};

  data.value.persons
    .filter(
      (person) =>
        person.registration?.status === "accepted" &&
        person.registration?.isCompeting &&
        person.wcaId,
    )
    .forEach((person) => {
      person.registration.eventIds.forEach((event: SupportedWCAEvent) => {
        if (!competitorsByEvent[event]) {
          competitorsByEvent[event] = [];
        }

        const competitor = processCompetitor(person, event);
        if (competitor) {
          competitorsByEvent[event].push(competitor);
        }
      });
    });

  Object.values(competitorsByEvent).forEach((competitors) => {
    competitors.sort((a, b) => a.rank - b.rank);
    competitors.forEach((c, i) => (c.selected = i < 16));
    competitors.splice(64);
  });

  selectedCompetitors.value = competitorsByEvent;
};

watch(data, updateSelected);

onMounted(updateSelected);
</script>

<template>
  <BackButton />
  <div class="flex flex-col items-center justify-center">
    <div v-if="isPending">
      <LoadingMessage message="Loading WCA Data" class="text-2xl m-4" />
    </div>

    <div v-else-if="isError || !data?.name" class="text-red-500">
      Error fetching data: {{ error }}
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
          :disable-run="numSelected < 2"
          @run-simulation="runSimulation"
        />

        <div v-if="!currentCompetitors.length" class="text-center m-6 text-lg">
          No one is registered for this event
        </div>

        <ol
          v-else
          class="max-h-[75vh] rounded-md border min-w-[70vw] overflow-y-scroll"
        >
          <li
            v-for="person in currentCompetitors"
            :key="person.id"
            @click="person.selected = !person.selected"
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
            />
          </li>
        </ol>
      </div>
    </template>
  </div>
</template>
