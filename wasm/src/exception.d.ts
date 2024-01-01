declare function getExceptionMessage(err: unknown): [string, string];
declare function incrementExceptionRefcount(err: unknown): void;
declare function decrementExceptionRefcount(err: unknown): void;
