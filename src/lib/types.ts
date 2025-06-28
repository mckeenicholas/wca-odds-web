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

export type SupportedWCAEvent = string; // Placeholder, use your actual type

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
  schedule: {
    startDate: string;
  };
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

// Payloads for worker messages
export interface RunSimulationPayload {
  competitorList: string[];
  event: SupportedWCAEvent;
  startDate: Date;
  endDate: Date;
  numSimulations: number;
  includeDNF: boolean;
  decayHalfLife: number;
  inputtedTimes: number[][];
}

export interface RecalculateSimulationPayload {
  numSimulations: number;
  includeDNF: boolean;
  inputtedTimes: number[][];
}

// Messages from Main Thread to Worker
export type WorkerMessage =
  | { type: "RUN_SIMULATION"; payload: RunSimulationPayload }
  | { type: "RECALCULATE_SIMULATION"; payload: RecalculateSimulationPayload };

// Messages from Worker to Main Thread
export type MainThreadMessage =
  | { type: "SIMULATION_COMPLETE"; results: SimulationResult[] }
  | { type: "SIMULATION_ERROR"; error: string };

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
  startDate?: string;
  endDate?: string;
  includeDnf?: string;
  decayRate?: string;
  competitionId?: string;
  date?: string;
}

export interface SimulationResultProps {
  data: SimulationResult[];
  colors: string[];
  numSimulations: number;
  event: SupportedWCAEvent;
}

export interface WCALiveCompetitionSuccess {
  data: {
    competition: {
      competitionEvents: WCALiveCompetitionEvent[];
    };
  };
}

export interface WCALiveCompetitionError {
  errors: {
    detail: string;
  };
}

export type WCALiveCompetitionData =
  | WCALiveCompetitionSuccess
  | WCALiveCompetitionError;

export interface WCALiveEventRound {
  id: string;
  number: number;
}

export interface WCALiveCompetitionEvent {
  event: {
    id: string;
  };
  rounds: WCALiveEventRound[];
}

export interface WCALiveAttempt {
  result: number;
}

export interface WCALivePerson {
  wcaId: string | null;
}

export interface WCALiveRoundResultItem {
  attempts: WCALiveAttempt[];
  person: WCALivePerson;
}

export interface WCALiveRoundFormat {
  numberOfAttempts: number;
}

export interface WCALiveRoundDetails {
  format: WCALiveRoundFormat;
  results: WCALiveRoundResultItem[];
}

export interface WCALiveRoundDataWrapper {
  round: WCALiveRoundDetails;
}

export interface FetchRoundResultsGraphQLResponse {
  data?: WCALiveRoundDataWrapper;
  errors?: Array<{ message: string; [key: string]: string }>;
}

export interface Competitor {
  id: string;
  country: string;
  name: string;
  rank: number;
  selected: boolean;
}
