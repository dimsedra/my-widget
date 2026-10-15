import React from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

export interface WidgetShellProps {
  instanceId: string;
  widgetType: string;
  title: string;
  children: React.ReactNode;
  onPositionSaved?: (x: number, y: number) => void;
  className?: string;
}

export const WidgetShell: React.FC<WidgetShellProps> = ({
  instanceId,
  widgetType: _widgetType,
  title,
  children,
  onPositionSaved,
  className = "",
}) => {
  const handleMouseUp = async () => {
    if (!instanceId) return;

    try {
      const win = getCurrentWindow();
      const pos = await win.innerPosition();
      const size = await win.innerSize();

      await invoke("cmd_save_window_position", {
        instanceId,
        x: pos.x,
        y: pos.y,
        width: size.width,
        height: size.height,
      });

      onPositionSaved?.(pos.x, pos.y);
    } catch (err) {
      console.error(`Failed to save window position for instance ${instanceId}:`, err);
    }
  };

  return (
    <div
      className={`relative flex flex-col rounded-2xl overflow-hidden shadow-2xl backdrop-blur-xl bg-slate-900/60 border border-white/10 text-slate-100 select-none ${className}`}
      data-testid="widget-shell-container"
    >
      <div
        data-tauri-drag-region
        data-testid="widget-drag-handle"
        onMouseUp={handleMouseUp}
        className="flex items-center justify-between px-4 py-3 bg-white/5 border-b border-white/10 cursor-grab active:cursor-grabbing"
      >
        <span className="text-sm font-semibold tracking-wide text-slate-200">
          {title}
        </span>
      </div>

      <div className="flex-1 p-4 overflow-auto">
        {children}
      </div>
    </div>
  );
};
