# Experimental!

~ "git: github.com/amatgil/steal-gift" ~ AoCGet

# ====== INPUTS ======
Input ← AoCGet 2024 2

# ====== PART 1 ======
# https://www.uiua.org/pad?src=0_14_0-dev_5__JCA3IDYgNCAyIDEKJCAxIDIgNyA4IDkKJCA5IDcgNiAyIDEKJCAxIDMgMiA0IDUKJCA4IDYgNCA0IDEKJCAxIDMgNiA3IDkKCmEg4oaQIFsxIDIgM10KYiDihpAgWy0xIC0yIDNdCgriipwo4oqc4ouV4omgQCAuKeKJoEBcbi4K4omh4perMgriiaHigoIvLQoKYQo_Ci_iip49Cg%3D%3D
# Part₁ ← ⊜(⊜⋕⊸≠@ )⊸≠@\n
Part₁ ← /+≡⌵≡/-⍉ ≡⍆ ⍉ ≡∵⋕ ∵(↘1⊢regex "(\\d+)\\s+(\\d+)") ⊜□⊸(≠@\n)

# ====== PART 2 ======
Part₂ ← ? /⊞= ? . ?♭↘1⇌:♭↘1. ⍉ ≡∵⋕ ∵(↘1⊢regex "(\\d+)\\s+(\\d+)") ⊜□⊸(≠@\n)


# ====== TESTS =======
# ⤙≍ 11 Part₁ "3   4\n4   3\n2   5\n1   3\n3   9\n3   3"
⤙≍ 31 Part₂ "3   4\n4   3\n2   5\n1   3\n3   9\n3   3"

# ======== RUNNING ========
# $"Part one solution is: '_'" Part₁ Input

