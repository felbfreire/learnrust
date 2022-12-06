use trust::Payment;

fn main() {
	
	// variables for matching
    let a_payment = Payment::Ether;
	let b_payment = Payment::Bitcoin;

	trust::process_payment(a_payment);
	trust::process_payment(b_payment);
}

