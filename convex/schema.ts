import { defineSchema, defineTable } from "convex/server";
import { v } from "convex/values";

export default defineSchema({
  widget_data: defineTable({
    instanceId: v.string(),
    widgetType: v.string(),
    title: v.string(),
    payload: v.any(),
    updatedAt: v.number(),
  })
    .index("by_instance", ["instanceId"])
    .index("by_type", ["widgetType"]),
});
