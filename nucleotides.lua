-- This is a nucleotides counting script

function dna(sequence, table)
  for i = 1, #sequence do
    local nuc = string.sub(sequence,i,i)
    table[nuc] = table[nuc] + 1
  end
end

nucleotides = {
  A = 0,
  G = 0,
  T = 0,
  C = 0 }

dna("ACGTCCCCGAGC", nucleotides)

print("A: "..nucleotides.A)
print("G: "..nucleotides.G)
print("T: "..nucleotides.T)
print("C: "..nucleotides.C)
