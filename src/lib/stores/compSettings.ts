import { defineStore } from "pinia";
import { ref } from "vue";
import type { SupportedWCAEvent, Competitor } from "@/lib/types";

export const useCompSettingsStore = defineStore("compSettings", () => {
  const compId = ref<string>();

  // Default values
  const defaultValues = {
    competitorsByEvent: () => ({}),
    selectedEventId: () => "333" as SupportedWCAEvent,
    simCount: () => 10000,
    includeDnf: () => true,
    decayHalfLife: () => 180,
    startDate: () =>
      new Date(new Date().setFullYear(new Date().getFullYear() - 1)),
    endDate: () => new Date(),
  };

  // State
  const competitorsByEvent = ref<
    Partial<Record<SupportedWCAEvent, Competitor[]>>
  >(defaultValues.competitorsByEvent());
  const selectedEventId = ref<SupportedWCAEvent>(
    defaultValues.selectedEventId(),
  );
  const simCount = ref<number>(defaultValues.simCount());
  const includeDnf = ref<boolean>(defaultValues.includeDnf());
  const decayHalfLife = ref<number>(defaultValues.decayHalfLife());
  const startDate = ref<Date>(defaultValues.startDate());
  const endDate = ref<Date>(defaultValues.endDate());

  function reset() {
    competitorsByEvent.value = defaultValues.competitorsByEvent();
    selectedEventId.value = defaultValues.selectedEventId();
    simCount.value = defaultValues.simCount();
    includeDnf.value = defaultValues.includeDnf();
    decayHalfLife.value = defaultValues.decayHalfLife();
    startDate.value = defaultValues.startDate();
    endDate.value = defaultValues.endDate();
  }

  return {
    compId,
    competitorsByEvent,
    selectedEventId,
    simCount,
    includeDnf,
    decayHalfLife,
    startDate,
    endDate,
    reset,
  };
});
