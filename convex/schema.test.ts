// Schema definition and index contract tests
// Why: Enforces ADR-004 generic polymorphic table structure and indexes for performant query lookups.
import { describe, it, expect } from "vitest";
import schema from "./schema";

describe("Convex Schema: widget_data", () => {
  it("should define widget_data table with required indexes", () => {
    expect(schema).toBeDefined();
    expect(schema.tables).toBeDefined();
    expect(schema.tables.widget_data).toBeDefined();

    const validator = schema.tables.widget_data.validator;
    expect(validator).toBeDefined();
    expect(validator.fields.instanceId).toBeDefined();
    expect(validator.fields.widgetType).toBeDefined();
    expect(validator.fields.title).toBeDefined();
    expect(validator.fields.payload).toBeDefined();
    expect(validator.fields.updatedAt).toBeDefined();

    const indexes = schema.tables.widget_data.indexes;
    expect(indexes).toBeDefined();

    // Verify index by_instance exists
    const hasInstanceIndex = indexes.some(
      (idx: { indexDescriptor: string; fields: string[] }) =>
        idx.indexDescriptor === "by_instance" &&
        idx.fields.includes("instanceId")
    );
    expect(hasInstanceIndex).toBe(true);

    // Verify index by_type exists
    const hasTypeIndex = indexes.some(
      (idx: { indexDescriptor: string; fields: string[] }) =>
        idx.indexDescriptor === "by_type" &&
        idx.fields.includes("widgetType")
    );
    expect(hasTypeIndex).toBe(true);
  });
});
