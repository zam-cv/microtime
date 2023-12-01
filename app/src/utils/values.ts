export interface Data {
  id: number;
  group: number;
  average: number;
}

export function getInfo(data: Data[]) {
  let min = data.reduce((a: any, b: any) => (a.average < b.average ? a : b));
  let max = data.reduce((a: any, b: any) => (a.average > b.average ? a : b));
  let average = data.reduce((a: any, b: any) => a + b.average, 0);
  let noise = data.reduce((a: any, b: any) => a + b.average, 0);
  noise = noise / data.length;
  noise = data.reduce((a: any, b: any) => a + (b.average - noise) ** 2, 0);
  noise = noise / data.length;
  noise = Math.sqrt(noise);
  let normality = data.filter((item) => item.average < 36 || item.average > 37);
  return {
    min,
    max,
    average: average / data.length,
    noise,
    normality: normality.length,
  };
}
