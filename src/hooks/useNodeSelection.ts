import { useState, useCallback } from 'react'

export function useNodeSelection(initialId: string | null = null) {
  const [selectedNodeId, setSelectedNodeId] = useState<string | null>(initialId)

  const selectNode = useCallback((nodeId: string) => {
    setSelectedNodeId(nodeId)
  }, [])

  const clearSelection = useCallback(() => {
    setSelectedNodeId(null)
  }, [])

  return {
    selectedNodeId,
    setSelectedNodeId,
    selectNode,
    clearSelection,
  }
}
