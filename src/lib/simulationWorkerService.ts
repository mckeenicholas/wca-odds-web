import type {
  SimulationResult,
  SupportedWCAEvent,
  WorkerMessage,
  MainThreadMessage,
  RunSimulationPayload,
  RecalculateSimulationPayload,
} from "./types"; // Ensure RecalculateSimulationPayload is imported
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

const createWorkerPromise = (
  message: WorkerMessage,
): Promise<SimulationResult[]> => {
  return new Promise((resolve, reject) => {
    const workerInstance = getSimulationWorker();

    if (!workerInstance) {
      reject(new Error("Web Workers are not supported in this environment."));
      return;
    }

    const handleMessage = (e: MessageEvent<MainThreadMessage>) => {
      if (e.data.type === "SIMULATION_COMPLETE") {
        workerInstance.removeEventListener("message", handleMessage);
        workerInstance.removeEventListener("error", handleError);
        resolve(e.data.results);
      } else if (e.data.type === "SIMULATION_ERROR") {
        workerInstance.removeEventListener("message", handleMessage);
        workerInstance.removeEventListener("error", handleError);
        reject(new Error(`Simulation worker error: ${e.data.error}`));
      }
    };

    const handleError = (error: ErrorEvent) => {
      workerInstance.removeEventListener("message", handleMessage);
      workerInstance.removeEventListener("error", handleError);
      reject(new Error(`Web Worker error: ${error.message}`));
    };

    workerInstance.addEventListener("message", handleMessage);
    workerInstance.addEventListener("error", handleError);

    workerInstance.postMessage(message);
  });
};

export const runSimulationInWorker = (
  competitorList: string[],
  event: SupportedWCAEvent,
  startDate: Date,
  endDate: Date,
  numSimulations: number,
  includeDNF: boolean,
  decayHalfLife: number,
  inputtedTimes: number[][],
): Promise<SimulationResult[]> => {
  const payload: RunSimulationPayload = {
    competitorList,
    event,
    startDate,
    endDate,
    numSimulations,
    includeDNF,
    decayHalfLife,
    inputtedTimes: toRaw(inputtedTimes),
  };
  const message: WorkerMessage = {
    type: "RUN_SIMULATION",
    payload,
  };
  return createWorkerPromise(message);
};

export const recalculateSimulationInWorker = (
  numSimulations: number,
  includeDNF: boolean,
  inputtedTimes: number[][],
): Promise<SimulationResult[]> => {
  const payload: RecalculateSimulationPayload = {
    numSimulations,
    includeDNF,
    inputtedTimes: toRaw(inputtedTimes),
  };
  const message: WorkerMessage = {
    type: "RECALCULATE_SIMULATION",
    payload,
  };
  return createWorkerPromise(message);
};
