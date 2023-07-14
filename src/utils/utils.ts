const stylesBorderColorScale = [
  'border-red-500 border-l-red-500',
  'border-orange-500 border-l-orange-500',
  'border-green-500 border-l-green-500'
]
const colorScale = ['text-red-500', 'text-orange-500', 'text-green-500']

export function getColor(seriesNumber: number) {
  let borderColor = stylesBorderColorScale[2]
  let textColor = colorScale[2]

  if (seriesNumber <= 10) {
    borderColor = stylesBorderColorScale[0]
    textColor = colorScale[0]
  } else if (seriesNumber <= 30) {
    borderColor = stylesBorderColorScale[1]
    textColor = colorScale[1]
  }

  return {
    border: borderColor,
    color: textColor
  }
}
