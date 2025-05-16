import init, { load_data, run_simulation } from "../../wasm/odds_web";
import type {
  WorkerMessage,
  MainThreadMessage,
  RunSimulationPayload,
  RecalculateSimulationPayload,
} from "./types";

let wasmInitialized = false;
let dataLoaded = false;

async function ensureWasmInitialized(): Promise<void> {
  if (!wasmInitialized) {
    try {
      await init();
      wasmInitialized = true;
    } catch (error) {
      console.error("Error initializing WASM:", error);
      const errorMessage = `Failed to initialize WASM: ${error instanceof Error ? error.message : String(error)}`;
      self.postMessage({ type: "SIMULATION_ERROR", error: errorMessage });
      throw new Error(errorMessage);
    }
  }
}

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
      } = payload as RunSimulationPayload;

      await ensureWasmInitialized();

      const loadResult = await load_data(
        competitorList,
        eventType,
        monthCutoff,
        decayHalfLife,
      );

      if (typeof loadResult === "string" && loadResult.startsWith("Error:")) {
        const errorMessage = `Failed to load competition data in worker: ${loadResult}`;
        console.error(errorMessage);
        self.postMessage({ type: "SIMULATION_ERROR", error: errorMessage });
        dataLoaded = false;
        return;
      }
      if (loadResult !== true) {
        const errorMessage = `Failed to load competition data in worker. Unexpected result: ${loadResult}`;
        console.error(errorMessage);
        self.postMessage({ type: "SIMULATION_ERROR", error: errorMessage });
        dataLoaded = false;
        return;
      }
      dataLoaded = true;

      const results = run_simulation(numSimulations, includeDNF, inputtedTimes);

      const message: MainThreadMessage = {
        type: "SIMULATION_COMPLETE",
        results,
      };
      self.postMessage(message);
    } else if (type === "RECALCULATE_SIMULATION") {
      if (!wasmInitialized) {
        const errorMessage = "WASM not initialized. Cannot recalculate.";
        console.error(errorMessage);
        self.postMessage({ type: "SIMULATION_ERROR", error: errorMessage });
        return;
      }
      if (!dataLoaded) {
        const errorMessage =
          "Data not loaded. Cannot recalculate. Run initial simulation first.";
        console.error(errorMessage);
        self.postMessage({ type: "SIMULATION_ERROR", error: errorMessage });
        return;
      }

      const { numSimulations, includeDNF, inputtedTimes } =
        payload as RecalculateSimulationPayload;
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
