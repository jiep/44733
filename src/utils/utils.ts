const stylesScale = ["border-red-500 border-l-red-500", "border-orange-500 border-l-orange-500", "border-green-500 border-l-green-500"]

export function getColor(seriesNumber: number): string {
    
    let color = stylesScale[2];

    if (seriesNumber <= 10) {
        color = stylesScale[0];
    } else if (seriesNumber <= 30) {
        color = stylesScale[1];
    }

    return `${color}`;
}