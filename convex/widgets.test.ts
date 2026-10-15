/// <reference types="vite/client" />
// Tests for widget queries and mutations
// Why: Enforces REQ-SYNC-001 generic polymorphic payload upsert and reactive retrieval.
import { describe, it, expect } from "vitest";
import { convexTest } from "convex-test";
import schema from "./schema";
import { api } from "./_generated/api";

const modules = import.meta.glob("./**/*.*s");

describe("Convex Widgets: Initial Upsert (Normal Path)", () => {
  it("should create a new widget record when instanceId does not exist", async () => {
    const t = convexTest(schema, modules);

    await t.mutation(api.widgets.updatePayload, {
      instanceId: "todo-1",
      widgetType: "todo",
      title: "My To-Do List",
      payload: { items: [{ id: "item-1", text: "Buy milk", isCompleted: false }] },
    });

    const result = await t.query(api.widgets.get, { instanceId: "todo-1" });
    expect(result).not.toBeNull();
    expect(result?.instanceId).toBe("todo-1");
    expect(result?.widgetType).toBe("todo");
    expect(result?.title).toBe("My To-Do List");
    expect(result?.payload).toEqual({
      items: [{ id: "item-1", text: "Buy milk", isCompleted: false }],
    });
    expect(result?.updatedAt).toBeGreaterThan(0);
  });

  it("should update existing widget record without duplicating rows", async () => {
    const t = convexTest(schema, modules);

    // Initial insert
    await t.mutation(api.widgets.updatePayload, {
      instanceId: "todo-1",
      widgetType: "todo",
      title: "Initial Title",
      payload: { items: [] },
    });

    const initial = await t.query(api.widgets.get, { instanceId: "todo-1" });
    expect(initial).not.toBeNull();

    // Update with new payload
    await t.mutation(api.widgets.updatePayload, {
      instanceId: "todo-1",
      widgetType: "todo",
      title: "Updated Title",
      payload: { items: [{ id: "1", text: "New Task", isCompleted: true }] },
    });

    // Verify record was patched in-place
    const updated = await t.query(api.widgets.get, { instanceId: "todo-1" });
    expect(updated).not.toBeNull();
    expect(updated?._id).toEqual(initial?._id);
    expect(updated?.title).toBe("Updated Title");
    expect(updated?.payload).toEqual({
      items: [{ id: "1", text: "New Task", isCompleted: true }],
    });

    // In-memory query to ensure no duplicate rows exist for this instanceId
    const allMatching = await t.run(async (ctx) => {
      return await ctx.db
        .query("widget_data")
        .withIndex("by_instance", (q) => q.eq("instanceId", "todo-1"))
        .collect();
    });
    expect(allMatching).toHaveLength(1);
  });

  it("should maintain strict data isolation between distinct widget instances", async () => {
    const t = convexTest(schema, modules);

    // Seed instance A
    await t.mutation(api.widgets.updatePayload, {
      instanceId: "todo-alpha",
      widgetType: "todo",
      title: "Alpha Widget",
      payload: { items: [{ id: "a1", text: "Alpha task", isCompleted: false }] },
    });

    // Seed instance B
    await t.mutation(api.widgets.updatePayload, {
      instanceId: "countdown-beta",
      widgetType: "countdown",
      title: "Beta Countdown",
      payload: { targetDate: 1893456000000, unit: "days" },
    });

    // Query instance B - must return Beta data, NOT Alpha data
    const betaData = await t.query(api.widgets.get, { instanceId: "countdown-beta" });
    expect(betaData).not.toBeNull();
    expect(betaData?.instanceId).toBe("countdown-beta");
    expect(betaData?.widgetType).toBe("countdown");
    expect(betaData?.title).toBe("Beta Countdown");

    // Query instance A - must return Alpha data
    const alphaData = await t.query(api.widgets.get, { instanceId: "todo-alpha" });
    expect(alphaData).not.toBeNull();
    expect(alphaData?.instanceId).toBe("todo-alpha");
    expect(alphaData?.widgetType).toBe("todo");
    expect(alphaData?.title).toBe("Alpha Widget");
  });

  it("should return null cleanly when querying a non-existent instanceId (graceful null)", async () => {
    const t = convexTest(schema, modules);

    // Query non-existent instance - must return null without throwing
    const result = await t.query(api.widgets.get, { instanceId: "non-existent-instance-xyz" });
    expect(result).toBeNull();
  });

  it("should return bounded list of widgets by type using take() limit", async () => {
    const t = convexTest(schema, modules);

    // Seed 5 widgets of type "todo"
    for (let i = 1; i <= 5; i++) {
      await t.mutation(api.widgets.updatePayload, {
        instanceId: `todo-batch-${i}`,
        widgetType: "todo",
        title: `Todo ${i}`,
        payload: { items: [] },
      });
    }

    // Seed 1 widget of type "countdown"
    await t.mutation(api.widgets.updatePayload, {
      instanceId: "countdown-other",
      widgetType: "countdown",
      title: "Countdown Other",
      payload: { targetDate: 12345 },
    });

    // Query listByType with limit = 3
    const results = await t.query(api.widgets.listByType, {
      widgetType: "todo",
      limit: 3,
    });

    expect(results).toHaveLength(3);
    for (const item of results) {
      expect(item.widgetType).toBe("todo");
    }
  });
});
