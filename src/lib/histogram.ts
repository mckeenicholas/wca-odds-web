import erfc from "@stdlib/math-base-special-erfc";

const hslToHex = (h: number, s: number, l: number) => {
  l /= 100;
  const a = (s * Math.min(l, 1 - l)) / 100;
  const f = (n: number) => {
    const k = (n + h / 30) % 12;
    const color = l - a * Math.max(Math.min(k - 3, 9 - k, 1), -1);
    return Math.round(255 * color)
      .toString(16)
      .padStart(2, "0");
  };
  return `#${f(0)}${f(8)}${f(4)}`;
};

export const generateColors = (num: number) => {
  const hexCodes = [];
  const step = 360 / num;
  for (let i = 0; i < num; i++) {
    const hue = i * step;
    hexCodes.push(hslToHex(hue, 100, 50));
  }
  return hexCodes;
};

const range = (min: number, max: number) => {
  const start = Math.floor(min) * 10;
  const end = Math.floor(max) * 10;
  const result = [];
  for (let i = start; i <= end; i++) {
    result.push(i / 10);
  }
  return result;
};

const pdf = (x: number, mu: number, sigma: number, lambda: number) => {
  return Math.min(
    (lambda / 2) *
      Math.exp((lambda / 2) * (2 * mu + lambda * sigma * sigma - 2 * x)) *
      erfc((mu + lambda * sigma * sigma - x) / (Math.sqrt(2) * sigma)),
    1,
  );
};

const getHistValues = (
  mu: number,
  sigma: number,
  tau: number,
  min: number,
  max: number,
) => {
  const lambda = Math.min(5 / tau, 5);
  return range(min, max).map((x) => {
    return { name: x, probability: pdf(x, mu, sigma, lambda) * 100 };
  });
};

export default getHistValues;
