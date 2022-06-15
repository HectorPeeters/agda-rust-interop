data Bool : Set where
  true false : Bool

not : Bool → Bool
not true  = false
not false = true

data Nat : Set where
  zero : Nat
  suc  : Nat → Nat

one = suc zero
two = suc one
three = suc two

plus : Nat → Nat → Nat
plus zero n = n
plus (suc m) n = suc (plus m n)
