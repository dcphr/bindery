var PuzzleHunt = function(options ){
  var defaultOptions = { }
  this.options = this.extend(options, defaultOptions)
}

PuzzleHunt.prototype = {
  extend: function(source, target ){
    if (source == null) { return target }
    for (var k in source) {
      if (source[k] != null && target[k] !== source[k]) {
        target[k] = source[k]
      }
    }
    return target
  }
}

module.exports = PuzzleHunt
