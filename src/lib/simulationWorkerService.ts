import type { SimulationResult, SupportedWCAEvent } from "@/lib/types";
import type { WorkerMessage, MainThreadMessage } from "./types";
import { toRaw } from "vue";

let worker: Worker | null = null;

export const getSimulationWorker = (): Worker | null => {
  if (!worker && typeof Worker !== "undefined") {
    worker = new Worker(new URL("./simulation-worker.ts", import.meta.url), {
      type: "module",
    });
  }
  return worker;
};

export const terminateSimulationWorker = (): void => {
  if (worker) {
    worker.terminate();
    worker = null;
  }
};

export const runSimulationInWorker = (
  competitorList: string[],
  event: SupportedWCAEvent,
  monthCutoff: number,
  numSimulations: number,
  includeDNF: boolean,
  inputtedTimes: number[][],
): Promise<SimulationResult[]> => {
  return new Promise((resolve, reject) => {
    const workerInstance = getSimulationWorker();

    if (!workerInstance) {
      reject(new Error("Web Workers are not supported in this environment"));
      return;
    }

    const messageHandler = (e: MessageEvent<MainThreadMessage>) => {
      if (e.data.type === "SIMULATION_COMPLETE") {
        workerInstance.removeEventListener("message", messageHandler);
        resolve(e.data.results);
      }
    };

    const errorHandler = (error: ErrorEvent) => {
      workerInstance.removeEventListener("message", messageHandler);
      workerInstance.removeEventListener("error", errorHandler);
      reject(new Error(`Worker error: ${error.message}`));
    };

    workerInstance.addEventListener("message", messageHandler);
    workerInstance.addEventListener("error", errorHandler);

    const message: WorkerMessage = {
      type: "RUN_SIMULATION",
      payload: {
        competitorList,
        event,
        monthCutoff,
        numSimulations,
        includeDNF,
        inputtedTimes: toRaw(inputtedTimes),
      },
    };

    workerInstance.postMessage(message);
  });
};
