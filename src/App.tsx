import { useState } from "react";
import { WidgetShell } from "./components/WidgetShell";

function App() {
  const [instanceId] = useState("todo_default");

  return (
    <div className="w-screen h-screen p-4 flex flex-col justify-start items-stretch bg-transparent">
      <WidgetShell
        instanceId={instanceId}
        widgetType="todo"
        title="To-Do Widget"
        className="w-full h-full"
      >
        <div className="flex flex-col gap-3">
          <p className="text-xs text-slate-400">
            Instance: <span className="font-mono text-slate-300">{instanceId}</span>
          </p>
          <div className="p-3 rounded-lg bg-white/5 border border-white/5 text-sm">
            Drag the top bar to move this widget across your desktop.
          </div>
        </div>
      </WidgetShell>
    </div>
  );
}

export default App;
