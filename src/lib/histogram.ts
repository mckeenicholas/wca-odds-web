import erfc from "@stdlib/math-base-special-erfc";

const range = (min: number, max: number) => {
  const result = [];
  for (let i = min; i <= max * 10; i++) {
    result.push(i / 10);
  }
  return result;
};

const cdf = (x: number, mu: number, sigma: number, lambda: number) => {
  return (
    (lambda / 2) *
    Math.exp((lambda / 2) * (2 * mu + lambda * sigma * sigma - 2 * x)) *
    erfc((mu + lambda * sigma * sigma - x) / (Math.sqrt(2) * sigma))
  );
};

const getHistValues = (
  mu: number,
  sigma: number,
  tau: number,
  min: number,
  max: number,
) => {
  const lambda = Math.min(1 / tau, 10);
  return range(min, max).map((x) => {
    return { name: x, probability: cdf(x, mu, sigma, lambda) * 100 };
  });
};

export default getHistValues;
