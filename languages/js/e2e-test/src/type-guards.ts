export function isNotDefined<T>(value: T | undefined): value is undefined {
  return value === undefined;
}

export function isNull<T>(value: T | null): value is null {
  return value === null;
}

export function forceDefined<T>(value: T | null | undefined): T {
  if (isNotDefined(value) || isNull(value)) {
    throw new Error("Value is not defined");
  }
  return value;
}
