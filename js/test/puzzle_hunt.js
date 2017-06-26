var test = require('tape')

var PuzzleHunt = require('../puzzle_hunt.js')

test('puzzle hunt is exported', function(t) {
  t.plan(1)
  t.equal(typeof PuzzleHunt, 'function')
})
