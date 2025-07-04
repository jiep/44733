import { Location } from '@/model/Location'

export function parseContent(content: string): Location[] | undefined {
  if (!content) return
  const blocks = content.trim().split(/\n\s*\n/)

  const locations = blocks.map((block) => {
    const location: Location = new Location(0, '', '', '', [], '')

    const lines = block.split('\n')
    for (const line of lines) {
      const [key, ...rest] = line.split(':')
      const value = rest.join(':').trim()

      switch (key.trim()) {
        case 'Administración':
          location.id = parseInt(value, 10)
          break
        case 'Dirección':
          location.address = value
          break
        case 'Población':
          location.city = value
          break
        case 'Provincia':
          location.province = value
          break
        case 'Telefono':
          location.phone = value
          break
        case 'Series':
          location.series = value.split(',').map((s) => parseInt(s.trim(), 10))
          break
        default:
          console.warn(`Unknown key: ${key}`)
      }
    }

    return location
  })

  return locations
}
