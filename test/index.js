const path = require('path')
const tape = require('tape')

const { Diorama, tapeExecutor, backwardCompatibilityMiddleware } = require('@holochain/diorama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/dots-and-boxes.dna.json")
const dna = Diorama.dna(dnaPath, 'dots-and-boxes')

const diorama = new Diorama({
  instances: {
    alice: dna,
    bob: dna,
  },
  bridges: [],
  debugLog: false,
  executor: tapeExecutor(require('tape')),
  middleware: backwardCompatibilityMiddleware,
})


// <<DEVCAMP>> Your tests here


//Scenarios keep a series of test steps separate from the overall test - clean slate inbetween
diorama.registerScenario("Can create a new game and make moves", async(s, t, { alice, bob}) => {

//Moves start at timestamp=10
move_timestamp = 10

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
} 

async function moveHelper (game, player, x, y, direction, valid) {
  console.log(`========Player ${player.id} moves (x=${x},y=${y},${direction}) and expects ${valid}.`);

  move_result = await player.callSync('main', 'make_move', {
    new_move: {
      game: game,
      move_type: { Place: {x:x, y:y, direction:direction} },
      timestamp: move_timestamp,
    }
  })
  console.log(move_result)
  if (valid) {
    // make sure the move was made without error
    t.equal(move_result.Err, undefined)
  } else {
    //check for match on error
    t.equal(move_result.Ok, undefined)
  }
  
/*   game_state = await alice.callSync('main', 'get_state',{
    game_address: game
  })
  console.log(JSON.stringify(game_state)) */
  render_state = await alice.callSync('main', 'render_state',{
    game_address: game
  })  
  console.log(render_state.Ok)

  move_timestamp++

  await sleep(6000); //Sleep 3 seconds to see if that gets around the Hachiko timeout
  
}

  //If you *.call instead of *.callSync, then you will not be waiting for the sychronization to complete
  const create_game_result = await alice.callSync('main', 'create_game', {
    opponent: bob.agentId,
    timestamp: 0,
  })
  console.log('Player 1 is Alice; Player 2 is Bob')

  //Might want to stringify this b/c objects embedded in the result will just show {object}
  console.log(create_game_result)
  // Check the ok respose is an address
  t.equal(create_game_result.Ok.length, 46)

  game_state = await alice.callSync('main', 'get_state',{
    game_address: create_game_result.Ok
  })
  console.log(JSON.stringify(game_state))
  render_state = await alice.callSync('main', 'render_state',{
    game_address: create_game_result.Ok
  })  
  console.log(render_state.Ok)

  // alice try to go out of turn
  await moveHelper(create_game_result.Ok, alice, 2, 1, "Up", false)

  // bob try to make a series of out of range moves
  await moveHelper(create_game_result.Ok, bob, 0, 2, "Up", false)
  await moveHelper(create_game_result.Ok, bob, 2, 0, "Right", false)
  await moveHelper(create_game_result.Ok, bob, 0, -1, "Up", false)  
  await moveHelper(create_game_result.Ok, bob, -1, 0, "Right", false)  

  // bob must make the first valid move
  await moveHelper(create_game_result.Ok, bob, 0, 0, "Up", true)
  
  // alice fail to make a move in the same location
  await moveHelper(create_game_result.Ok, alice, 0, 0, "Up", false)

  // alice make a valid move
  await moveHelper(create_game_result.Ok, alice, 2, 1, "Up", true)
  
  // bob make a 2nd valid move
  await moveHelper(create_game_result.Ok, bob, 0, 0, "Right", true)

   // alice make a 2nd valid move
   await moveHelper(create_game_result.Ok, alice, 1, 0, "Up", true)

  // bob complete a box
  await moveHelper(create_game_result.Ok, bob, 0, 1, "Right", true)

  // alice try to move when it is not her turn b/c of box completion
  await moveHelper(create_game_result.Ok, alice, 1, 1, "Right", false)

  // bob make the same move since it is his turn
  await moveHelper(create_game_result.Ok, bob, 1, 1, "Right", true)

  // alice and bob alternate to complete the game
  await moveHelper(create_game_result.Ok, alice, 0, 1, "Up", true)
  await moveHelper(create_game_result.Ok, bob, 1, 0, "Right", true)
  await moveHelper(create_game_result.Ok, alice, 2, 0, "Up", true)
  await moveHelper(create_game_result.Ok, alice, 1, 1, "Up", true)
  await moveHelper(create_game_result.Ok, bob, 0, 2, "Right", true)
  await moveHelper(create_game_result.Ok, bob, 1, 2, "Right", true)
})

diorama.run()
