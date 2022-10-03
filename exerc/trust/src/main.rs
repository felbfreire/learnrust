#[allow(dead_code)]

// from Rust: enum -> a type that
// can be any one of several variants.
enum Payment {
	Cash,
	CreditCard,
	DebitCard,
	Bitcoin,
	Ether,
}

// structs are types composed 
// by other types.
// mounting a Bitcoin struct:
struct Bitcoin {
	amount: f64,
}

struct Ether {
	amount: f64,
}

// implementing Bitcoin struct.
impl Bitcoin {

	/*
	 The function returns self, a instance of
	 bitcoin. so we can instantiate by:
	 Bitcoin::new(float number);
	*/
	fn new(amount: f64) -> Self {
		Self {
			amount
		}
	}
}

// implementing Ether struct.
impl Ether {
	// Ether::new(float number);
	fn new(amount: f64) -> Self {
		Self {
			amount
		}
	}

}

fn process_payment(a_payment: Payment) {

	// creating coin instances
	let btc_payment: Bitcoin = Bitcoin::new(0.012);
	let etr_payment: Ether = Ether::new(0.215);

	// match reminds me of case, from bash.
	match a_payment {

		Payment::Cash => {
			println!("Paying with Cash");
		}
		Payment::CreditCard => {
			println!("Paying with CreditCard");

		}
		Payment::DebitCard => {
			println!("Paying with DebitCard");

		}
		Payment::Bitcoin => {
			println!("Paying with {} Btc(s)", btc_payment.amount);
		}
		
		Payment::Ether => {
			println!("Paying with {} Ether(s)", etr_payment.amount);
		}
	}
}

fn main() {
	
	// variables for matching
	let a_payment = Payment::Ether;
	let b_payment = Payment::Bitcoin;

	process_payment(a_payment);
	process_payment(b_payment);
}

