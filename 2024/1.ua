# Experimental!

~ "git: github.com/amatgil/steal-gift" ~ AoCGet

# ====== INPUTS ======
Input ← AoCGet 2024 1

# ====== PART 1 ======
# Part₁ ← ⊜(⊜⋕⊸≠@ )⊸≠@\n
Part₁ ← /+≡⌵≡/-⍉ ≡⍆ ⍉ ≡∵⋕ ∵(↘1⊢regex "(\\d+)\\s+(\\d+)") ⊜□⊸(≠@\n)

# ====== PART 2 ======
Part₂ ← ? /⊞= ? . ?♭↘1⇌:♭↘1. ⍉ ≡∵⋕ ∵(↘1⊢regex "(\\d+)\\s+(\\d+)") ⊜□⊸(≠@\n)


# ====== TESTS =======
# ⤙≍ 11 Part₁ "3   4\n4   3\n2   5\n1   3\n3   9\n3   3"
⤙≍ 31 Part₂ "3   4\n4   3\n2   5\n1   3\n3   9\n3   3"

# ======== RUNNING ========
# $"Part one solution is: '_'" Part₁ Input

