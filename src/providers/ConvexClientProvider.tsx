import { ReactNode, useState, useEffect } from "react";
import { ConvexProvider, ConvexReactClient } from "convex/react";

interface ConvexClientProviderProps {
  children: ReactNode;
  url?: string;
}

export function ConvexClientProvider({ children, url }: ConvexClientProviderProps) {
  const convexUrl =
    url || import.meta.env.VITE_CONVEX_URL || "https://placeholder.convex.cloud";

  const [client] = useState(() => new ConvexReactClient(convexUrl));

  useEffect(() => {
    return () => {
      void client.close();
    };
  }, [client]);

  return <ConvexProvider client={client}>{children}</ConvexProvider>;
}
