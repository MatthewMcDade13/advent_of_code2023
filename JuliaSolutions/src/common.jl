module Common
export input_tolist

const AOC_INPUT::UInt32 = 1000

input_tolist(filepath::String)::Vector{String} = begin
  list::Vector{String} = []
  sizehint!(list, AOC_INPUT)

  open(filepath) do file
    for line in eachline(file)
      push!(list, line)
    end
  end
  list
end

end
