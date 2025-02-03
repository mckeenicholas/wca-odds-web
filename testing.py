import matplotlib.pyplot as plt

import numpy as np
import pandas as pd
import math
import random


def fit_skew_normal(data):
    """
    Estimate shape, scale, and location parameters of skew normal distribution
    using method of moments without scipy.

    Args:
        data: numpy array of observations

    Returns:
        tuple of (shape, scale, location) parameters
    """
    # Calculate first three moments
    mean = np.mean(data)
    variance = np.var(data)
    skewness = np.mean(((data - mean) / np.sqrt(variance)) ** 3)

    # Helper constants
    b = 4 / np.pi - 1

    # Bound skewness to prevent numerical issues
    max_skew = (
        0.995 * (4 - np.pi) ** 0.5 * (2 / np.pi) ** 0.5 * (1 - 2 / np.pi) ** (-1.5)
    )
    skewness = np.clip(skewness, -max_skew, max_skew)

    # Estimate delta (intermediate parameter)
    delta = np.sign(skewness) * np.sqrt(
        np.pi
        / 2
        * (
            abs(skewness) ** (2 / 3)
            / (abs(skewness) ** (2 / 3) + ((4 - np.pi) / 2) ** (2 / 3))
        )
    )

    # Ensure delta stays within valid range
    delta = np.clip(delta, -0.9995, 0.9995)

    # Calculate alpha (shape parameter) from delta
    alpha = delta / np.sqrt(1 - delta**2)

    # Constants for location and scale calculations
    mu_z = delta * np.sqrt(2 / np.pi)
    sigma_z = np.sqrt(1 - mu_z**2)

    # Calculate scale (omega) and location (xi)
    omega = np.sqrt(variance / (1 - 2 * delta**2 / np.pi))
    xi = mean - omega * delta * np.sqrt(2 / np.pi)

    return alpha, omega, xi


def skew_normal_pdf(x, alpha, omega, xi):
    """
    Calculate PDF of skew normal distribution at points x

    Args:
        x: points to evaluate PDF at
        alpha: shape parameter
        omega: scale parameter
        xi: location parameter

    Returns:
        PDF values at x
    """
    z = (x - xi) / omega
    pdf = (2 / omega) * norm_pdf(z) * norm_cdf(alpha * z)
    return pdf


def norm_pdf(x):
    """Standard normal PDF"""
    return np.exp(-(x**2) / 2) / np.sqrt(2 * np.pi)


def norm_cdf(x):
    """Standard normal CDF using error function"""
    return 0.5 * (1 + erf(x / np.sqrt(2)))


def erf(x):
    """Approximation of error function"""
    # Constants
    a = 0.140012

    # Horner form polynomial approximation
    sign = np.sign(x)
    x = np.abs(x)

    # This approximation has max error < 2.5e-5
    t = 1.0 / (1.0 + a * x)
    y = 1.0 - (
        ((((0.937298 * t + 0.176091) * t + 0.430638) * t + 1.0) * t) * np.exp(-x * x)
    )

    return sign * y


personId = "2019WANY36"
event = "333"

results_file = pd.read_feather("db/results_dump/results_joined.feather")

results_file["date"] = pd.to_datetime(results_file["date"], errors="coerce")

print(results_file.head())
# Assuming 'results_file' is your DataFrame
filtered_array = results_file.loc[
    (results_file["personId"] == "2019WANY36")
    & (results_file["eventId"] == event)
    & (results_file["date"] > "2024-03-31"),
    ["value1", "value2", "value3", "value4", "value5"],
].to_numpy()

print(filtered_array)
values = np.reshape(filtered_array, -1)

values = values[values > 0]

mean = np.mean(values)
stdev = np.std(values)

values = values[values < mean + stdev * 3]

print(values)


ATTEMPTS = 5
SIMULATIONS = 10000

plt.figure(figsize=(10, 6))
plt.hist(values, bins=50, edgecolor="black", alpha=0.7)
plt.title("Distribution of Values")
plt.xlabel("Value")
plt.ylabel("Frequency")
plt.grid(True)

# Save the plot as an image file
plt.savefig("./hist.png")


def randn_skew_fast(N, alpha=0.0, loc=0.0, scale=1.0):
    sigma = alpha / np.sqrt(1.0 + alpha**2)
    u0 = np.random.randn(N)
    v = np.random.randn(N)
    u1 = (sigma * u0 + np.sqrt(1.0 - sigma**2) * v) * scale
    u1[u0 < 0] *= -1
    u1 = u1 + loc
    return u1


alpha, omega, xi = fit_skew_normal(values)

print(f"Location (xi): {xi}")
print(f"Scale (omega): {omega}")
print(f"Shape (alpha): {alpha}")

sim = randn_skew_fast(10000, alpha, xi, omega)

plt.figure(figsize=(10, 6))
plt.hist(sim, bins=50, edgecolor="black", alpha=0.7)
plt.title("Distribution of Values")
plt.xlabel("Value")
plt.ylabel("Frequency")
plt.grid(True)

# Save the plot as an image file
plt.savefig("./dist.png")
