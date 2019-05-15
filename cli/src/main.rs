use std::io;
use serde_json::json;
use structopt::StructOpt;
use linefeed::{Interface, ReadResult};

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Cli {
	/// Url to connect to the running conductor HTTP port (e.g. http://localhost:3000)
	url: reqwest::Url,
	/// This is the instance ID in the conductor that is running the game on the given port (e.g gameInstance)
	instance: String,
}

fn main() -> io::Result<()> {
    let cli = Cli::from_args();
    println!("{:?}", cli);

    // create the functions required for playing the game
    let valid_moves = holochain_call_generator(cli.url.clone(), cli.instance.clone(), "main".into(), "valid_moves".into());
    let make_move = holochain_call_generator(cli.url.clone(), cli.instance.clone(), "main".into(), "make_move".into());
    let create_game = holochain_call_generator(cli.url.clone(), cli.instance.clone(), "main".into(), "create_game".into());
    // let render_game = holochain_call_generator(cli.url.clone(), cli.instance.clone(), "main".into(), "render_state".into());

    let interface = Interface::new("Holochain generic game")?;

    println!("CLI interface for games written using the Holochain Generic Game framework.");
    println!("Enter \"help\" for a list of commands.");
    println!("Press Ctrl-D or enter \"quit\" to exit.");
    println!("");

	interface.set_prompt("No game> ")?;

	let mut current_game: Option<String> = None;

 	while let ReadResult::Input(line) = interface.read_line()? {

        let (cmd, args) = split_first_word(&line);

        match cmd {
            "help" => {
                println!("This is where it will print help commands")
            }
            "set_game" => {
            	if is_hash(args) {
            		println!("Setting current game hash to {}", args);
            		current_game = Some(args.into());
            	} else {
            		println!("argument does not appear to be a valid address")
            	}
            }
            "new_game" => {
            	if is_hash(args) {
            		let result = create_game(json!({
            			"opponent": args,
            			"timestamp": 0,
            		}));
            		match result {
            			Ok(result) => {
            				current_game = result["Ok"].as_str().map(|s| s.to_string());
            			},
            			Err(e) => {
            				println!("{:?}", e);
            			}
            		}
            	} else {
            		println!("argument must be valid agent address of an opponent.")
            	}
            }
            "moves" => {
            	match valid_moves(json!({})) {
            		Ok(result) => {
		            	println!("The valid moves are:");

		            	result["Ok"].as_array().unwrap()
		            	.iter()
		            	.for_each(|elem| {
		            		println!("{}", elem);
		            	});
            		},
            		Err(e) => {
            			println!("Unable to make call to holochain conductor. Make sure the it is running and the URL and instanceId are correct.");
            			println!("{:?}", e);
            		}
            	};
            },
            "make_move" => {
            	if let Some(current_game) = current_game.clone() {
	            	println!("making move: {:?}", args);
	            	let result = make_move(json!({
		            	"game_move": {
		            		"game": current_game,
		            		"move_type": args,
		            		"timestamp": 0,
		            	}
	            	}));
	            	match result {
	            		Ok(result) => {
			            	println!("Move made successfully");
			            	println!("{:?}", result);
	            		},
	            		Err(e) => {
	            			println!("Unable to make move on this game.");
	            			println!("{:?}", e);
	            		}
	            	};
            	} else {
            		println!("No game set to make moves on. use the \"set_game\" command.");
            	}
            },
            "exit" => {
            	println!("You can resume any game at a later date. Bye!");
            	break
            }
            _ => {
            	println!("Invalid command!")
            }
		}

		if let Some(current_game) = current_game.clone() {
 			interface.set_prompt(&format!("{}> ", current_game))?;
 		}
	}


    // let res = make_move(json!({}));
    // println!("{:?}", res)
    Ok(())
}



/**
 * @brief      Returns functions to make calls to a particular zome function on a url
 *
 * @return     { description_of_the_return_value }
 */
fn holochain_call_generator(
	url: reqwest::Url, 
	instance: String,
	zome: String,
	func: String,
) -> Box<Fn(serde_json::Value) -> reqwest::Result<serde_json::Value>> {


	let client = reqwest::Client::new();

	let make_rpc_call = move |params| {
		json!({
			"jsonrpc": "2.0",
			"id": 0,
			"method": "call",
			"params": {
				"instance_id": instance,
				"zome": zome,
				"function": func,
				"params": params
			}
		})
	};

	Box::new(move |params| {

		client.post(url.clone())
		    .json(&make_rpc_call(params))
		    .send()?
		    .json()
		    .map(|r: serde_json::Value| {
		    	println!("{:?}", r); 
		    	r["result"].clone()
		    })
		    .map(|s| serde_json::from_str(s.as_str().unwrap()).unwrap())
	})

}

fn split_first_word(s: &str) -> (&str, &str) {
    let s = s.trim();

    match s.find(|ch: char| ch.is_whitespace()) {
        Some(pos) => (&s[..pos], s[pos..].trim_start()),
        None => (s, "")
    }
}

fn is_hash(s: &str) -> bool {
	s.starts_with("Qm") && s.len() == 46
}