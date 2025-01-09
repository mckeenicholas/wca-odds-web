import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import { Competition, Competitor, eventInfo, SupportedWCAEvent } from "./types";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export const fetchWCAInfo = async <T>(url: string | URL): Promise<T> => {
  const response = await fetch(url);
  const data: T = await response.json();
  return data;
};

export const filterCompetitions = (
  competitions: Competition[],
  competitors: Competitor[],
  event: SupportedWCAEvent,
  startDate: Date,
): { id: string; name: string; results: number[] }[] => {
  const numSolves = eventInfo[event].attempts;
  const compDate = createCompetitionDateMap(competitions);

  return competitors.flatMap((person) => {
    const results = extractValidResults(
      person,
      compDate,
      event,
      startDate,
      numSolves,
    );
    return results.length
      ? [{ id: person.id, name: person.name, results }]
      : [];
  });
};

const createCompetitionDateMap = (
  competitions: Competition[],
): Record<string, string> => {
  return competitions.reduce(
    (acc, comp) => {
      acc[comp.id] = comp.date;
      return acc;
    },
    {} as Record<string, string>,
  );
};

const extractValidResults = (
  person: Competitor,
  compDate: Record<string, string>,
  event: SupportedWCAEvent,
  startDate: Date,
  numSolves: number,
): number[] => {
  return Object.entries(person.results)
    .filter(
      ([key, values]) => new Date(compDate[key]) > startDate && values[event],
    )
    .flatMap(([_, values]) =>
      values[event]?.flatMap((round: { solves: number[] }) =>
        round.solves.slice(0, numSolves).filter((solve) => solve !== 0),
      ),
    );
};

export const fetchData = async (
  persons: string[],
  event: SupportedWCAEvent,
  startDate: Date,
): Promise<{ id: string; name: string; results: number[] }[]> => {
  const years = getYearsFromDate(startDate);

  const competitors = await fetchCompetitorData(persons);
  const competitions = await fetchCompetitionData(years);

  return filterCompetitions(competitions, competitors, event, startDate);
};

const fetchCompetitorData = async (
  persons: string[],
): Promise<Competitor[]> => {
  return Promise.all(
    persons.map((person) =>
      fetchWCAInfo<Competitor>(
        `https://raw.githubusercontent.com/robiningelbrecht/wca-rest-api/master/api/persons/${person}.json`,
      ),
    ),
  );
};

const fetchCompetitionData = async (
  years: number[],
): Promise<Competition[]> => {
  const competitionData = await Promise.all(
    years.map(async (year) => {
      const response = await fetchWCAInfo<{
        items: { id: string; date: { from: string } }[];
      }>(
        `https://raw.githubusercontent.com/robiningelbrecht/wca-rest-api/master/api/competitions/${year}.json`,
      );
      return response?.items.map((comp) => ({
        id: comp.id,
        date: comp.date.from,
      }));
    }),
  );
  return competitionData.flat();
};

export const getYearsFromDate = (startDate: Date): number[] => {
  const currentYear = new Date().getFullYear();
  const startYear = startDate.getFullYear();
  return Array.from(
    { length: currentYear - startYear + 1 },
    (_, i) => startYear + i,
  );
};
