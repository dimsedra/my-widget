import { v } from "convex/values";
import { query, mutation } from "./_generated/server";

// Retrieves single widget instance state with strict index-based data isolation (returns null if not found)
export const get = query({
  args: {
    instanceId: v.string(),
  },
  handler: async (ctx, args) => {
    return await ctx.db
      .query("widget_data")
      .withIndex("by_instance", (q) => q.eq("instanceId", args.instanceId))
      .unique();
  },
});

// Queries widgets of a specific type bounded by limit (prevents unbounded memory scan)
export const listByType = query({
  args: {
    widgetType: v.string(),
    limit: v.optional(v.number()),
  },
  handler: async (ctx, args) => {
    const bound = args.limit ?? 50;
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
    const existing = await ctx.db
      .query("widget_data")
      .withIndex("by_instance", (q) => q.eq("instanceId", args.instanceId))
      .unique();

    const now = Date.now();

    if (existing) {
      await ctx.db.patch(existing._id, {
        title: args.title,
        payload: args.payload,
        updatedAt: now,
      });
      return existing._id;
    }

    return await ctx.db.insert("widget_data", {
      instanceId: args.instanceId,
      widgetType: args.widgetType,
      title: args.title,
      payload: args.payload,
      updatedAt: now,
    });
  },
});
