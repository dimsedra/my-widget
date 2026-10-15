import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { WidgetShell } from "./WidgetShell";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: vi.fn(),
}));

describe("WidgetShell Drag Handle & IPC Contract", () => {
  const mockInnerPosition = vi.fn();
  const mockInnerSize = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
    mockInnerPosition.mockResolvedValue({ x: 250, y: 350 });
    mockInnerSize.mockResolvedValue({ width: 320, height: 450 });
    vi.mocked(getCurrentWindow).mockReturnValue({
      innerPosition: mockInnerPosition,
      innerSize: mockInnerSize,
    } as any);
    vi.mocked(invoke).mockResolvedValue(undefined);
  });

  it("normal path: provides data-tauri-drag-region attribute and renders widget content", () => {
    render(
      <WidgetShell instanceId="todo_1" widgetType="todo" title="My Tasks">
        <div data-testid="widget-content">Task Item 1</div>
      </WidgetShell>
    );

    const dragHandle = screen.getByTestId("widget-drag-handle");
    expect(dragHandle).toHaveAttribute("data-tauri-drag-region");
    expect(screen.getByText("My Tasks")).toBeInTheDocument();
    expect(screen.getByTestId("widget-content")).toBeInTheDocument();
  });

  it("normal path: dispatches cmd_save_window_position on mouseup after drag interaction", async () => {
    const onSaved = vi.fn();
    render(
      <WidgetShell
        instanceId="todo_1"
        widgetType="todo"
        title="My Tasks"
        onPositionSaved={onSaved}
      >
        <div>Content</div>
      </WidgetShell>
    );

    const dragHandle = screen.getByTestId("widget-drag-handle");
    fireEvent.mouseUp(dragHandle);

    await waitFor(() => {
      expect(mockInnerPosition).toHaveBeenCalled();
      expect(mockInnerSize).toHaveBeenCalled();
      expect(invoke).toHaveBeenCalledWith("cmd_save_window_position", {
        instanceId: "todo_1",
        x: 250,
        y: 350,
        width: 320,
        height: 450,
      });
      expect(onSaved).toHaveBeenCalledWith(250, 350);
    });
  });

  it("error path: catches IPC save failure gracefully without crashing the UI", async () => {
    const consoleSpy = vi.spyOn(console, "error").mockImplementation(() => {});
    vi.mocked(invoke).mockRejectedValueOnce(new Error("IPC communication failure"));

    render(
      <WidgetShell instanceId="todo_err" widgetType="todo" title="Failing Widget">
        <div>Content</div>
      </WidgetShell>
    );

    const dragHandle = screen.getByTestId("widget-drag-handle");
    fireEvent.mouseUp(dragHandle);

    await waitFor(() => {
      expect(invoke).toHaveBeenCalledWith("cmd_save_window_position", expect.anything());
      expect(consoleSpy).toHaveBeenCalled();
    });

    // Content remains mounted and interactive
    expect(screen.getByText("Failing Widget")).toBeInTheDocument();
    consoleSpy.mockRestore();
  });

  it("boundary invariant: ignores mouseup if instanceId is empty", async () => {
    render(
      <WidgetShell instanceId="" widgetType="todo" title="No ID Widget">
        <div>Content</div>
      </WidgetShell>
    );

    const dragHandle = screen.getByTestId("widget-drag-handle");
    fireEvent.mouseUp(dragHandle);

    expect(invoke).not.toHaveBeenCalled();
  });
});
