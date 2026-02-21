import { useState, useCallback } from 'react'
import { GeminiService } from '@services/gemini.service'

export function useAnalyze() {
  const [isAnalyzing, setIsAnalyzing] = useState(false)
  const [aiInsight, setAiInsight] = useState<string | null>(null)
  const [error, setError] = useState<string | null>(null)

  const analyze = useCallback(async (apiKey: string, hubType: string) => {
    setIsAnalyzing(true)
    setError(null)
    setAiInsight(null)

    try {
      const service = new GeminiService(apiKey)
      const result = await service.generateAudit(hubType)
      setAiInsight(result)
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Security audit failed'
      setError(message)
      setAiInsight(message)
    } finally {
      setIsAnalyzing(false)
    }
  }, [])

  return {
    isAnalyzing,
    aiInsight,
    error,
    analyze,
  }
}
