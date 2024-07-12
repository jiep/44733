import { Location } from '@/model/Location'

export function parseContent(content: string): Location[] {
  const lines = content.split('\n')
  const locations: Location[] = []
  let location: Location = new Location('', '', '', '', [], '')

  for (let line of lines) {
    line = line.trim()
    if (line.startsWith('Nombre del receptor:')) {
      if (
        location.name ||
        location.address ||
        location.city ||
        location.province ||
        location.phone ||
        location.series.length
      ) {
        locations.push(location)
        location = new Location('', '', '', '', [], '')
      }
      location.name = line.replace('Nombre del receptor:', '').trim()
    } else if (line.startsWith('Dirección:')) {
      location.address = line.replace('Dirección:', '').trim()
    } else if (line.startsWith('Población:')) {
      location.city = line.replace('Población:', '').trim()
    } else if (line.startsWith('Provincia:')) {
      location.province = line.replace('Provincia:', '').trim()
    } else if (line.startsWith('Telefono:')) {
      location.phone = line.replace('Telefono:', '').trim()
    } else if (line.startsWith('Series:')) {
      location.series = line.replace('Series:', '').trim().split(',').map(Number)
    }
  }

  if (
    location.name ||
    location.address ||
    location.city ||
    location.province ||
    location.phone ||
    location.series.length ||
    location.phone
  ) {
    locations.push(location)
  }

  return locations
}
