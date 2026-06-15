type ClassValue =
  | string
  | number
  | null
  | false
  | undefined
  | ClassValue[]
  | Record<string, boolean | null | undefined>;

function toClasses(value: ClassValue): string[] {
  if (!value) return [];
  if (typeof value === "string" || typeof value === "number") {
    return [String(value)];
  }
  if (Array.isArray(value)) {
    return value.flatMap(toClasses);
  }
  return Object.entries(value)
    .filter(([, enabled]) => Boolean(enabled))
    .map(([key]) => key);
}

/**
 * Merge class names, dropping falsy values. Lightweight stand-in for
 * clsx/tailwind-merge — sufficient for composing conditional Tailwind classes.
 */
export function cn(...inputs: ClassValue[]): string {
  return inputs.flatMap(toClasses).join(" ");
}
