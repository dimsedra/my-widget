import { ReactNode, useMemo } from "react";
import { ConvexProvider, ConvexReactClient } from "convex/react";

interface ConvexClientProviderProps {
  children: ReactNode;
  url?: string;
}

export function ConvexClientProvider({ children, url }: ConvexClientProviderProps) {
  const convexUrl =
    url || import.meta.env.VITE_CONVEX_URL || "https://placeholder.convex.cloud";

  const client = useMemo(() => new ConvexReactClient(convexUrl), [convexUrl]);

  return <ConvexProvider client={client}>{children}</ConvexProvider>;
}
