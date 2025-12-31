use crate::models::Booking;

/// Strategy Pattern - Different pricing strategies
pub trait PricingStrategy {
    fn calculate_price(&self, base_rate: f64, duration_hours: f64, booking: &Booking) -> f64;
}

/// Standard pricing - Simple hourly rate
pub struct StandardPricing;

impl PricingStrategy for StandardPricing {
    fn calculate_price(&self, base_rate: f64, duration_hours: f64, _booking: &Booking) -> f64 {
        base_rate * duration_hours
    }
}

/// Peak hours pricing - Higher rates during peak times
pub struct PeakHoursPricing;

impl PricingStrategy for PeakHoursPricing {
    fn calculate_price(&self, base_rate: f64, duration_hours: f64, booking: &Booking) -> f64 {
        let hour = booking.start_time.hour();
        let multiplier = if (9..=11).contains(&hour) || (14..=16).contains(&hour) {
            1.5 // Peak hours
        } else {
            1.0
        };
        
        base_rate * duration_hours * multiplier
    }
}

/// Discount pricing - Volume discounts
pub struct DiscountPricing;

impl PricingStrategy for DiscountPricing {
    fn calculate_price(&self, base_rate: f64, duration_hours: f64, _booking: &Booking) -> f64 {
        let total = base_rate * duration_hours;
        
        if duration_hours >= 8.0 {
            total * 0.80 // 20% discount for full day
        } else if duration_hours >= 4.0 {
            total * 0.90 // 10% discount for half day
        } else {
            total
        }
    }
}

/// Weekend pricing - Different rates for weekends
pub struct WeekendPricing;

impl PricingStrategy for WeekendPricing {
    fn calculate_price(&self, base_rate: f64, duration_hours: f64, booking: &Booking) -> f64 {
        let weekday = booking.start_time.weekday();
        let multiplier = if weekday.num_days_from_monday() >= 5 {
            1.25 // Weekend premium
        } else {
            1.0
        };
        
        base_rate * duration_hours * multiplier
    }
}

pub struct PricingCalculator {
    strategy: Box<dyn PricingStrategy>,
}

impl PricingCalculator {
    pub fn new(strategy: Box<dyn PricingStrategy>) -> Self {
        Self { strategy }
    }

    pub fn set_strategy(&mut self, strategy: Box<dyn PricingStrategy>) {
        self.strategy = strategy;
    }

    pub fn calculate(&self, base_rate: f64, booking: &Booking) -> f64 {
        let duration = booking.duration_hours();
        self.strategy.calculate_price(base_rate, duration, booking)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Local, Duration};
    use uuid::Uuid;

    #[test]
    fn test_standard_pricing() {
        let calculator = PricingCalculator::new(Box::new(StandardPricing));
        let booking = Booking::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Local::now(),
            Local::now() + Duration::hours(3),
            "Test".to_string(),
            5,
        );
        
        let price = calculator.calculate(50.0, &booking);
        assert_eq!(price, 150.0);
    }

    #[test]
    fn test_discount_pricing() {
        let calculator = PricingCalculator::new(Box::new(DiscountPricing));
        let booking = Booking::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Local::now(),
            Local::now() + Duration::hours(8),
            "Test".to_string(),
            5,
        );
        
        let price = calculator.calculate(50.0, &booking);
        assert_eq!(price, 320.0); // 400 * 0.8
    }
}
