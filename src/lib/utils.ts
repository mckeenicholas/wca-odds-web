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
