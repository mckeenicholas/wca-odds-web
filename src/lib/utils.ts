import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export const fetchWCAInfo = async <T>(url: string | URL) => {
  try {
    const response = await fetch(url);
    if (response.ok) {
      const data: T = await response.json();
      return data;
    } else {
      console.error("Error fetching competitions:", response.statusText);
      return null;
    }
  } catch (error) {
    console.error("Error fetching competitions:", error);
    return null;
  }
};

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

interface Registration {
  wcaRegistrationId: number;
  eventIds: string[];
  status: string;
  isCompeting: boolean;
}

interface Person {
  name: string;
  wcaId: string;
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
