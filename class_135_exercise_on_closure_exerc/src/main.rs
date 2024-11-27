struct TaxCalculator {
    calculation: Box<dyn Fn(f64) -> f64>,
}

impl TaxCalculator {
    fn new(calculation: Box<dyn Fn(f64) -> f64>) -> TaxCalculator {
        TaxCalculator { calculation }
    }

    fn calculate(&self, amount: f64) -> f64 {
        (self.calculation)(amount)
    }
}

fn main() {
    println!("\n\n------------------\n\n");

    let vat_calculator = TaxCalculator::new(Box::new(|amount| amount * 0.2));
    let income_tax_calculator = TaxCalculator::new(Box::new(|amount| amount * 0.3));

    println!("VAT for 1000: {}", vat_calculator.calculate(1000.0));
    println!(
        "Income tax for 1000: {}",
        income_tax_calculator.calculate(1000.0)
    );

    println!("\n\n------------------\n\n");
}
