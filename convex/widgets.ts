import { v } from "convex/values";
import { query, mutation } from "./_generated/server";

// Retrieves single widget instance state with strict index-based data isolation (returns null if not found)
export const get = query({
  args: {
    instanceId: v.string(),
  },
  handler: async (ctx, args) => {
    const trimmedId = args.instanceId.trim();
    if (!trimmedId) {
      return null;
    }

    return await ctx.db
      .query("widget_data")
      .withIndex("by_instance", (q) => q.eq("instanceId", trimmedId))
      .unique();
  },
});

// Queries widgets of a specific type bounded by clamped limit (prevents unbounded memory scan)
export const listByType = query({
  args: {
    widgetType: v.string(),
    limit: v.optional(v.number()),
  },
  handler: async (ctx, args) => {
    const rawLimit = args.limit ?? 50;
    // Defensive clamping between 1 and 100
    const bound = Math.min(Math.max(1, Math.floor(rawLimit)), 100);

    return await ctx.db
      .query("widget_data")
      .withIndex("by_type", (q) => q.eq("widgetType", args.widgetType))
      .take(bound);
  },
});

// Upserts widget data: patches existing row in-place or inserts new record
export const updatePayload = mutation({
  args: {
    instanceId: v.string(),
    widgetType: v.string(),
    title: v.string(),
    payload: v.any(),
  },
  handler: async (ctx, args) => {
    const trimmedId = args.instanceId.trim();
    if (!trimmedId) {
      throw new Error("instanceId must not be empty");
    }

    const existing = await ctx.db
      .query("widget_data")
      .withIndex("by_instance", (q) => q.eq("instanceId", trimmedId))
      .unique();

    const now = Date.now();

    if (existing) {
      await ctx.db.patch(existing._id, {
        widgetType: args.widgetType,
        title: args.title,
        payload: args.payload,
        updatedAt: now,
      });
      return existing._id;
    }

    return await ctx.db.insert("widget_data", {
      instanceId: trimmedId,
      widgetType: args.widgetType,
      title: args.title,
      payload: args.payload,
      updatedAt: now,
    });
  },
});
