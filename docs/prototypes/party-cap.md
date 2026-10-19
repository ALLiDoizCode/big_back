# PROTOTYPE sheet: the party skill cap (issue #47)

Throwaway. Lives on `prototype/party-cap` only, branched from `prototype/queue-structure`
(the one-day queue simulator from #14).

Open `docs/prototypes/party-cap.html` in a browser (double-click; no install, no server).
Hosted copy: https://claude.ai/artifact/SBVAh2N7QoTcrowykw9UCo (private to the owner).

**Question:** how wide may a party's skill spread be in the one rated queue? ADR 0018 fixed the
cap's shape (shown in tiers and divisions, enforced on the wider of rank and rating, wider for
bigger parties, a full 5-stack any width for a sliding point cut). This sets the widths and the
5-stack cut. Resolved when the human picks the widths per party size and the 5-stack schedule.

## What changed in the simulator

- **Each member keeps their own skill.** Friends' skills are correlated: each member is √ρ of a
  shared draw plus √(1−ρ) of their own (ρ = 0.5 by default).
- **Rank and rating.** A member's hidden rating maps to ADR 0011's tiers by share of players
  (15 / 25 / 27 / 18 / 10 / 4 / 1%), 3 divisions per tier (Master and Champion count as 3 wide).
  Their shown rank sits a random ±1 division off (by default). The cap reads the wider spread.
- **A party over its cap drops a member** (picked for this sheet, per ADR 0018's "the fix is
  always plain: drop a member"): its furthest-out member queues solo, until the rest fit the
  cap for their new size. Splitting everyone up and turning the party away are settings too.
- **Carry.** Under ADR 0010 a team is the sum of its players plus a party bonus, so a party's
  spread on its own changes nothing. The sim adds a carry: each weaker member plays a share of
  the way up toward the party's best (25% by default). The matchmaker can't see it; it only
  knows the party bonus it learned, here the real average over all parties queuing. Carry is
  a guess; real match data would measure it.
- **Sessions end.** After each match a player logs off with chance 1 in 4 and a new one
  arrives, so a day has about 480 parties instead of about 25 that never leave.
- **A fix.** The queue-structure sheet only let a party log off when the region stayed at or
  above its curve without them, which almost never fit, so parties piled up. Its "25% in
  parties" ran at about 48% of matched players (checked: 42–52% over its 4 days). **ADR 0012's
  "68% close" was measured with about half the players in parties.** Fixed, 25% in parties
  gives 83% close at 200 (no carry) or 78% (carry 25%), against 99% solo only.

## Findings (5v5, full lobbies, 25% arriving in parties, medians of 4 days)

Close = the favourite's real chance at most 55%. Blocked = share of parties over the cap when
they queued (most still play, a member short).

**Carry 25% (the page's default):**

| Cap (duo / trio / 4-stack, tiers) | Close at 200 | at 100 | at 60 | Parties blocked | Matched players in parties |
|---|---|---|---|---|---|
| Solo players only | 99% | 100% | 100% | — | 0% |
| No cap | 78% | 77% | 76% | 0% | 23% |
| 3 / 3 / 3 | 83% | 80% | 79% | 9% | 22% |
| 2 / 3 / 4 | 83% | 82% | 83% | 18% | 20% |
| **2 / 2 / 3** | **86%** | **84%** | **86%** | **28%** | 20% |
| 2 / 2 / 2 | 87% | 86% | 85% | 30% | 20% |
| 1 / 2 / 3 | 89% | 87% | 88% | 48% | 15% |
| 1 / 1 / 1 | 93% | 93% | 93% | 66% | 13% |

**Carry 0 (only the sum counts):** no cap 83%, 3 / 3 / 3 85%, 2 / 2 / 3 88%, 2 / 2 / 2 89%,
1 / 1 / 1 93% at 200, with the same blocked shares.

- **Waits don't move.** Every cap kept the off-peak median and 90% within a few seconds of no
  cap, at every population (200: 30 s / 1:30; 100: about 50 s / 3:00; 60: about 1:05 / 5:00).
- **Without carry, a cap only helps by making parties rarer.** No cap with 14% of matched
  players in parties gives 91% close; a 1-tier cap, which leaves 14% in parties, gives 93%.
  The lever is party count, not party width.
- **With carry, width matters on top of that:** about 5 more points than no cap at the same
  party share (no cap at 20% in parties: 82%; 2 / 2 / 2 at 20%: 87%). The matchmaker
  under-rates wide parties: close "as predicted" runs up to 6 points above the real figure
  with no cap, and the gap closes as the cap narrows.
- **1 tier blocks most parties.** Tiers are narrow (Diamond is 10% of players), and rank lag
  adds to the spread. Even friends who are alike (ρ = 0.5) are over 1 tier apart in about 2 of
  3 parties.
- **The blocked share hangs on two guesses**, how alike friends are and how far rank lags:

  | Blocked at 2 / 2 / 3 | rank = rating | ±1 division | ±2 divisions |
  |---|---|---|---|
  | ρ = 0.3 | 31% | 35% | 43% |
  | ρ = 0.5 | 22% | 26% | 34% |
  | ρ = 0.7 | 11% | 16% | 23% |

  Real party data at launch should re-set the widths.
- **Out of depth barely moves** (about 28% of players more than 25 percentile points from their
  lobby's average at 200, 35% at 60, with or without a cap). The skill band, widening to anyone
  at 2 minutes, causes most of it, not parties.
- **Short starts swamp it.** With ADR 0012's short starts and bots in empty seats, close
  matches are 49% solo only, 39% with no cap and 43% at 2 / 2 / 3 (200), and 18–20% at 60
  whatever the cap. Late joiners and bots decide fairness there, not party width.
- **Over the cap: drop, split or turn away** give about the same close share at the same cap.
  Turning away costs those players their session, so drop (ADR 0018) stays.

## The 5-stack cut

A full 5-stack is a whole team, so its width only hurts its opponents through carry the
matchmaker can't see. The cut on points won that cancels its edge is c = (2p − 1) / p, for its
real chance p in a match predicted at 50%. With 2 / 2 / 3 caps (sampled, 40,000 stacks):

| 5-stack spread | Share of stacks | Cut that cancels the edge, carry 10% / 25% / 50% | Suggested |
|---|---|---|---|
| up to 3 tiers | 70% | 0–8% / 0–19% / 0–34% | none |
| 3–4 tiers | 23% | 15% / 34% / 57% | −25% |
| 4–5 tiers | 6% | 23% / 48% / 74% | −50% |
| over 5 tiers | 1% | 31% / 60% / 86% | −75% |

Suggested schedule: **no cut up to the 4-stack cap, then −25% per tier beyond it, at most
−75%.** At carry 25% it sits close to the cancelling cut; at carry 10% it over-cuts, which is
the safe side for boosting.

## Recommendation (verdict pending)

- **Caps: 2 tiers for duos and trios, 3 tiers for 4-stacks** (2 / 2 / 3). It gets back about
  a third of the gap to solo-only (78% → 86% at carry 25%; 83% → 88% at none) and blocks about
  a quarter of parties, most of which still play a member short. Nothing costs wait.
- 1 / 1 / 1 gets back most of the gap (93%) but blocks two parties in three; 3 / 3 / 3 blocks
  few (9%) but buys little (78% → 83%).
- **5-stacks:** exempt, cut −25% / −50% / −75% for each tier past 3.
- **Re-fit with real data:** carry, how alike friends are, and rank lag decide these numbers.
