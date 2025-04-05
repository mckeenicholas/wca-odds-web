<script setup lang="ts">
import { watch, ref, computed } from "vue";
import { useRoute } from "vue-router";
import { fetchWCAInfo } from "@/lib/utils";
import { wcif, SupportedWCAEvent } from "@/lib/types";
import { Checkbox } from "@/components/ui/checkbox";
import { useQuery } from "@tanstack/vue-query";
import LoadingMessage from "@/components/custom/LoadingMessage.vue";
import ControlPanel from "@/components/custom/ControlPanel.vue";
import FlagIcon from "@/components/custom/FlagIcon.vue";

interface EventRegistration {
  [key: string]: {
    id: string;
    name: string;
    country: string;
    rank: number;
    selected: boolean;
  }[];
}

const route = useRoute();
const selectedCompetitors = ref<EventRegistration>({});
const selectedEventId = ref<string>("");
const simCount = ref<number>(10000);
const monthCount = ref<number>(12);
const includeDnf = ref<boolean>(false);

const defaultShownNum = 64;
const defaultSelectedNum = 16;

const { isPending, isError, data, error } = useQuery({
  queryKey: ["competition", route.params.id],
  queryFn: () =>
    fetchWCAInfo<wcif>(
      `https://api.worldcubeassociation.org/competitions/${route.params.id}/wcif/public`,
    ),
});

watch(data, () => {
  const response = data.value;
  if (isError.value || !response) {
    return;
  }

  selectedEventId.value = response.events[0].id;
  let competitorsByEvent: EventRegistration = {};

  response.persons.forEach((person) => {
    if (
      person.registration?.status === "accepted" &&
      person.registration?.isCompeting &&
      person.wcaId
    ) {
      person.registration.eventIds.forEach((event: SupportedWCAEvent) => {
        if (!competitorsByEvent[event]) {
          competitorsByEvent[event] = [];
        }

        const worldRank = person.personalBests.find(
          (personalBest) => personalBest.eventId === event,
        )?.worldRanking;

        if (worldRank) {
          competitorsByEvent[event].push({
            id: person.wcaId,
            country: person.countryIso2,
            name: person.name,
            rank: worldRank,
            selected: false,
          });
        }
      });
    }
  });

  Object.values(competitorsByEvent).forEach((competitors) => {
    competitors.sort((a, b) => a.rank - b.rank);

    competitors.forEach(
      (competitor, index) => (competitor.selected = index < defaultSelectedNum),
    );

    competitors.splice(defaultShownNum);
  });

  selectedCompetitors.value = competitorsByEvent;
});

const runSimulation = () => {
  if (data.value) {
    const eventSelectedCompetitors = selectedCompetitors.value[
      selectedEventId.value
    ]
      .filter((item) => item.selected)
      .map((item) => item.id);
    const queryParams = new URLSearchParams({
      name: data.value.name,
      eventId: selectedEventId.value,
      simCount: simCount.value.toString(),
      monthCutoff: monthCount.value.toString(),
      includeDnf: includeDnf.value.toString(), // Updated to match new naming
      competitors: eventSelectedCompetitors.join(","),
    });
    const url = `/simulation?${queryParams.toString()}`;
    window.location.href = url;
  }
};

const eventIds = computed(() => {
  return data.value
    ? data.value.events.map((event: { id: SupportedWCAEvent }) => event.id)
    : [];
});
</script>

<template>
  <div class="flex flex-col items-center justify-center">
    <div v-if="isPending" class="text-2xl m-4">
      <LoadingMessage message="Loading WCA Data" />
    </div>
    <div v-else-if="isError || !data?.name">
      Error fetching data: {{ error }}
    </div>
    <div v-else>
      <h1 class="text-center text-2xl font-bold m-4">
        {{ data.name }}
      </h1>
      <div class="min-w-[70vw]">
        <ControlPanel
          v-bind:event-ids="eventIds"
          v-model:month-count="monthCount"
          v-model:selected-event-id="selectedEventId"
          v-model:sim-count="simCount"
          v-model:include-dnf="includeDnf"
          @run-simulation="runSimulation"
        />
      </div>
      <ol class="max-h-[75vh] rounded-md border min-w-[70vw] overflow-y-scroll">
        <li
          v-for="person in selectedCompetitors[selectedEventId]"
          @click="() => (person.selected = !person.selected)"
          :key="person.id"
          class="p-2 hover:bg-secondary rounded-md flex justify-between items-center"
        >
          <span :class="{ 'text-muted-foreground': !person.selected }">
            <FlagIcon :code="person.country" :muted="!person.selected" />
            <a
              @click.stop
              :href="`https://worldcubeassociation.org/persons/${person.id}`"
              class="hover:underline ms-2"
            >
              {{ person.name }}
            </a></span
          >
          <Checkbox
            v-model:checked="person.selected"
            @click.stop
            class="me-2"
          />
        </li>
      </ol>
    </div>
  </div>
</template>
