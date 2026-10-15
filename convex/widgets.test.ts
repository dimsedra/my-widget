/// <reference types="vite/client" />
// Tests for widget queries and mutations
// Why: Enforces REQ-SYNC-001 generic polymorphic payload upsert and reactive retrieval.
import { describe, it, expect } from "vitest";
import { convexTest } from "convex-test";
import schema from "./schema";
import { get, updatePayload } from "./widgets";

const modules = import.meta.glob("./**/*.*s");

describe("Convex Widgets: Initial Upsert (Normal Path)", () => {
  it("should create a new widget record when instanceId does not exist", async () => {
    const t = convexTest(schema, modules);

    await t.mutation(updatePayload, {
      instanceId: "todo-1",
      widgetType: "todo",
      title: "My To-Do List",
      payload: { items: [{ id: "item-1", text: "Buy milk", isCompleted: false }] },
    });

    const result = await t.query(get, { instanceId: "todo-1" });
    expect(result).not.toBeNull();
    expect(result?.instanceId).toBe("todo-1");
    expect(result?.widgetType).toBe("todo");
    expect(result?.title).toBe("My To-Do List");
    expect(result?.payload).toEqual({
      items: [{ id: "item-1", text: "Buy milk", isCompleted: false }],
    });
    expect(result?.updatedAt).toBeGreaterThan(0);
  });
});
