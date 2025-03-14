import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
}

export function isType<T>(value: any, field: string): value is T {
    if (value === null || typeof value !== "object") {
        return false;
    }
    return field in value;
}
