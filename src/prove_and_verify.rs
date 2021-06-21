use std::env;

use crate::ropsten_query;
use crate::cairo_sharp;

// Dependancies 
	// - Python 3.7.1, pip3 (Write installation in libra/scripts/dev_setup.sh)
	// - Cairo Lang Files (pip3 install cairo-lang-0.1.0.zip)
	

// Goal of Cairo Program is to prove and verifiy that the input's value is above x
//TODO: Convert bool -> Result<bool, io::Error>
pub async fn verify(name_input: String, val_input: u128) -> bool {
	/* --------------
		Recording current driectory and switching to project_root/assests
	---------------- */
	let dir = env::current_dir().unwrap();
	let mut path = project_root::get_project_root().expect("Could not get project root"); //TODO: Return an error instead
	//path.push("ol");
	//path.push("zero_knowledge");
	path.push("assets");

	assert!(env::set_current_dir(path).is_ok());

	// DEBUGGING
	//let path_str = path.into_os_string().into_string().unwrap();
	//println!("{}", dir.display());


	/* -------------
		Sending Cairo Program to SHARP Prover
	--------------- */
	let output = cairo_sharp::send_program(&name_input, val_input);

	//Fail if sneding to SHARP Fails
	if !output.status.success() {
		println!("cairo unsuccessfully ran");
		/* DEBUGGING
		// let mut err_message = String::from("");
		// for out_int in output.stderr {
		// 	err_message.push(out_int as char);
		// }
		// println!("OUTPUT: {}", err_message);
		--------------- */
		return false;
	}

	
	/* --------------
	Parse out job key and fact (represented as Strings) 
	Contents of (Succesful) Output:
		Job key: Some_Hex_Val-Some_Hex_Val-Some_Hex_Val-Some_Hex_Val-Some_Hex_Val
		Fact: 0xSome_BIG_Hex_Val
	------------------ */
	let (fact, job_key) = cairo_sharp::parse_output(&output).expect("Couldn't Properly parse Fact");


	// DEBUGGING
	println!("The parsed Job Key: {}", job_key);
	println!("The Parsed Fact: {}", fact);




	/* --------------
	OPTION 1 for waiting:
	Poll "cairo-sharp status" until it is ready
	Currently this is pretty bad since the Move Command will have to wait until SHARP is done verifying
	Alternatives:
		Use multi-threading (is it possible to achieve true asynchrousity on the blockchain?)
	------------------ */
	// let query_limit = 100;
	// let mut status_success = false;
	// for _ in 0..query_limit {
	// 	//Create the query
	// 	let output = Command::new("cairo-sharp") //cairo-sharp status job_key
    //     	.args(&["status", &job_key[..]])
	// 		.output()
    //     	.expect("Failed to execute: cairo-sharp status job_key"); //TODO: Return an error instead
	// 	if !output.status.success() { continue; }

	// 	//Break out of loop when query is "PROCESSED"
	// 	let mut status = String::from("");
	// 	for code_ascii in output.stdout {
	// 		status.push(code_ascii as char);
	// 	}
		
	// 	// DEGUBBING
	// 	//println!("{}", status);

	// 	if status == String::from("PROCESSED\n") {
	// 		status_success = true;
	// 		break;
	// 	}
	// }

	// //If status timed out, return false
	// if !status_success {
	// 	return false;
	// }




	/* ------------------
	Query the ETH Network (or use a yet-to be SHARP API) somehow to obtain a STARK Proof (Contain multiple programs)
	
	Option:
		Read each block registered on the Ropsten Chain (takes ~30 sec)
		Read each transaction in the pool and ensure that it's from SHARP's account # and to the Verification contract account #
		Decode the input of the transaction
	--------------------- */
	let encoded_proof = ropsten_query::get_encoded_proof().await.expect("Couldn't query Proof");
	let params = ropsten_query::decode_proof(&encoded_proof);

	// DEBUGGING
	println!("{:?}", params);


	//Revert back to original env's directory
	assert!(env::set_current_dir(dir).is_ok());


	//Pass proof to verifier

	//Get back facts that verified succesfully?

	//If our fact is one of the ones verified succesfully, return true

	return true;
}