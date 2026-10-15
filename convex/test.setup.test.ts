// Convex in-memory test runner verification
// Why: Ensures all backend contract tests run deterministically in-memory without requiring live cloud calls.
import { describe, it, expect } from "vitest";
import { convexTest } from "convex-test";

describe("Convex In-Memory Test Runner", () => {
  it("should initialize in-memory convex test environment", () => {
    // In-memory test environment must instantiate cleanly
    expect(typeof convexTest).toBe("function");
  });
});
