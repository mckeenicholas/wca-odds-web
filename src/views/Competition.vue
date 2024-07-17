<script setup lang="ts">
import { watch, ref, onMounted } from "vue";
import { useRoute } from "vue-router";
import {
  fetchWCAInfo,
  wcif,
  wcifEvent,
  eventNames,
  WCAEvent,
} from "@/lib/utils";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Checkbox } from "@/components/ui/checkbox";
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from "@/components/ui/select";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";

interface EventRegistration {
  [key: string]: {
    id: string;
    name: string;
    rank: number;
    selected: boolean;
  }[];
}

const route = useRoute();
const wcif = ref<wcif | null>(null);
const selectedCompetitors = ref<EventRegistration>({});
const selectedEventId = ref<string>("");

const defaultShownNum = 64;
const defaultSelectedNum = 16;

const fetchCompetitors = async (compid: string) => {
  const response = await fetchWCAInfo<wcif>(
    `https://api.worldcubeassociation.org/competitions/${compid}/wcif/public`,
  );
  wcif.value = response;
  selectedEventId.value = response.events[0].id;
  if (response) {
    let competitorsByEvent: EventRegistration = {};
    response.persons.map(
      (person: {
        personalBests: any;
        registration: {
          status: string;
          isCompeting: any;
          eventIds: WCAEvent[];
        };
        wcaId: any;
        name: any;
      }) => {
        if (
          person.registration?.status === "accepted" &&
          person.registration?.isCompeting &&
          person.wcaId
        ) {
          person.registration.eventIds.forEach((event: WCAEvent) => {
            if (!competitorsByEvent[event]) {
              competitorsByEvent[event] = [];
            }
            const worldRank = person.personalBests.find(
              (personalBest: { eventId: any }) =>
                personalBest.eventId === event,
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
        (competitor, index) =>
          (competitor.selected = index < defaultSelectedNum),
      );
      competitors.splice(defaultShownNum); // Keep only the top n competitors
    });

    selectedCompetitors.value = competitorsByEvent;
  }
};

const runSimulation = () => {
  const eventSelectedCompetitors = selectedCompetitors.value[
    selectedEventId.value
  ]
    .filter((item) => item.selected)
    .map((item) => item.id);
  const queryParams = new URLSearchParams({
    name: wcif.value.name,
    event: selectedEventId.value,
    competitors: eventSelectedCompetitors.join(","),
  });
  const url = `/simulation?${queryParams.toString()}`;
  window.location.href = url;
};

onMounted(() => {
  const initialId = route.params.id;
  if (initialId) {
    fetchCompetitors(initialId as string);
  }
});

watch(
  () => route.params.id,
  (newId, _) => {
    console.log(newId);
    fetchCompetitors(newId as string);
  },
);
</script>

<template>
  <div class="flex flex-col items-center justify-center">
    <div v-if="wcif">
      <h1 class="text-center">{{ wcif.name }}</h1>
      <Select v-model="selectedEventId">
        <SelectTrigger class="ms-0">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectItem
            v-for="event of wcif.events.map((event: wcifEvent) => event.id)"
            :value="event"
          >
            {{ eventNames[event] }}
          </SelectItem>
        </SelectContent>
      </Select>
      <div class="border rounded-md my-2 p-2 flex items-center space-x-4">
        <Label for="simCount">Number of simulations:</Label>
        <Input placeholder="100000" class="min-w-16 max-w-36" id="simCount" />
        <Label for="useDNF">Include DNF rate in calculations:</Label>
        <Checkbox id="useDNF" />
        <div class="flex flex-grow justify-end">
          <Button @click="runSimulation">Run Simulation</Button>
        </div>
      </div>
      <ScrollArea class="h-[85vh] rounded-md border min-w-[70vw]">
        <ol>
          <li
            v-for="person in selectedCompetitors[selectedEventId]"
            @click="() => (person.selected = !person.selected)"
            :key="person.id"
            class="p-2 hover:bg-secondary rounded-md flex justify-between items-center"
          >
            <span :class="{ 'text-muted-foreground': !person.selected }">{{
              person.name
            }}</span>
            <Checkbox
              v-model:checked="person.selected"
              @click.stop
              class="me-2"
            />
          </li>
        </ol>
      </ScrollArea>
    </div>
  </div>
</template>
