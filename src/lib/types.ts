export const eventNames: { [key: string]: string } = {
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
  "333mbf": "3x3x3 Multi-Blind",
};

export type WCAevent =
  | "222"
  | "333"
  | "444"
  | "555"
  | "666"
  | "777"
  | "333bf"
  | "333fm"
  | "333oh"
  | "minx"
  | "pyram"
  | "clock"
  | "skewb"
  | "sq1"
  | "444bf"
  | "555bf";

export const eventInfo = {
  333: { attempts: 5, format: "a" },
  222: { attempts: 5, format: "a" },
  444: { attempts: 5, format: "a" },
  555: { attempts: 5, format: "a" },
  666: { attempts: 3, format: "m" },
  777: { attempts: 3, format: "m" },
  "333bf": { attempts: 3, format: "b" },
  "333fm": { attempts: 3, format: "m" },
  "333oh": { attempts: 5, format: "a" },
  minx: { attempts: 5, format: "a" },
  pyram: { attempts: 5, format: "a" },
  clock: { attempts: 5, format: "a" },
  skewb: { attempts: 5, format: "a" },
  sq1: { attempts: 5, format: "a" },
  "444bf": { attempts: 3, format: "b" },
  "555bf": { attempts: 3, format: "b" },
};

interface Registration {
  wcaRegistrationId: number;
  eventIds: WCAevent[];
  status: string;
  isCompeting: boolean;
}

interface Person {
  name: string;
  wcaId: string;
  personalBests: any;
  registration: Registration;
}

export interface wcifEvent {
  id: WCAevent;
}

export interface wcif {
  name: string;
  events: wcifEvent[];
  persons: Person[];
}
