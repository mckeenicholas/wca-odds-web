import type { Updater } from "@tanstack/vue-table";
import { computed, h, type Ref } from "vue";
import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import { times } from "lodash-es";
import { ChartTooltipProps, SupportedWCAEvent, wcif } from "./types";
import HistogramCustomTooltip from "@/components/charts/HistogramCustomTooltip.vue";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function valueUpdater<T extends Updater<unknown>>(
  updaterOrValue: T,
  ref: Ref,
) {
  ref.value =
    typeof updaterOrValue === "function"
      ? updaterOrValue(ref.value)
      : updaterOrValue;
}

export const fetchWCIF = async (id: string): Promise<wcif> => {
  const cachedComps = ["WC2025"];

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

export const generateDefaultTimesArray = (
  competitorsCount: number,
  attempts: number,
) => {
  return times(competitorsCount, () => times(attempts, () => 0));
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

export const formatPercentage = (val: string | number): string => {
  const numVal = getNumericValue(val);

  if (numVal < 0.01) {
    return "<0.01%";
  }

  return `${numVal.toFixed(2)}%`;
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
