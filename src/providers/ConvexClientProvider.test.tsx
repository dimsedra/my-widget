// Tests for ConvexClientProvider
// Why: Enforces REQ-SYNC-001 providing reactive Convex client context to the React tree.
import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { useConvex } from "convex/react";
import { ConvexClientProvider } from "./ConvexClientProvider";

function TestConsumer() {
  const convex = useConvex();
  return (
    <div>
      <span>Consumer Rendered</span>
      <span>Has Client: {convex ? "yes" : "no"}</span>
    </div>
  );
}

describe("ConvexClientProvider", () => {
  it("should provide Convex context to nested child components", () => {
    render(
      <ConvexClientProvider url="https://mock-test.convex.cloud">
        <TestConsumer />
      </ConvexClientProvider>
    );

    expect(screen.getByText("Consumer Rendered")).toBeInTheDocument();
    expect(screen.getByText("Has Client: yes")).toBeInTheDocument();
  });
});
