import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
}

export type Result<T, E> = { data: T; error: null } | { data: null; error: E };

export function tryCatch<T, E extends Error = Error>(fn: () => T): Result<T, E> {
    try {
        const data = fn();
        return { data, error: null };
    } catch (error) {
        return { data: null, error: error as E };
    }
}

export async function tryCatchAsync<T, E extends Error = Error>(fn: () => Promise<T>): Promise<Result<T, E>> {
    try {
        const data = await fn();
        return { data, error: null };
    } catch (error) {
        return { data: null, error: error as E };
    }
}
