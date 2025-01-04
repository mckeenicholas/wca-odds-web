<script setup lang="ts">
import { watch, ref, computed } from "vue";
import { useRoute } from "vue-router";
import { fetchWCAInfo } from "../lib/utils";
import { wcif, WCAevent, eventNames } from "../lib/types";
import { Checkbox } from "../components/ui/checkbox";
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from "../components/ui/select";
import { Input } from "../components/ui/input";
import { Button } from "../components/ui/button";
import { Label } from "../components/ui/label";
import {
  NumberField,
  NumberFieldContent,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput,
} from "../components/ui/number-field";
import { useQuery } from "@tanstack/vue-query";
import LoadingMessage from "../components/custom/LoadingMessage.vue";

interface EventRegistration {
  [key: string]: {
    id: string;
    name: string;
    rank: number;
    selected: boolean;
  }[];
}

const route = useRoute();
const selectedCompetitors = ref<EventRegistration>({});
const selectedEventId = ref<string>("");
const simCount = ref<number>(10000);
const monthCount = ref<number>(12);

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
  response.persons.map(
    (person: {
      personalBests: any;
      registration: {
        status: string;
        isCompeting: any;
        eventIds: WCAevent[];
      };
      wcaId: any;
      name: string;
    }) => {
      if (
        person.registration?.status === "accepted" &&
        person.registration?.isCompeting &&
        person.wcaId
      ) {
        person.registration.eventIds.forEach((event: WCAevent) => {
          if (!competitorsByEvent[event]) {
            competitorsByEvent[event] = [];
          }
          const worldRank = person.personalBests.find(
            (personalBest: { eventId: any }) => personalBest.eventId === event,
          )?.worldRanking;
          if (worldRank) {
            competitorsByEvent[event].push({
              id: person.wcaId,
              name: person.name,
              rank: worldRank,
              selected: false,
            });
          }
        });
      }
    },
  );

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
      event: selectedEventId.value,
      simCount: simCount.value.toString(),
      monthCutoff: monthCount.value.toString(),
      competitors: eventSelectedCompetitors.join(","),
    });
    const url = `/simulation?${queryParams.toString()}`;
    window.location.href = url;
  }
};

const eventIds = computed(() => {
  return data.value
    ? data.value.events.map((event: { id: WCAevent }) => event.id)
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
      <Select v-model="selectedEventId">
        <SelectTrigger class="ms-0">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectItem v-for="event of eventIds" :value="event">
            {{ eventNames[event] }}
          </SelectItem>
        </SelectContent>
      </Select>
      <div class="border rounded-md my-2 p-2 flex items-center space-x-4">
        <Label for="simCount">Number of simulations:</Label>
        <Input
          placeholder="100000"
          class="min-w-16 max-w-36"
          id="simCount"
          v-model="simCount"
        />
        <Label for="resultCutoff">Using results from past</Label>
        <NumberField
          v-model="monthCount"
          :default-value="18"
          :min="1"
          id="resultCutoff"
          class="w-24"
        >
          <NumberFieldContent>
            <NumberFieldDecrement />
            <NumberFieldInput />
            <NumberFieldIncrement />
          </NumberFieldContent>
        </NumberField>
        <Label for="resultCutoff">{{
          monthCount === 1 ? "month" : "months"
        }}</Label>
        <!-- <Label for="useDNF">Include DNF rate in calculations:</Label>
        <Checkbox id="useDNF" /> -->
        <div class="flex flex-grow justify-end">
          <Button @click="runSimulation">Run Simulation</Button>
        </div>
      </div>
      <div
        class="max-h-[75vh] rounded-md border min-w-[70vw] overflow-y-scroll"
      >
        <ol>
          <li
            v-for="person in selectedCompetitors[selectedEventId]"
            @click="() => (person.selected = !person.selected)"
            :key="person.id"
            class="p-2 hover:bg-secondary rounded-md flex justify-between items-center"
          >
            <span :class="{ 'text-muted-foreground': !person.selected }">
              <a
                @click.stop
                :href="`https://worldcubeassociation.org/persons/${person.id}`"
                class="hover:underline"
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
  </div>
</template>
