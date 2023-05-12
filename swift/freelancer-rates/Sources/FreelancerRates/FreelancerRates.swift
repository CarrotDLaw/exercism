func dailyRateFrom(hourlyRate: Int) -> Double {
  return Double(hourlyRate * 8)
}

func monthlyRateFrom(hourlyRate: Int, withDiscount discount: Double) -> Double {
  return (Double(hourlyRate * 8) * 22.0 * (1 - (discount / 100.0))).rounded()
}

func workdaysIn(budget: Double, hourlyRate: Int, withDiscount discount: Double) -> Double {
  return (Double(budget) / (dailyRateFrom(hourlyRate: hourlyRate) * (1 - (discount / 100)))).rounded(.down)
}
