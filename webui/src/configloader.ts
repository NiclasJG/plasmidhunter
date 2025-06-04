let configCache: any = null

export async function loadConfig(): Promise<any> {
    if (configCache) return configCache

    const response = await fetch('/config.json')

    if (!response.ok) {
        throw new Error('Failed to load config: ${response.statusText}')
    }

    configCache = await response.json()
    return configCache
}

export function getConfig() {
    if (!configCache) {
        throw new Error('Config has not been loaded yet. Call loadConfig() first.')
    }

    return configCache;
}