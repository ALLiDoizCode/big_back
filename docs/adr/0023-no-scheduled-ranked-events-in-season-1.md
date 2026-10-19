# No scheduled ranked events in Season 1

Season 1 has **no scheduled ranked events**. The one queue
([0012](0012-one-rated-queue-with-short-starts.md)), open at every hour, is the whole ranked
offer: no prime-time windows, no event rewards and no event formats. This settles the timed
ranked events that [0009](0009-progression-buys-identity-only.md) handed to
[0011](0011-divisions-and-points-over-a-hidden-rating.md), and the "events may still come
later" layer 0012 left open. We chose this in
[#55](https://github.com/ALLiDoizCode/big_back/issues/55) (grilling). The human agreed with each
recommendation. The revisit thresholds are tuning, not decisions.

## Why none

- **Short starts already fixed the waits.** In 0012's sim the off-peak 90% stayed at about 1:00
  even at 60 players online at the busiest hour. What a thin queue still costs is bot seats:
  about 28% of matches unrated at 60, about 3% at 200. That's the only thing an event could
  improve.
- **A spike costs servers.** Egress dominates the server bill, and the cheap options are fleets
  sized for the peak: bare VMs or GameLift, about $0.09–0.11 a match-hour
  ([#40](https://github.com/ALLiDoizCode/big_back/issues/40)). An event pulls players into one
  hour, so the fleet is sized for that hour and sits idle the rest of the day. Per-use hosting
  absorbs a spike but costs about 4× as much a match-hour.
- **Rank pays for skill, not for time** (0011). An event reward would pay for playing at a
  certain hour, and would be a second reward track beside the ranked-win rewards (0009).
- **There is nothing else to play.** Season 1 has one mode
  ([0014](0014-one-mode-at-launch-crate-race.md)) and one map
  ([0020](0020-one-big-lanes-and-jungle-map-at-launch.md)), and the 3v3 map isn't shipped.

## Instead

- **Busy hours are shown, not scheduled.** The estimated wait 0012 left open can also show when
  a region is busiest ("busiest around 20:00"). That nudges players toward the same hours
  without closing ranked at other times or paying for it. Its design stays with 0012's open
  question.
- **Player-run events use practice lobbies** ([0019](0019-a-tutorial-then-the-rated-queue.md)):
  any party shape, any mix of humans and bots, no party cap, unrated and earning nothing.
  Season 1 adds no brackets, spectating or tournament tooling.

## When to look again

At the Season 2 review, per region and hour, check:

- **the share of unrated matches** (those with a bot seat), which the server already tracks for
  the bot ([0021](0021-one-bot-that-plays-like-an-average-player.md));
- **the off-peak 90% wait.**

If unrated matches are above about 10% across a day, or the off-peak 90% wait is above about
2:00, prime-time windows go back on the table. A person makes the call; nothing turns on by
itself. At that point the end-of-life fallback from #40 may be the better answer.

## Considered options

- **Prime-time windows:** the same queue and rating, no reward, just a time advertised to pull
  players together. It only helps the bot-seat share at the lowest population, and it sizes the
  fleet for the spike. This is the first option to revisit.
- **Reward events:** the same queue and rating, with extra cosmetics or faster tier progress for
  wins in the window. It pays for when someone plays, not how well, and adds a second track.
- **Format events:** a different mode, map or team size. It would reopen 0014 and 0020, and
  Season 1 has nothing to run them on.
- **Ranked only during events:** already rejected in 0012, with ranked closed 21 hours a day.

## Consequences

- **Matchmaking needs no schedule**, and the fleet is sized for the natural daily peak only.
- **The server reports unrated-match share and waits per region and hour**, so the Season 2
  review has its numbers.
- **The estimated-wait design** (open in 0012) picks up the busiest-hours hint.
- **Settles** the timed ranked events left open by 0009 and 0011, and the event layer 0012 left
  for later.
