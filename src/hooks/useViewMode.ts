import { useState, useCallback } from 'react'

export type ViewMode = 'map' | 'code'

export function useViewMode(initialMode: ViewMode = 'map') {
  const [viewMode, setViewMode] = useState<ViewMode>(initialMode)

  const toggleViewMode = useCallback(() => {
    setViewMode((prev) => (prev === 'map' ? 'code' : 'map'))
  }, [])

  const setMode = useCallback((mode: ViewMode) => {
    setViewMode(mode)
  }, [])

  return {
    viewMode,
    setViewMode: setMode,
    toggleViewMode,
  }
}
