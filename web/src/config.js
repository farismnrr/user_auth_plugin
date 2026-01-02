// Use runtime config if available, otherwise fallback
const config = window.config || { API_KEY: '', ENDPOINT: '', ALLOWED_ORIGINS: '' }

export const API_URL = config.ENDPOINT || (typeof window !== 'undefined' ? window.location.origin : 'http://localhost:5500')
export const API_KEY = config.API_KEY || ''
export const ALLOWED_ORIGINS_CONFIG = config.ALLOWED_ORIGINS || ''
