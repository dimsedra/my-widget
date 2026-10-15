// Tests for useWidgetSync hook
// Why: Enforces REQ-SYNC-001 and NFR-REL-001 for reactive state and offline fallback handling.
import { describe, it, expect, vi } from "vitest";
import { renderHook, act } from "@testing-library/react";
import * as convexReact from "convex/react";
import { useWidgetSync } from "./useWidgetSync";

vi.mock("convex/react", () => ({
  useQuery: vi.fn(),
  useMutation: vi.fn(),
}));

describe("useWidgetSync", () => {
  it("should handle loading state when data is being fetched", () => {
    vi.mocked(convexReact.useQuery).mockReturnValue(undefined);
    vi.mocked(convexReact.useMutation).mockReturnValue(vi.fn() as any);

    const { result } = renderHook(() =>
      useWidgetSync({ instanceId: "todo_1", widgetType: "todo" })
    );

    expect(result.current.isLoading).toBe(true);
    expect(result.current.data).toBeNull();
  });

  it("should return synced data when Convex query resolves", () => {
    const mockDoc = {
      _id: "doc_123" as any,
      _creationTime: 123456,
      instanceId: "todo_1",
      widgetType: "todo",
      title: "My Tasks",
      payload: { items: [{ id: "1", text: "Task 1", isCompleted: false }] },
      updatedAt: 123456,
    };
    vi.mocked(convexReact.useQuery).mockReturnValue(mockDoc);
    vi.mocked(convexReact.useMutation).mockReturnValue(vi.fn() as any);

    const { result } = renderHook(() =>
      useWidgetSync({ instanceId: "todo_1", widgetType: "todo" })
    );

    expect(result.current.isLoading).toBe(false);
    expect(result.current.data).toEqual(mockDoc);
  });

  it("should call updatePayload mutation with proper arguments", async () => {
    const mockMutation = vi.fn().mockResolvedValue("doc_123");
    vi.mocked(convexReact.useQuery).mockReturnValue(null);
    vi.mocked(convexReact.useMutation).mockReturnValue(mockMutation as any);

    const { result } = renderHook(() =>
      useWidgetSync({ instanceId: "todo_1", widgetType: "todo" })
    );

    await act(async () => {
      await result.current.savePayload("New Title", { items: [] });
    });

    expect(mockMutation).toHaveBeenCalledWith({
      instanceId: "todo_1",
      widgetType: "todo",
      title: "New Title",
      payload: { items: [] },
    });
  });
});
