import init, { load_data, run_simulation } from "../../wasm/odds_web";
import type { WorkerMessage, MainThreadMessage } from "./types";
import { SupportedWCAEvent } from "@/lib/types.js";

let initialized = false;
let loadArgs: {
  competitorList: string[];
  event: SupportedWCAEvent;
  monthCutoff: number;
} = {
  competitorList: [],
  event: "333" as SupportedWCAEvent,
  monthCutoff: 0,
};

const argsHaveChanged = (
  competitorList: string[],
  event: SupportedWCAEvent,
  monthCutoff: number,
): boolean => {
  const sameCompetitors =
    competitorList.length === loadArgs.competitorList.length &&
    competitorList.every((id, i) => id === loadArgs.competitorList[i]);

  return (
    !sameCompetitors ||
    event !== loadArgs.event ||
    monthCutoff !== loadArgs.monthCutoff
  );
};

const updateLoadArgs = (
  competitorList: string[],
  event: SupportedWCAEvent,
  monthCutoff: number,
): void => {
  loadArgs = {
    competitorList: [...competitorList],
    event,
    monthCutoff,
  };
};

self.onmessage = async (event: MessageEvent<WorkerMessage>) => {
  try {
    const { type, payload } = event.data;

    if (type === "RUN_SIMULATION") {
      const {
        competitorList,
        event: eventType,
        monthCutoff,
        numSimulations,
        includeDNF,
        inputtedTimes,
      } = payload;

      if (!initialized) {
        try {
          await init();
          initialized = true;
        } catch (error) {
          console.error("Error initializing WASM:", error);
          self.postMessage({
            type: "SIMULATION_ERROR",
            error: `Failed to initialize WASM: ${error instanceof Error ? error.message : String(error)}`,
          });
          return;
        }
      }

      if (argsHaveChanged(competitorList, eventType, monthCutoff)) {
        const result = await load_data(competitorList, eventType, monthCutoff);

        if (!result) {
          const errorMessage = "Failed to load competition data in worker.";
          console.error(errorMessage);
          self.postMessage({
            type: "SIMULATION_ERROR",
            error: errorMessage,
          });
          return;
        }

        updateLoadArgs(competitorList, eventType, monthCutoff);
      }

      const results = run_simulation(numSimulations, includeDNF, inputtedTimes);

      const message: MainThreadMessage = {
        type: "SIMULATION_COMPLETE",
        results,
      };

      self.postMessage(message);
    }
  } catch (error) {
    console.error("Error in worker:", error);
    self.postMessage({
      type: "SIMULATION_ERROR",
      error: error instanceof Error ? error.message : String(error),
    });
  }
};
