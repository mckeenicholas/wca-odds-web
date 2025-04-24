export const eventNames: Record<SupportedWCAEvent, string> = {
  "333": "3x3x3 Cube",
  "222": "2x2x2 Cube",
  "444": "4x4x4 Cube",
  "555": "5x5x5 Cube",
  "666": "6x6x6 Cube",
  "777": "7x7x7 Cube",
  "333bf": "3x3x3 Blindfolded",
  "333fm": "3x3x3 Fewest Moves",
  "333oh": "3x3x3 One-Handed",
  minx: "Megaminx",
  pyram: "Pyraminx",
  clock: "Clock",
  skewb: "Skewb",
  sq1: "Square-1",
  "444bf": "4x4x4 Blindfolded",
  "555bf": "5x5x5 Blindfolded",
};

export const eventAttempts: Record<SupportedWCAEvent, number> = {
  "333": 5,
  "222": 5,
  "444": 5,
  "555": 5,
  "666": 3,
  "777": 3,
  "333bf": 3,
  "333fm": 3,
  "333oh": 5,
  minx: 5,
  pyram: 5,
  clock: 5,
  skewb: 5,
  sq1: 5,
  "444bf": 3,
  "555bf": 3,
};

export type SupportedWCAEvent = (typeof supportedWCAEvents)[number];

export const supportedWCAEvents = [
  "222",
  "333",
  "444",
  "555",
  "666",
  "777",
  "333bf",
  "333fm",
  "333oh",
  "minx",
  "pyram",
  "clock",
  "skewb",
  "sq1",
  "444bf",
  "555bf",
] as const;

interface Registration {
  wcaRegistrationId: number;
  eventIds: SupportedWCAEvent[];
  status: string;
  isCompeting: boolean;
}

export interface Person {
  name: string;
  wcaId: string;
  countryIso2: string;
  personalBests: PersonalBest[];
  registration: Registration;
}

interface PersonalBest {
  eventId: string;
  best: number;
  worldRanking: number;
}

export interface wcifEvent {
  id: SupportedWCAEvent;
}

export interface wcif {
  name: string;
  id: string;
  events: wcifEvent[];
  persons: Person[];
}

export interface SimulationResult {
  name: string;
  sample_size: number;
  win_count: number;
  pod_count: number;
  total_rank: number;
  mean_no_dnf: number;
  rank_dist: number[];
  hist_values_single: Map<number, number>;
  hist_values_average: Map<number, number>;
}

export interface ChartTooltipProps {
  title?: string;
  data: {
    name: string;
    color: string;
    value: number;
  }[];
  isFmc?: boolean;
}

export interface SimulationRouteQuery {
  competitors?: string;
  eventId?: string;
  name?: string;
  simCount?: string;
  monthCutoff?: string;
  includeDnf?: string;
}

export interface SimulationResultProps {
  data: SimulationResult[];
  colors: string[];
  numSimulations: number;
  event: SupportedWCAEvent;
}

export interface WorkerMessage {
  type: "RUN_SIMULATION";
  payload: {
    competitorList: string[];
    event: SupportedWCAEvent;
    monthCutoff: number;
    numSimulations: number;
    includeDNF: boolean;
    decayHalfLife: number;
    inputtedTimes: number[][];
  };
}

export interface MainThreadMessage {
  type: "SIMULATION_COMPLETE";
  results: SimulationResult[];
  error?: Error;
}
