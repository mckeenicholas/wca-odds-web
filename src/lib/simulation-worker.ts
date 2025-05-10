import init, { load_data, run_simulation } from "../../wasm/odds_web";
import type { WorkerMessage, MainThreadMessage } from "./types";

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
        decayHalfLife,
        inputtedTimes,
      } = payload;

      try {
        await init();
      } catch (error) {
        console.error("Error initializing WASM:", error);
        self.postMessage({
          type: "SIMULATION_ERROR",
          error: `Failed to initialize WASM: ${error instanceof Error ? error.message : String(error)}`,
        });
        return;
      }

      const result = await load_data(
        competitorList,
        eventType,
        monthCutoff,
        decayHalfLife,
      );

      if (!result) {
        const errorMessage = "Failed to load competition data in worker.";
        console.error(errorMessage);
        self.postMessage({
          type: "SIMULATION_ERROR",
          error: errorMessage,
        });
        return;
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
