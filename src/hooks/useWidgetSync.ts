import { useCallback } from "react";
import { useQuery, useMutation } from "convex/react";
import { api } from "../../convex/_generated/api";
import type { Doc } from "../../convex/_generated/dataModel";

export interface UseWidgetSyncOptions {
  instanceId: string;
  widgetType: string;
}

export function useWidgetSync({ instanceId, widgetType }: UseWidgetSyncOptions) {
  // Query document by instanceId from Convex cloud
  const queryResult = useQuery(api.widgets.get, { instanceId });
  const updatePayloadMutation = useMutation(api.widgets.updatePayload);

  // queryResult === undefined indicates active loading state in Convex
  const isLoading = queryResult === undefined;
  const data: Doc<"widget_data"> | null = queryResult ?? null;

  const savePayload = useCallback(
    async (title: string, payload: any) => {
      return await updatePayloadMutation({
        instanceId,
        widgetType,
        title,
        payload,
      });
    },
    [instanceId, widgetType, updatePayloadMutation]
  );

  return {
    data,
    isLoading,
    savePayload,
  };
}
