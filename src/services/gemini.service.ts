import { GoogleGenAI } from '@google/genai'

export class GeminiService {
  private apiKey: string

  constructor(apiKey: string) {
    if (!apiKey) {
      throw new Error('Gemini API key is required')
    }
    this.apiKey = apiKey
  }

  async generateAudit(hubType: string): Promise<string> {
    const prompt = `Explain the security advantages of using a Cloudflare Worker as a Token Exchange Relay for ${hubType}. Specifically, how it enables sovereign identity across AWS, GCP, and Azure without a traditional VPN.`

    try {
      const ai = new GoogleGenAI({ apiKey: this.apiKey })
      const response = await ai.models.generateContent({
        model: 'gemini-3-flash-preview',
        contents: prompt,
      })

      return response.text || 'No response generated'
    } catch (error) {
      console.error('Gemini service error:', error)
      throw error
    }
  }
}
