import { computed, h, toRaw } from "vue";
import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import {
  ChartTooltipProps,
  SimulationResult,
  SimulationRouteQuery,
  SupportedWCAEvent,
  wcif,
} from "./types";
import HistogramCustomTooltip from "@/components/charts/HistogramCustomTooltip.vue";

export const BREAKPOINT = 1255 as const;

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export const fetchWCIF = async (id: string): Promise<wcif> => {
  const cachedComps = [
    "WC2025",
    "RubiksUKChampionship2025",
    "NorthCarolinaChampionship2025",
  ];
  const wcaURL = cachedComps.includes(id)
    ? `/wcif/${id}.json`
    : `https://api.worldcubeassociation.org/competitions/${id}/wcif/public`;
  return fetchWCAInfo<wcif>(wcaURL);
};

export const fetchWCAInfo = async <T>(url: string | URL): Promise<T> => {
  const response = await fetch(url);
  const data: T = await response.json();
  return data;
};

const hslToHex = (h: number, s: number, l: number) => {
  l /= 100;
  const a = (s * Math.min(l, 1 - l)) / 100;
  const f = (n: number) => {
    const k = (n + h / 30) % 12;
    const color = l - a * Math.max(Math.min(k - 3, 9 - k, 1), -1);
    return Math.round(255 * color)
      .toString(16)
      .padStart(2, "0");
  };
  return `#${f(0)}${f(8)}${f(4)}`;
};

export const generateColors = (num: number) => {
  const hexCodes = [];
  const step = 360 / num;
  for (let i = 0; i < num; i++) {
    const hue = i * step;
    hexCodes.push(hslToHex(hue, 100, 50));
  }
  return hexCodes;
};

export const totalSolves = (results: Map<number, number>) => {
  return Array.from(results.values()).reduce((sum, count) => sum + count, 0);
};

export const toClockFormat = (centiseconds: number): string => {
  if (centiseconds === -1) return "DNF";
  if (centiseconds === -2) return "DNS";
  if (!Number.isFinite(centiseconds)) {
    throw new Error(
      `Invalid centiseconds, expected positive number, got ${centiseconds}.`,
    );
  }
  return new Date(centiseconds * 10)
    .toISOString()
    .substr(11, 11)
    .replace(/^[0:]*(?!\.)/g, "");
};

export const toFMC = (result: number): string => {
  if (result === -1) return "DNF";
  if (result === -2) return "DNS";

  if (result % 100 === 30) {
    return ((result + 3) / 100).toString();
  }

  if (result % 100 === 60) {
    return ((result + 7) / 100).toString();
  }

  return (result / 100).toString();
};

export const renderTime = (time: number, isFMC: boolean) => {
  return isFMC ? toFMC(time) : toClockFormat(time);
};

export function times<T>(n: number, iteratee: (index: number) => T): T[] {
  return Array.from({ length: n }, (_, index) => iteratee(index));
}

export const generateDefaultTimesArray = (
  competitorsCount: number,
  attempts: number,
) => {
  return Array.from({ length: competitorsCount }, () =>
    Array.from({ length: attempts }, () => 0),
  );
};

export const createFMCTooltip = (event: SupportedWCAEvent) =>
  computed(() => {
    return (props: ChartTooltipProps) =>
      h(HistogramCustomTooltip, {
        ...props,
        isFmc: event === "333fm",
      });
  });

export const getNumericValue = (val: string | number): number => {
  if (typeof val === "string") {
    return parseFloat(val);
  }
  return val;
};

export const formatPercentage = (
  val: string | number,
  normalize: boolean = false,
): string => {
  const numVal = getNumericValue(val);
  const pctVal = normalize ? numVal * 100 : numVal;

  return `${pctVal.toFixed(2)}%`;
};

export const getParentPath = (path: string) => {
  const normalizedPath = path.endsWith("/") ? path : path + "/";

  if (normalizedPath.includes("/results/")) {
    return normalizedPath
      .substring(0, normalizedPath.lastIndexOf("/results/"))
      .replace(/\/$/, "");
  } else if (normalizedPath.startsWith("/competition/")) {
    return "/";
  } else if (normalizedPath === "/custom/" || normalizedPath === "/custom") {
    return "/";
  }

  return "/";
};

export const ArrEq2D = (arr1: number[][], arr2: number[][]): boolean => {
  if (arr1 === arr2) return true;
  if (!arr1 || !arr2) return false;
  if (arr1.length !== arr2.length) return false;

  for (let i = 0; i < arr1.length; i++) {
    const row1 = arr1[i];
    const row2 = arr2[i];

    if (row1.length !== row2.length) return false;

    for (let j = 0; j < row1.length; j++) {
      if (row1[j] !== row2[j]) return false;
    }
  }

  return true;
};

export const clone2DArr = (arr: number[][]): number[][] => {
  return arr.map((row) => [...row]);
};

export const formatDate = (date: string | Date | number) =>
  new Date(date).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });

export const buildSimulationQuery = (params: {
  name: string;
  eventId: string;
  simCount: number;
  startDate: Date;
  endDate: Date;
  includeDnf: boolean;
  decayRate: number;
  competitors: string[];
  competitionId?: string;
  date?: string;
}): SimulationRouteQuery => {
  return {
    name: params.name,
    ...(params.competitionId && { competitionId: params.competitionId }),
    ...(params.date && { date: params.date }),
    eventId: params.eventId,
    simCount: params.simCount.toString(),
    startDate: params.startDate.toISOString(),
    endDate: params.endDate.toISOString(),
    includeDnf: params.includeDnf.toString(),
    decayRate: params.decayRate.toString(),
    competitors: params.competitors.join(","),
  };
};

export const createJSONExport = ({
  competitionName,
  results,
  ids,
  currentTimes,
  startDate,
  endDate,
  simCount,
  decayRate,
  includeDnf,
  event,
}: {
  competitionName: string;
  results: SimulationResult[];
  ids: string[];
  currentTimes: number[][];
  startDate: Date;
  endDate: Date;
  simCount: number;
  decayRate: number;
  includeDnf: boolean;
  event: SupportedWCAEvent;
}) => {
  const config = {
    startDate,
    endDate,
    competitionName,
    simCount,
    decayRate,
    includeDnf,
    event,
    generatedOn: new Date(),
  };

  const personResults = results.map((result, index) => ({
    id: ids[index],
    name: result.name,
    winChance: result.win_count / simCount,
    podiumChance: result.pod_count / simCount,
    globalMean: result.mean_no_dnf,
    expectedRank: result.total_rank / simCount,
    rankCount: Object.fromEntries(
      result.rank_dist.map((count, rank) => [rank + 1, count]),
    ),
    histSingle: Object.fromEntries(result.hist_values_single),
    histAverage: Object.fromEntries(result.hist_values_average),
    enteredTimes: currentTimes[index].filter((time) => time !== 0),
  }));

  return JSON.stringify({
    config,
    results: personResults,
  });
};

export const createCSVExport = (
  results: SimulationResult[],
  ids: string[],
  currentTimes: number[][],
  simCount: number,
) => {
  const headers = [
    "id",
    "name",
    "win_chance",
    "podium_chance",
    "global_mean",
    "expected_rank",
    "time_1",
    "time_2",
    "time_3",
    "time_4",
    "time_5",
  ];

  const rows = results.map((result, idx) => {
    return [
      ids[idx],
      result.name,
      result.win_count / simCount,
      result.pod_count / simCount,
      result.mean_no_dnf,
      result.total_rank / simCount,
      ...currentTimes[idx],
    ].join(",");
  });

  return [headers.join(","), ...rows].join("\n");
};

export const downloadTextBlob = (
  data: string,
  filename: string,
  mimeType: string,
): void => {
  const blob = new Blob([data], { type: mimeType });
  const url = URL.createObjectURL(blob);

  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = filename;
  anchor.style.display = "none";

  document.body.appendChild(anchor);
  anchor.click();
  document.body.removeChild(anchor);

  URL.revokeObjectURL(url);
};

export const formatInputtedTimes = (
  inputtedTimes: number[][],
  event: SupportedWCAEvent,
) => {
  const timesArray = toRaw(inputtedTimes);

  if (event != "333fm") {
    return timesArray;
  }

  return timesArray.map((person) => person.map((time) => time * 100));
};
