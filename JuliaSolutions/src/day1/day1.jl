module Day1
export solution_p1, solution_p2

include("../common.jl")
using .Common

const inputs::Vector{String} = input_tolist("src/day1/input.txt")

solution_p1()::Number = begin
  result = mapreduce(parse_digits, +, inputs)
  result
end

solution_p2()::Number = begin
  # I did this already in rust, i dont really want to do it again... lol
  10
end

parse_digits(line::String)::Int32 = begin
  firstindex = findfirst(isdigit, line)
  lastindex = findlast(isdigit, line)

  combine_digits(line[firstindex], line[lastindex])
end

combine_digits(a::Int32, b::Int32)::Int32 = 10a + b
combine_digits(a::Char, b::Char)::Int32 = begin
  left = parse(Int32, a)
  right = parse(Int32, b)

  combine_digits(left, right)
end



combine_digits(a::String, b::String)::Int32 = begin

end


end
