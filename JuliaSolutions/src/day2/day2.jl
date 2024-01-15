module Day2
export solution_p1, solution_p2

include("../common.jl")
using .Common

mutable struct Game
  id::UInt32
  red::UInt32
  green::UInt32
  blue::UInt32

end

const INPUTS::Vector{String} = input_tolist("src/day2/input.txt")

const RED_MAX = 12
const GREEN_MAX = 13
const BLUE_MAX = 14

println(g::Game) = begin
  "Game $g.id { red: $g.red, green: $g.green, blue: $g.blue}"

end

solution_p1() = begin

  games = []
  sizehint!(games, length(INPUTS))


  for line in INPUTS
    game = parse_togame(line)
    push!(games, game)
  end

  isgame_possible(g::Game)::Bool = begin
    (g.red < RED_MAX) && (g.green < GREEN_MAX) && (g.blue < BLUE_MAX)
  end

  games = filter(isgame_possible, games)

  for g in games
    println(g)
  end
  # n = mapreduce(g -> g.id, +, games)
  # println(n)

end

solution_p2() = begin end


parse_togame(line)::Game = begin
  game, rounds = split(line, ":")
  rounds = split(rounds, ";")

  game = parse_gameid(game)


  for round in rounds

    colors = map(strip, split(round, ","))
    for color in colors

      n, c = split(color, " ")
      n = parse(UInt32, n)

      if c == "red"
        game.red += n
      elseif c == "blue"
        game.blue += n
      elseif c == "green"
        game.green += n
      else
        continue
      end
    end
  end

  game
end

parse_gameid(gstr)::Game = begin
  g, id = split(strip(gstr), " ")
  @assert g == "Game"
  id = parse(UInt32, id)
  Game(id, 0, 0, 0)
end

end
