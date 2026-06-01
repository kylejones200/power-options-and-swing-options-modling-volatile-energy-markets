"""European call intrinsic values."""

from __future__ import annotations

import numpy as np


def call_intrinsic_values(spots: np.ndarray, strike: float) -> np.ndarray:
    s = np.asarray(spots, dtype=float)
    return np.maximum(s - strike, 0.0)
