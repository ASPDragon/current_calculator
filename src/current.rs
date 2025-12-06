/// Calculates the phase current (I) of a three-phase AC motor.
///
/// The formula used:
///
/// ```text
///        P(kW) * 1000
/// I = ─────────────────────
///     √3 × U × cosφ × η
/// ```
///
/// # Arguments
///
/// * `power_kw` – Motor power in kilowatts (kW)
/// * `voltage` – Line voltage in volts (V), e.g., 380 or 400
/// * `power_factor` – Cosine of the phase angle (cosφ), typical range: 0.7–0.9
/// * `efficiency` – Motor efficiency η (0.8–0.95)
///
/// # Returns
///
/// Phase current in amperes (A)
///
/// # Example
///
/// ```
/// use yourcrate::current::current;
///
/// // Example: 5.5 kW motor, 380 V, cosφ = 0.82, η = 0.88
/// let i = current(5.5, 380.0, 0.82, 0.88);
///
/// // Expected ≈ 11.58 A
/// assert!((i - 11.58).abs() < 0.1);
/// ```
///
/// # Note
///
/// If your power is already in watts, remove the `× 1000`.
pub fn current(power_kw: f32, voltage: f32, power_factor: f32, efficiency: f32) -> f32 {
    (power_kw * 1000.0) / (f32::sqrt(3.0) * voltage * power_factor * efficiency)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_calculation() {
        // Example from theory:
        // 5.5 kW / (sqrt(3) * 380V * 0.82 * 0.88) ≈ 12.2 A
        let i = current(5.5, 380.0, 0.82, 0.88);

        assert!((i - 11.58).abs() < 0.1, "Expected ~11.58 A, got {}", i);
    }
}