
# The Existenz White Paper

## A Fair Global Currency: Providing a Basic Human Right Through Virtual Currency

*A project by and for utopian and agnostic citizens of the world.*

## Abstract / Executive Summary

The Existenz (EXZ) project proposes a fair global virtual currency designed to guarantee, through monetary design itself, a subsistence floor for every human being.

**The inspiration.** Article 3 of the Universal Declaration of Human Rights ("Everyone has the right to life, liberty and security of person"), read as implying access to the material means of staying alive: food, shelter and basic care.

**The design in one sentence.** Every verified human existence receives the same daily issuance of a purpose-built currency that cannot be hoarded: accumulation is capped per account, and idle balances melt, recycling wealth into a global solidarity fund.

**What this paper does.** It formalizes the concept presented in the original Existenz presentation, develops its monetary mechanics, examines two distribution scenarios, addresses governance, identity and technical requirements, and provides an economic and risk analysis together with an implementation roadmap.

**The annexes.** Annexes A to G propose robust protocols for the structural weaknesses identified: human uniqueness, or Sybil resistance (A); geographic value anchoring (B); melting equity (C); the three-tier governance matrix (D); fully offline existence verification at the point of sale for holders who own no phone and carry no physical credential (E); immediate intraday transaction security for the merchant, and end-of-day settlement of merchant value into local currency under a strict single-hop rule, at a statistically bounded, abuse-resistant conversion rate (F); extended offline operation secured by a daily regenerated cryptographic trace and opportunistic peer-to-peer relay sync (E.11); and a quantitative solvency model of the global fund (G).

**What is actually guaranteed.** The universal guarantee is goods-denominated, not currency-denominated: what is promised to every holder everywhere is the meal, while conversion into local currency is defined as a bounded, coverage-indexed public service of the global fund rather than an unconditional exchange.

**Beyond the floor.** Annex H extends the design in three directions: the issuance rate as a governance parameter, the bridge from subsistence to universal income; a bounded merchant circuit that reduces conversion demand; and an honest treatment of credit and lending outside the protocol, including the inalienability of the issuance stream.

# 1. Introduction: Recognizing Human Existence

**The starting point.** Modern monetary systems reward accumulation and speculation, while roughly a quarter of humanity lives below subsistence thresholds and more than a billion people remain financially excluded.

**The route taken.** Instead of taxing and redistributing within national borders, Existenz embeds distribution into the currency protocol itself. Every verified human existence receives the same amount of a purpose-built virtual currency.

**Why it works.** Because the currency melts when hoarded and is capped per account, it functions as a medium of subsistence and circulation rather than a store of value. Each human existence is recognized, economically, by the network.

**The core thesis.** A well-designed melting currency with equal issuance, an accumulation cap and geographic value adjustment can operationalize the right to subsist at global scale, without relying on any state budget.

## 1.1 Why now: the technological condition

Existenz is made possible now, and only now, by the diffusion of blockchain technology. For the first time, a protocol can issue, cap, melt, verify and settle a currency without any state, bank or central counterparty, at global scale, at near-zero marginal cost, and with rules enforced by mathematics rather than by institutions. Equal issuance to every human being was a philosophical proposition for centuries; it becomes an engineering proposition the moment a distributed ledger can hold the rules themselves. The blockchain is not a feature of Existenz: it is its condition of existence.

## 1.2 The founding stewardship

Launching the protocol requires a founding team that fixes the initial parameters before any community exists to vote on them. This launch oligarchy is assumed openly: it is deliberately altruistic and disinterested, its mandate is to install a universal means of subsistence and nothing else, and its legitimacy is bounded by the handover. Every constitutional and parametric decision belongs, from the moment the community reaches voting scale, to the two-thirds rule of all account owners. Founding stewardship is a transition mechanism, not a regime: the founders' measure of success is their own obsolescence.

# 2. Philosophical and Legal Foundations

Article 3 of the Universal Declaration of Human Rights states: "Everyone has the right to life, liberty and security of person." A right to life, taken seriously, implies access to the material means of staying alive: food, shelter and basic care. Article 22 (right to social security) and Article 25 (right to a standard of living adequate for health and well-being) reinforce this reading.

Existenz translates this principle into a monetary protocol:

- Recognition: the act of existing is the sole eligibility criterion for issuance.
- Equality: the issuance amount is identical for every account holder.
- Non-accumulation: the currency cannot become an instrument of structural domination, because holdings above the cap melt or are transferred to the global fund.
- Universality: the protocol operates at global scale, unconstrained by nationality or residence.

The project also stands in the lineage of monetary thinkers such as Silvio Gesell (Freigeld, stampscrip), the WIR complementary currency in Switzerland, and contemporary universal basic income experiments. Gesell is invoked here for one idea only, the carrying cost on idle money; the social-darwinist passages of his 1916 preface are rejected without reservation (see the note in the references). Existenz differs from a classic UBI in that it does not transfer existing national money; it issues a dedicated instrument whose properties prevent hoarding and encourage circulation, closer to a barter-like exchange medium than to savings.

# 3. Problem Statement

The problems Existenz is designed to address are:

- Absolute poverty: hundreds of millions of people lack daily access to basic needs.
- Financial exclusion: a large share of the global population is unbanked and cannot participate in digital exchange.
- Accumulation dynamics: conventional money rewards hoarding and speculation, concentrating wealth and draining purchasing power from local circulation.
- Aid friction: humanitarian transfers are slow, costly, geographically uneven and politically conditional.
- Currency asymmetries: the same nominal amount of money purchases very different baskets of goods depending on where one lives.

Existenz answers each point respectively with: equal per-capita issuance, a wallet for every verified human, melting and capped holdings, a blockchain-validated global emergency fund, and geographic value adjustment.

# 4. The Existenz Ecosystem: Core Rules

The protocol rests on five founding rules, as defined in the original presentation:

- Equal issuance: the same amount of Existenz is issued for each human being.
- Capped accumulation: each account has a maximum balance.
- Melting currency: stored balances lose value over time at a defined melting rate.
- Geographic value basis: the purchasing value of the Existenz is geographically anchored, by blockchain decision.
- Protocol democracy: any further modification of these rules must be validated by a two-thirds majority of account owners.

These rules ensure that no perverted accumulation system can emerge. Any amount held above the cap of two blocs is automatically transferred to a global fund, which acts as an emergency solution activated through blockchain validation (10 validations within a defined geographical range). At the end of a human existence, the remaining balance is transferred to the global fund, suspending its melting process.

# 5. Monetary Mechanics

## 5.1 Melting Currency (Demurrage)

A melting currency carries a negative carrying cost: idle balances shrink over time. This is not inflation of the money supply but a transfer of value from idle balances to circulation. Effects:

- High velocity: holders are incited to spend rather than store, since saving in EXZ is structurally penalized.
- Local circulation: melting encourages spending within one's immediate economic environment, similar to barter dynamics.
- Reduced speculation: the currency is unattractive as a speculative asset, which stabilizes its exchange function.

The melting process of the global fund is suspended, so that the fund can accumulate the transfers it receives (excess balances above caps, balances of deceased account holders) and act as a stable emergency reserve.

## 5.2 Accumulation Cap and the Global Fund

Each account is limited to a maximum balance. Amounts above the cap flow to the global fund. The fund serves as:

- An emergency mechanism: activation requires 10 blockchain validations within a geographical range (for example, validators geographically distributed around a disaster area), preventing both fraud and unilateral capture.
- A solidarity recycling channel: wealth that would otherwise concentrate returns to the commons.
- A death-settlement mechanism: at the end of an existence, remaining EXZ joins the fund rather than any heir or estate, reinforcing the principle that the currency recognizes existence, not inheritance.
- The merchant liquidity backstop, bounded: the fund buys merchant settlement credit back into local currency at the anchored rate, within the statistical caps of Annex F and the coverage-indexed ration of Annexes G and H.1. It is a buyer of last resort within a published ration, not a universal exchange. This is the economic role that pays for the verification work merchants perform.

## 5.3 Geographic Value Basis

The same human need (a meal, a liter of clean water) costs different amounts in different places. Existenz therefore anchors its purchasing value geographically: the blockchain validates local value references so that one EXZ provides comparable access to basic needs anywhere on Earth. The full anti-manipulation design is specified in Annex B.

## 5.4 Two Distribution Scenarios

The original presentation leaves the fair distribution segmentation open. The two candidate models are formalized below.

Scenario A - Century model:

- Assumption: a human life can reach 100 years, with 365 days per year; one lifetime is valued at 36,500 EXZ.
- Issuance: 1 EXZ per person per day over a lifetime.
- Accumulation cap: two days of issuance (2 EXZ) in per-day terms; the lifetime framing yields the 73,000 EXZ figure for a full unspent life, and the cap is expressed in days of issuance for coherence.
- Melting rhythm: a loss of 1 EXZ per week of idle balance.
- Excess: any unit above the cap transfers to the global fund.

Scenario B - Subsistence model:

- Assumption: a person eats twice per day; 2 EXZ are created per day.
- Horizon: with a life extent reference of 60 days, the melting rate is 1 unit lost at every 60th, i.e. a melting currency at approximately 1.6% per period.
- Reset: the melting rate is reset at each exchange, rewarding active circulation.

Comparative analysis:

| Dimension | Scenario A (century) | Scenario B (subsistence) |
|---|---|---|
| Daily issuance | 1 EXZ | 2 EXZ |
| Frame of reference | 100-year life, 36,500 EXZ | 60-day cycle, meal-based |
| Melting | 1 EXZ per week | ~1.6% per period, reset on exchange |
| Simplicity | High (integer, weekly) | Medium (rate, reset logic) |
| Behavioral signal | Mild incentive to spend | Strong incentive to circulate |
| Best suited for | Stable long-term baseline | High-velocity local economies |

Scenario A offers simplicity and stability; Scenario B ties issuance directly to the most immediate need (food) and maximizes circulation. A hybrid path is plausible: Scenario A issuance with a Scenario B-style melting reset at exchange, and both parameters governed by the two-thirds rule so the community can tune the equilibrium.

# 6. Governance
- One account, one person, one vote: every verified account owner participates in protocol decisions. A merchant is a citizen like any other: merchant status neither adds voting weight nor excludes from any protocol decision, one vote per living human, intangible and inalienable. Their specific economic interests are addressed by transparency of debate and by the parametric scope rules, never by disenfranchisement.
- Supermajority: any modification of the five founding rules requires a two-thirds majority of account owners.
- Geographic fund activation: emergency disbursement requires 10 validator signatures within the relevant geographical range.
- On-chain transparency: rule changes, fund flows and validations are public and auditable.
- Founding stewardship: until the community reaches voting scale, the founding team exercises launch stewardship under the mandate and sunset conditions of Section 1.2.

The refined three-tier governance matrix (constitutional, parametric, emergency) is specified in Annex D.

# 7. Technical Architecture Requirements
- Distributed ledger: issuance, melting, caps and fund transfers must be protocol-level, not policy-level, operations.
- Human uniqueness (Sybil resistance): each account must map to one living human being. The full protocol is specified in Annex A.
- Offline access: basic-need usage cannot assume connectivity, and it cannot assume that the holder owns a phone or carries any physical credential. Working from the realistic premise that roughly 70% of the target population owns a mobile phone, the Local Edge Verification Protocol of Annex E specifies two modes: a device mode (roughly 7 holders out of 10) where the holder's own secure element validates the transaction through the blockchain at the next sync, and a cardless presence mode for the remaining minority, where the merchant's phone alone performs the entire verification from a photo and voice capture analyzed by an on-device algorithm. The physical printed card, considered as a verification artifact, was examined and deliberately abandoned: it is falsifiable, it can be lost, and it adds a fragile object to a protocol that needs none. Extended offline operation, including the
case of a merchant away from any connectivity for up to a year, is secured by the day-chain trace and the opportunistic relay sync of E.11. In both modes, the merchant obtains immediate intraday security through the intraday presence template mechanism of E.5.
- Merchant liquidity and the single-hop rule: merchants must be able to convert earned EXZ into local currency at the end of each day, at a public, anchored rate, within statistical caps; EXZ never circulates beyond the merchant (N+1 rule). The full settlement and conversion protocol is specified in Annex F.
- Melting at protocol level: the ledger must compute demurrage natively (per-block or per-epoch decay) so that wallets cannot avoid it.
- Validators with geographic attestation: the geographic value basis and the 10-validation emergency rule require validators to be verifiably distributed across geographic zones.

# 8. Economic Analysis
- Money properties: EXZ deliberately abandons the store-of-value property to maximize medium-of-exchange performance. Like Gesell's Freigeld, it privileges circulation over accumulation.
- Velocity and demand: melting creates continuous spending pressure; the daily issuance stream creates continuous replenishment, a steady-state economy of subsistence.
- Inflation control: because balances are capped and melt, the effective money in circulation is bounded; issuance replaces melted units rather than expanding the base indefinitely.
- Speculation resistance: with no store-of-value function, a cap, and a single hop after which the currency exits to local money at a fixed indexed rate, EXZ offers no attractive speculative position and no arbitrage loop.
- Comparison with UBI: Existenz avoids fiscal dependence and national borders, but requires global identity infrastructure and does not by itself provide stores, logistics or services that accept the currency; merchant adoption is a first-order success factor, and Annex F gives merchants a same-day liquidity guarantee, which is the single strongest adoption lever. The issuance rate being a governance parameter also means the mechanism itself is the universal-income architecture of H.3, with only the subsistence level currently funded.

# 9. Risk Analysis and Open Questions

Identity and Sybil attacks: fraud (duplicate accounts, false deaths) is the primary systemic risk. Resolved by the layered protocol of Annex A.

Privacy: identity verification must not become surveillance; the zero-knowledge approach of Annex A is designed so that the chain verifies existence proofs, never identities. Annex E extends this guarantee offline: presence captures become day-bound encrypted templates, auto-purged after the audit window, never uploaded raw.

Death-settlement integrity: the liveness attestation cycle of Annex A (issuance suspension, grace period, transfer to the global fund) makes settlement reliable and non-perverse.

Geographic anchoring manipulation: local value references could be gamed by colluding validators; the Basic Needs Basket index and committee sampling of Annex B mitigate this structurally.

Melting regressivity: saving for a lumpy expense (medicine, repair) is penalized; the equity layer of Annex C (exemption floor, targeted sub-wallets) addresses this without breaking the anti-accumulation logic.

Offline double-spending and merchant abuse: offline acceptance of EXZ (Annex E) risks replay, double-spending and, in the cardless presence mode, fabricated transactions by a dishonest merchant; the intraday presence template of E.5, provisional settlement of E.6, the cap, stake and audit regime of E.7, and the day-chain continuity proof of E.11 bound this risk.

Gray allocations market: a holder selling or pledging their daily allowance for cash is as old as rationing systems and cannot be prevented by any technology, because biometrics prove presence, not freedom. The protocol does not attempt to police individual behavior; its only effective lever is on the beneficiary, the merchant, whose monetizable volume is bounded by the caps of Annex F regardless of how the allowance reached them (F.6).

Value-chain fraud: allowing EXZ to circulate beyond the merchant (supplier chains, B2B circuits) would multiply fabrication opportunities and break the BNB anchoring; the single-hop rule of Annex F, enforced at protocol level, removes this surface entirely.

Conversion abuse and fund drain: same-day conversion into local currency (Annex F) could be abused to drain the global fund; the statistical cap band, tier-based limits, integrity-indexed advance fraction and retrospective adjustment regime of Annex F are designed precisely against this.

Regulatory: issuance of a global currency intersects with monetary sovereignty, KYC/AML law and securities regulation in many jurisdictions; a sandbox and progressive regional rollout are advisable. Conversion agents of Annex F fall under local payment regulation and apply local KYC and tax rules.

Adoption: a currency is only as useful as what it can buy; anchor use cases (food, telecom credit, utilities) must be secured early. Annex E gives every small merchant a direct, low-cost role in the ecosystem, and Annex F guarantees the merchant same-day liquidity at a known rate.

# 10. Implementation Roadmap
- Research phase: formalize melting mathematics, validate the RHUP identity protocol of Annex A, the BNB index of Annex B, the equity layer of Annex C, the offline protocols of Annex E, the day-chain and relay-sync design of E.11, and the single-hop settlement and conversion design of Annex F through public review of this white paper.
- Prototype: testnet with a small cohort, Scenario A parameters, simulated geographic zones, RHUP enrollment on a volunteer web of trust, offline payment pilots with participating merchants, including cardless presence verification, intraday template capture, the offline day-chain, opportunistic BLE relay sync between devices, and a simulated conversion agent.
- Pilot: one or two regions with basic-need merchants (food, water, telecom), measure velocity, adoption, fraud rate, index manipulation attempts, offline double-spend attempts, template replay attempts, merchant abuse attempts, day-chain continuity across long offline periods, relay sync latency in sparse populations, single-hop enforcement, conversion demand vs. statistical caps, and merchant circuit take-up vs. direct conversion (H.2).
- Parameter settlement: first two-thirds vote of the account owners, ending the founding stewardship of Section 1.2, on distribution model (A, B or hybrid), cap level, melting rate, floor exemption, sub-wallet limits, offline session limits, template retention window, tolling rules of E.11, merchant cap tiers, commission rate, advance fraction and conversion caps of Annex F, and the merchant circuit fraction of H.2.
- Scale-up: progressive onboarding by geographic zones under the three-tier governance matrix of Annex D.

# 11. Known Limits and Structural Weaknesses: a Deliberately Honest Statement

This section departs from the promotional tone of most white papers. Existenz is an audacious design, and its weaknesses are real, structural and, in several cases, not fully solvable by the protocol itself. They are stated here explicitly, first because intellectual honesty is a condition of credibility, second because a community asked to govern such a system by a two-thirds rule must know exactly what it is governing, and third because the parameter votes of the roadmap cannot be informed votes if the weaknesses are hidden.

## 11.1 The scale question: what if all eight billion actually spend their daily EXZ

Under Scenario B, a fully deployed Existenz issues up to 16 billion EXZ per day, and the design intends every one of them to be spent on basic needs. This raises two distinct questions.

What happens to unused EXZ: this case is benign by construction. An unused daily allowance is not debt and not a stockpiled promise: it is simply units that melt, and above the cap, units that flow to the global fund. The money base does not grow with non-use, because issuance replaces melted units (Section 8) and the cap of a few days of issuance bounds any individual accumulation to a negligible quantity. In the aggregate, massive non-use would even reduce conversion pressure on the fund (11.2). Non-use is, monetarily, the least dangerous failure mode of the system.

What happens if everyone uses them: this is the serious case, and it is the macroeconomic weak point of the entire design. If the whole enrolled population spent its allowance daily, and if every merchant converted all received EXZ into local currency at the anchored rate, the global fund would have to pay out, every day, the local-currency value of the basic needs basket of the entire human population. No endogenous fund can honor that: the fund receives EXZ (melted units, excess above caps, death settlements), and EXZ is not local currency. The fund's guarantee of Section 5.2 is therefore only as good as its external endowment and its utilization asymmetry. In other words, the system's local-currency solvency structurally relies on the fact that a large fraction of allowances is not spent at converting merchants in the same period, which is a fact, not a guarantee: the equilibrium
could break precisely where Existenz succeeds most.

Mitigations, partial and stated as such:

- The caps of Annex F already ration conversion per merchant (physical, solidarity, tier), which slows any drain but does not create local-currency liquidity.
- The bounded merchant circuit of H.2 lets merchants spend a capped fraction of their settlement credit on basic-need goods from other merchants before converting: every EXZ spent in the circuit is an EXZ the fund does not have to buy back, reducing the conversion rate v structurally instead of merely capping it.
- The fund must publish a live coverage ratio (endowment available for conversion vs. conversion demand) and dynamically ration conversion when the ratio degrades, converting the guarantee from an unconditional promise into a transparent, queue-managed public service. Rationing is a weakness in plain sight, and it is better than silent insolvency.
- The protocol must explicitly invite external endowment of the fund (philanthropic, institutional, eventually state contributions) as the honest answer to the circularity: a universal subsistence claim ultimately draws on real production, and real production must be paid for in instruments the producers accept. The melting-currency design reduces, but cannot abolish, this dependence.
- Long-term equilibrium relies on merchants accepting EXZ as customer demand, with conversion used only partially: the goal is an economy where EXZ circulates as a basic-need medium, not one where every unit immediately exits to fiat.

This weakness is not a reason to abandon the design; it is the boundary of what a currency protocol can do. Existenz can guarantee an equal, universal, non-hoardable medium of subsistence. It cannot, by monetary design alone, guarantee unlimited same-day fiat liquidity for the whole planet at once. The white paper refuses to pretend otherwise. The quantitative model behind this assessment is developed in Annex G.

## 11.2 The fund's circularity: an EXZ-rich fund is not a solvent fund

Corollary of 11.1: the global fund accumulates EXZ, but its obligations (emergency aid, merchant conversion) are largely denominated in real goods and local currency. Unless externally endowed, the fund is a circular bookkeeping device: it recycles claims, it does not create purchasing power. Annex H.1 resolves this structurally by redefining what the protocol guarantees: the floor is goods-denominated (the meal at the merchant) and holds by construction, while currency conversion is a bounded, coverage-indexed service whose exposure is capped by the operational-window demurrage of settlement credit, by the zone rollout gates and rationing queue of G.3, and reduced in volume by the merchant circuit of H.2. The residual weakness is therefore no longer solvency but communication: if the public reads the floor as a conversion guarantee, every rationing event will be experienced as a broken promise. The governance matrix must state the distinction in the protocol's own vocabulary: fund guarantee levels are a parametric rule that the two-thirds vote may only raise against demonstrated endowment, never on promise.

## 11.3 Identity at planetary scale: an unsolved problem, priced rather than solved

RHUP (Annex A) prices, detects and recovers from Sybil fraud; it does not eliminate it (A.5). Beyond collusion, the honest weaknesses are: enrolling eight billion humans through a sponsored web of trust is an operational feat with no precedent; biometric capture at that scale, even zero-knowledge, creates a single global attack surface that no nation-state system has ever offered; and exclusion errors (a real living person falsely rejected) hurt exactly the people the system exists for. False negatives are cheap to appeal but not free for a person with no connectivity, no sponsors and no documents. The web of trust may reproduce existing social inequalities: the well-connected enroll easily, the isolated do not.

## 11.4 Governance realism

Two-thirds of eight billion account owners is an abstraction. Real turnout will be low, concentrated among the connected and motivated, which shifts effective power toward an activist minority and the validators who frame the votes. The founding stewardship of Section 1.2 is a declared oligarchy with a sunset; the permanent risk is that governance capture simply migrates from the founders to whoever controls turnout, media and defaults. The parametric/constitutional split of Annex D mitigates drift, but no protocol immunizes a community against its own apathy.

## 11.5 State and institutional opposition

A currency issued to every human, outside any state budget, is structurally in tension with monetary sovereignty, capital controls, sanctions regimes and tax enforcement. The most predictable failure mode of Existenz is not technical: it is prohibition, criminalization of conversion agents, or diplomatic pressure on host states. The design's legality varies by jurisdiction, and the white paper cannot promise otherwise.

## 11.6 The merchant economy is load-bearing

The entire offline verification layer (Annex E) and the entire conversion layer (Annex F) rest on small merchants finding their role profitable. If the commission, margins and conversion liquidity fail to compensate the operational burden (captures, statements, audits, stake), merchants do not adopt, and Existenz becomes a currency that exists but cannot be spent anywhere. Merchant adoption is the single point of failure of the project. This is not a hypothetical risk: in the Circles UBI pilot in Berlin (2021-2023), the closest documented precedent, the currency lived and died with its merchant economy, and when the cooperative that subsidized merchant conversion into euros ran out of funding, the pilot ended with it (see the references).

## 11.7 Dependence on device infrastructure

The cardless presence mode exists precisely because the poorest own nothing, but it still depends on the merchant owning a smartphone, on biometric sensors of sufficient quality, and on sync points existing in the zone. Entire populations (conflict zones, remote areas) may remain outside all of these. Existenz reduces the connectivity requirement to the merchant's, but does not remove it from the system. The design's honest premise is that the minority without any phone is also, statistically, the minority in the most remote places, where the population density, and therefore the daily transaction volume per merchant, is small enough for the merchant's phone to carry the entire verification load; the corollary weakness is that this same remoteness makes sync points rare, so provisional windows stretch and audit latency grows. The day-chain trace and the opportunistic relay sync of E.11
mitigate this by letting any holder's phone carry the merchant's encrypted backlog toward connectivity, but they cannot abolish it: a zone with no passing device and no connectivity for months still has no conversion path, and the honest limit of E.11 is that it protects the merchant's claim, not their instant liquidity.

## 11.8 What melting cannot do

Melting guarantees circulation, not welfare. EXZ cannot buy a medicine that does not exist locally, a harvest failure, or a roof after a flood; the emergency fund and the BNB anchoring are the only instruments against these, and both are bounded. The design is a subsistence floor, not a development policy: it does not create jobs, infrastructure or peace, and pretending it substitutes for them would be the greatest dishonesty of all.

## 11.9 Residual gray markets and social tolerance

F.6 assumes the gray allocations market rather than eliminating it. The corresponding weakness: any cap-based bounding of merchant monetization creates pressure on exactly the honest merchants who operate in dense low-income areas, where volumes legitimately sit near plausibility caps. An honest merchant and a harvesting merchant can look statistically alike for weeks. False positives in the audit regime are not costless: they are delays, frozen conversions and reputational harm to people with no cushion. The 95th-percentile automatic review is designed to be a review, not a punishment, but the risk of adversarial bureaucracy is real and must be governed.

## 11.10 Technological maturity

The design stacks several technologies (zero-knowledge existence proofs, cancelable day-bound biometric embeddings, Chaumian offline credentials, VRF committee sampling, native demurrage ledgers, offline hash-chain timestamping, BLE epidemic data relay) that are individually established in research but have never been composed at planetary scale. Integration risk, energy and hardware costs of the ledger itself, and upgrade paths across protocol upgrades are open engineering questions, honestly listed as such.

## 11.11 Why state them anyway

Every one of these weaknesses is either bounded by an annex protocol, priced by an incentive design, or explicitly outside what a currency can solve. A project whose white paper claims to have no weaknesses forfeits the right to be believed about its strengths. The two-thirds community inherits not only the rules, but the limits; listing them is the first act of the governance it will have to exercise.

# 12. Conclusion

**The proposal in one line.** Existenz proposes that the recognition of a human existence can be encoded in a monetary protocol: equal issuance for every person, capped accumulation, melting that rewards circulation, and a global fund that recycles excess into emergency solidarity.

**What it is, and what it is not.** It is not a savings vehicle and does not aim to be one; it aims to be a helping hand toward daily basic needs for every human, anywhere.

**The founding rules are simple.** The hard problems are not: identity without surveillance; value without borders; melting without punishing the poor; subsistence without connectivity, without a phone and without any physical credential; honest merchant liquidity without fund abuse or chain fraud, even a year away from the network; a fund honestly bounded as a public service rather than disguised as a universal exchange; a parameterized bridge from subsistence to universal income; an account of credit outside the protocol that protects the issuance stream without pretending to abolish its gray market; and an honest account of the system's structural limits.

**How they are addressed.** These problems are addressed by the annex protocols and by Section 11 of this paper, whose parameters remain under the two-thirds governance of the very people the currency exists to serve.

# Annex A - Robust Human Uniquity Protocol (RHUP)

The Sybil problem, one account per living human, is the single point of failure of the entire system. This annex proposes a layered protocol that combines four mechanisms so that no single one is a trusted authority.

## A.1 Layer 1: Enrollment web of trust
- Enrollment is sponsored: a new account requires attestations from N existing account holders in good standing (proposed N = 5), geographically spread across at least 3 distinct zones.
- Sponsors stake a portion of their own issuance stream for a bonding period (proposed 180 days): if the sponsored identity is proven fraudulent, sponsors lose the stake and see their attestation reputation reduced.
- Reputation is public and slashing is automatic, so mass-fraud enrollment carries a real, escalating cost.

## A.2 Layer 2: Periodic liveness attestation
- Each account must renew a liveness attestation every 12 months: a zero-knowledge biometric freshness proof (proof that a living person is present, not identity disclosure), plus a geographic-zone claim.
- Missing two consecutive cycles suspends issuance (not seizure): the account freezes, and after a grace period the balance transfers to the global fund per the death-settlement rule.

## A.3 Layer 3: Zero-knowledge existence credentials
- The ledger stores no identity data. It stores only a nullifier commitment per person and a ZK proof that: (1) the person exists, (2) is enrolled exactly once, (3) is alive, (4) is in the claimed zone.
- Personal data never leaves the wallet; the chain verifies proofs, not identities. This structurally prevents the network from becoming a surveillance instrument.

## A.4 Layer 4: Continuous fraud detection
- Anomaly detection on the public graph: attestation clusters with unusual density, zone-hopping patterns, liveness timing correlations.
- Suspected duplicates trigger a challenge round: a random committee of validators (rotating, geographically constrained) re-verifies only the ZK proofs, never the underlying identity.
- False-positives are cheap to appeal (resubmit proofs); false-negatives are expensive to create (attack costs scale with stake and reputation).

## A.5 Residual risk statement

Collusion at nation-state scale against the web of trust remains theoretically possible. RHUP does not eliminate it; it prices it, detects it and makes it recoverable via slashing. This residual risk remains in the two-thirds governance scope: the community can raise N, the stake or the liveness frequency at any time.

# Annex B - Geographic Value Anchoring with Anti-Manipulation Design

The geographic value basis is novel and fragile. The proposal is to anchor EXZ to a transparent, manipulation-resistant Basic Needs Basket (BNB) index.

## B.1 The Basic Needs Basket index
- Each zone maintains a published BNB: the local price of a fixed basket (kcal/day, clean water, basic shelter, hygiene, telecom credit), collected by validators and cross-checked against open data (public market prices, humanitarian price databases).
- The index is public, versioned and auditable: anyone can reproduce it from published observations.

## B.2 Committee sampling resistant to collusion
- Zone index values are computed by a randomly sampled, rotating committee of validators drawn from the global set, with a hard constraint: no more than 1/3 of any committee may share a geographic or attestation cluster, and sampling is verifiable on-chain (VRF).
- Validators post stake; index manipulation detected in retrospective audit slashes the stake and bans the validator.
- The 10-validation emergency rule inherits this committee structure: emergency disbursement requires 10 signatures from at least 3 distinct clusters, sampled randomly at activation time so attackers cannot pre-position.

## B.3 Manipulation economics

An attacker must simultaneously: control a random committee, survive retrospective audit against public data sources, and absorb slashing. Publishing the basket composition and all observations makes retrospective audit reliable, which is the main deterrent. The system accepts slower index updates (weekly) in exchange for verifiability.

## B.4 Drift and transition

Zones with thin data default to a conservative regional reference basket rather than a volatile local one. Convergence toward local baskets is progressive as data quality improves, and every basket change is a two-thirds governance decision in the affected zone, not a validator unilateral act.

# Annex C - Melting Equity Layer

Melting penalizes saving for lumpy needs (medicine, repairs, seeds). Three mitigation mechanisms, all compatible with the founding rules:

## C.1 Melting exemption floor
- Balances below a floor (proposed: 14 days of issuance) melt at a reduced rate or not at all: the poorest holders are protected by design.
- The cap remains untouched: this is a floor protection, not an accumulation channel.

## C.2 Targeted savings sub-wallets
- A holder may freeze a bounded amount (proposed: up to 60 days of issuance) in labeled sub-wallets (health, education, repair) with melting suspended, subject to ZK eligibility checks and a maximum duration.
- This restores pre-planning for lumpy expenses without restoring hoarding: the limits keep total exposure inside the anti-accumulation logic.

## C.3 Explicit bridge to store-of-value instruments
- The protocol does not compete with savings vehicles: it documents that EXZ is a medium of subsistence and circulation, and that interop bridges (to fiat, to other assets) are expected to exist and are not the protocol's concern.
- This honesty clause prevents the temptation to relax melting rules under pressure, which would destroy the currency's purpose; any relaxation is anyway protected by the two-thirds rule.

# Annex D - Governance Matrix

The founding rules (equality, cap, melting, geographic basis) are split into three tiers:

- Constitutional rules: equal issuance, the very existence of a cap and of melting, the single-hop rule of Annex F, and universal suffrage of every verified living account owner, merchants included, one vote each, inalienable. Change requires two-thirds of all accounts plus a minimum turnout (proposed 50%).
- Parametric rules: melting rate, cap level, floor exemption, sub-wallet limits, zone basket composition, N and stake in RHUP, committee sizes, offline session limits, template retention window, tolling rules of E.11, merchant cap tiers, conversion commission rate, and the solidarity scale parameters of F.4 (class table sizes, quota headroom, overshoot melting rate). Change requires two-thirds of the affected scope (global or zone) with standard turnout.
- Emergency rules: global fund disbursement, frozen-account handling. Handled by the committee mechanisms of Annex B, not by vote.

This matrix prevents both drift of the founding intent and paralysis by over-democratizing parameter tuning.

# Annex E - Local Edge Verification Protocol (Offline Existence at the Point of Sale)

The target population is often precisely the population without reliable connectivity. This annex specifies two verification modes, sized on a realistic premise: roughly 70% of the target population owns a mobile phone, roughly 7 holders out of 10; the remaining minority owns nothing. For the majority, the device mode validates the holder through their own secure element and the blockchain. For the minority, the cardless presence mode makes the merchant's phone, from a photo and voice capture analyzed by an on-device algorithm, the entire verification infrastructure. The physical printed existence card, examined as a candidate artifact, is deliberately abandoned: it is falsifiable, it can be lost or stolen, it adds a fragile physical object to the protocol, and the population it would serve is precisely the population that would suffer most from its failure. No physical credential exists
in this protocol; a person is their own credential. Extended offline operation, the case of a merchant who stays away from any connectivity for months or even a full year, is secured by the day-chain trace and the opportunistic relay sync of E.11.

## E.1 Principle and the two modes

Device mode (roughly 70% of holders, the realistic baseline):

- At enrollment, the person receives a secure element: an eSIM, a smart card or the wallet app on a low-cost phone. The element holds the person's biometric template commitments (iris and outer ear proposed: both are unique, stable over decades, and capturable with an ordinary camera), their ZK existence credential from Annex A, and pre-issued signed offline spending credentials.
- The transaction is validated by cryptography: the element signs, the blockchain settles at the next sync. The biometric match happens on the element or the merchant's phone, but the security guarantee comes from the blockchain-anchored credential, not from the merchant's judgment.

Cardless presence mode (the minority without any device):

- The holder carries nothing and presents nothing. The merchant's phone, running the EXISTENZ app without any connection, captures a photo (outer ear or face) and a short voice phrase, computes day-bound embeddings on-device, and runs the verification algorithm of E.4: identification against the cached zone registry, duplicate detection against every embedding the zone has ever served, and statistical coherence checks. The person is their own credential; the passage is stored on the merchant's phone as a signed transcript until the next sync.

The merchant's phone never stores and never transmits biometric data, in either mode, beyond the strictly local, day-bound, encrypted intraday presence template defined in E.5.

## E.2 Device mode: offline transaction flow
- The person presents their element (NFC tap or QR code) to the merchant's phone.
- The merchant's phone captures an iris or outer-ear image locally, and performs on-device matching against the template commitment transmitted by the element.
- On match, the element signs an offline transaction: a pre-issued credential (nullifier) is handed to the merchant for the amount of the good, for example 1 EXZ for one sandwich.
- The merchant's phone verifies the signature offline against the pinned public issuance key, and releases the good. The captured image is destroyed immediately; only a cryptographic proof transcript is retained.
- When either device regains connectivity, transactions sync; the ledger consumes nullifiers, detects any reuse, and settles.

## E.3 Device mode: anti-double-spending design

Offline spending relies on pre-issued bearer credentials, in the spirit of Chaumian offline e-cash:

- Each credential carries a unique nullifier; the secure element enforces per-day and per-merchant session limits.
- A credential spent twice is detected at sync (nullifier reuse): the element is locked, the excess is recovered, and repeated abuse triggers the RHUP challenge round of Annex A.
- The merchant cannot forge credentials, cannot invent a customer (the credential must come from a person's element with a passing biometric match), and cannot alter amounts (the signature covers the amount and the zone).

## E.4 Cardless presence mode: the photo/voice/algorithm verification

The people this mode serves are, in reality, few and far between: communities deep in the Amazon, in the mountains of Central Asia, in the Australian bush, in remote desert and island populations. Statistically, the phoneless minority concentrates in low-density zones, which makes the design feasible where it is needed: a remote-zone merchant serves small numbers of distinct persons, so the zone commitment registry is small, and the merchant's phone can carry the entire identification load.

- At each sync, the merchant app caches the commitment registry of its zone: a compact cryptographic accumulator (Merkle structure) of the cancelable, salted biometric embedding commitments of all enrolled persons of the zone, with per-epoch diffs. In low-density zones this registry is small; even a dense zone fits in a few tens of megabytes, within reach of any smartphone.
- At the point of sale, the holder presents nothing. The merchant app captures an outer-ear or face photo and a short voice phrase (a spoken response to an unpredictable prompt, for example a number displayed on the screen), computes day-bound embeddings on-device, and destroys the raw captures immediately.
- Identification, not matching: without a card or a phone, the algorithm performs a one-to-many comparison of the fresh embeddings against the cached zone registry, returning either a single best candidate above the match threshold, or a rejection. Voice and photo are combined: photo (ear or face) carries stronger uniqueness, voice carries stronger liveness; requiring both keeps a printed photo or a recording from passing.
- Duplicate and coherence checks: before accepting, the algorithm compares the fresh embedding against the day's local vault (has this presence already been served today by this merchant?), against the registry (does this embedding match one and only one enrolled person?), and against every embedding recently served in the zone (is the same statistical presence appearing under different identifiers?). If a presence is unique, not duplicated, and statistically coherent with the zone's population, the transaction is accepted; otherwise it is refused and the incident is logged to the sync queue.
- On acceptance, the app emits a signed offline payment transcript binding: the identified public identifier, the amount, the zone, the registry epoch, the day epoch and a fresh transaction nonce. The passage is stored on the merchant's phone as part of the day's tamper-evident statement; at sync, the ledger settles, the holder's accrued issuance covers the spent amounts, and the merchant is credited.

Re-encounters within the day: the local vault matches the person in seconds from the day's first capture, with no registry lookup needed, and refuses a second daily allowance from the same merchant.

Unregistered person: if the embeddings match no registry entry, the transaction is refused. The person must enroll through the RHUP web of trust (Annex A) at any sync point; in a remote zone this happens through the very merchants and sponsors who already serve the zone, and the enrollment biometrics become that person's registry commitments. Enrollment without a device is therefore possible and produces no credential to carry: the person's biometric commitments in the zone registry are the credential.

Fabricated persons: a merchant cannot invent a customer, because every accepted transcript must bind to a real registry entry that the merchant's phone cannot modify (the accumulator is authenticated against the ledger at sync), and cross-zone duplicate detection at sync (the same embeddings appearing under two identifiers) flags identity lending and triggers the Annex A challenge round.

## E.5 Intraday Presence Template: immediate merchant security

The merchant must be certain at the moment of the exchange, not a week later at settlement. The intraday presence template (IPT) provides this certainty, from a single photo and voice capture per person per day, and it carries most of the verification load in the cardless presence mode.

Principle: at the first encounter of a person during a day, the merchant app captures a presence sample: an outer-ear or face photo plus a short voice phrase. The app computes on-device embeddings (mathematical fingerprints of the samples, not the samples themselves), binds them to the day epoch, and stores them encrypted in a local vault. The raw captures are destroyed immediately. The vault auto-purges after the audit window (proposed: 48 hours, i.e. the current day plus one sync cycle).

What the merchant gains immediately:

- Same-day re-recognition: if the same person returns the same day, the app matches their capture against the vault in seconds and refuses to grant a second daily allowance from the same merchant: no double-subsidy, no replay of the morning's sandwich.
- Same-day multi-identity detection: if two different persons produce the same ear, face or voice embedding, or if one embedding serves multiple identifiers, the app flags identity lending or duplication on the spot, refuses the transaction, and logs the incident to the sync queue. This is the strongest immediate anti-Sybil signal available offline, and in the cardless mode it is also the primary identification input.
- Cryptographic proof of delivery: each transcript binds the day epoch and the presence-embedding hash, so the merchant can prove, at settlement, that a unique living presence stood in front of them on that specific day. A replayed capture from yesterday fails the day-epoch check immediately, in the merchant's own hands.
- Known worst-case exposure: the daily allowance per person and the per-merchant offline cap (E.7) mean the merchant's maximum loss on unconfirmed transactions is a small, known-in-advance quantity, not an open-ended risk.

Why day-bound binding matters: a photo taken today is only a proof today. Templates and transcript hashes include the day epoch, so a stolen or copied capture cannot be replayed tomorrow, and the merchant's vault is automatically worthless after the audit window: there is nothing to steal that retains value.

Voice and photo are complementary modalities: photo (ear or face) carries stronger uniqueness, voice carries stronger liveness (a recording is defeated by the unpredictable spoken prompt). The app requires both in the cardless presence mode, and may escalate requirements after anomalies.

## E.6 Cardless presence mode: provisional settlement

In device mode, the person's secure element enforces limits and nullifiers cryptographically. In cardless mode, the merchant app alone records the transaction, so a dishonest merchant could fabricate transcripts for persons who never came, or inflate amounts. The design therefore makes every cardless offline payment provisional, with the intraday presence template and the registry identification carrying most of the detection load:

- Provisional credit: the merchant is credited only after settlement, and settlement confirms only transactions that pass the controls of E.7 and E.9.
- Confirmation requirement: a cardless payment finalizes when the identified holder re-attests within K days (proposed K = 30) at any sync point: a kiosk, a connected merchant terminal, or their annual RHUP liveness cycle. In remote zones, the merchant's own sync acts as the holder's re-attestation when the person passes again and re-matches, which is the common case in small communities. During a certified offline period of the merchant (E.11), the confirmation window is tolled, not expired: K days resume when connectivity or a relay reaches the ledger.
- Unconfirmed transactions revert: the provisional credit lapses and the incident counts against the merchant's integrity score.
- Cross-merchant dedup at sync: each transcript carries the day-bound, cancelable embedding hash of the presence. At sync, the ledger counts, per public identifier, the total offline spending across all merchants of the day, without ever learning who the person is: only salted hashes cross the boundary. A holder exceeding the daily allowance, or one identifier served by implausibly many merchants, is flagged to Annex A Layer 4.
- Template replay across days: because embedding hashes are day-bound, the same presence hash appearing on two different days is a merchant fabrication signal (a real person produces a fresh capture each day) and triggers slashing review.
- Registry replay across zones: the same cancelable embedding commitments appearing in two zones' registries is a Sybil signal at enrollment, caught by Annex A Layer 4, not by the merchant.

## E.7 Merchant abuse controls (both modes)

The merchant is the most powerful actor offline, so the protocol prices, caps and audits merchant power:

- Hard offline caps: per merchant, per sync cycle, the app enforces a maximum number of offline transactions and a maximum EXZ volume, set as a function of the merchant's attestation tier and of plausible local BNB demand (Annex B). Caps grow with demonstrated integrity, never with self-declaration alone.
- Attestation tiers with stake: casual acceptance is open to anyone, but meaningful offline limits and registry listing require a stake, slashed on confirmed fabrication. A slashed merchant falls back to minimal caps or exclusion.
- Retrospective audit: offline proof transcripts, including the day-bound presence embedding hashes, sync to the ledger, where Annex A Layer 4 runs anomaly detection: volumes implausible for the zone's BNB, timing correlations, repeated identifiers, one embedding serving multiple identifiers, or one identifier collecting an implausible number of distinct daily embeddings from the same merchant.
- Random holder challenges: a fraction p of offline transactions (proposed p = 5%) is randomly flagged for mandatory holder confirmation at the next sync; systematic failure to confirm is a strong fabrication signal and triggers slashing. In low-density zones where the same persons pass every week, challenge failure is unusually informative, so the challenge fraction adapts upward there.
- Diversity requirement: a single merchant cannot be the sole offline verification channel for the same holder indefinitely; concentration patterns are flagged and, beyond a threshold, the merchant's offline caps are reduced.
- No issuance to merchants: the founding principle is repeated here because it is the main structural protection. A merchant earns EXZ only against a real good delivered to a real, verified person. Fabrication creates claims against nothing, and every claim is auditable, now with a day-bound presence proof attached.

## E.8 Privacy guarantees
- No biometric image or voice recording ever leaves the capturing device: raw captures are destroyed immediately after embedding computation.
- The intraday presence template is encrypted at rest, local to the merchant phone, day-bound, and auto-purged after the audit window (proposed 48 hours).
- Only cancelable, salted, day-bound embedding hashes sync, solely for per-identifier daily allowance counting and duplicate detection: the ledger learns how many times a pseudonymous presence was served, never whose ear, face or voice it was, and the hashes are useless on any other day.
- The zone commitment registry contains commitments, not images or names: a stolen merchant phone leaks mathematically irreversible commitments, not a biometric database.
- The one-to-many identification of the cardless mode runs entirely on the merchant's device against commitments; no identification query crosses the network, and the network never learns who was identified, only that some valid pseudonymous identifier spent an allowance.
- Relay bundles of E.11 carry only signed transcripts, chain heads and salted hashes: a relay learns that an unknown merchant in an unknown zone has unclaimed statements, and nothing else.
- This extends the Annex A guarantee to the offline world: the merchant verifies existence, never identity, and even the merchant's own vault cannot be re-purposed beyond the day.

## E.9 Residual risks
- Sensor spoofing (printed iris or ear photos, recordings): mitigated by requiring photo plus voice in the cardless mode, the unpredictable spoken prompt, liveness cues (gaze tracking, angle variation), hardware attestation, session caps, day-bound embedding and the provisional-settlement regime bounding the maximum loss.
- One-to-many false matches: the identification step of E.4 is statistically weaker than one-to-one matching. A rare false match costs at most one daily allowance for one day, is detected at sync by cross-merchant counting, and the match threshold is raised automatically after any collision incident. In low-density zones, where the cardless mode actually operates, candidate sets are small, which keeps false-match probability negligible precisely where it matters.
- Template collision or drift: embeddings are day-bound and used only for same-day matching, dedup counting and one-to-many identification against zone registries, not for enrollment-grade identification, so a rare error costs at most one daily allowance, detected at sync.
- Stale registry: a merchant offline for a very long period identifies only against the registry epoch it has cached; persons enrolled after that epoch are refused offline service until the next sync, bounding exposure and, in remote zones, simply waiting for the sync that the merchant must do anyway. E.11 turns this waiting from a passive loss into a tolled, traceable period.
- Merchant collusion rings: several merchants validating each other's fabricated transactions are caught by cross-merchant embedding analysis, holder non-confirmation rates and volume plausibility audits; the cost of sustaining a ring grows with the stake of every participant. In the cardless mode, fabrication additionally requires inventing registry-valid identifiers, which cross-zone duplicate detection makes expensive.
- Relay abuse (E.11): a malicious relay can drop, delay or replay a bundle; dropping and delaying only harm the merchant's liquidity, not the validity of the claims; replay is defeated by bundle nonces and day-epoch checks; a relay cannot alter a bundle without breaking signatures. Relays are cheap to run and rewardless on purpose, so there is nothing to farm.
- The cardless presence mode is strictly less secure than the device mode; it is accepted as a deliberate trade-off so that the poorest holders, with no device and no credential of any kind, are first-class participants, with the intraday presence template, the registry identification and the provisional-settlement regime carrying the residual risk instead of the holder.

## E.10 Why this matters

This protocol turns the original presentation's intuition into the adoption engine of the whole system: any person able to sell a sandwich becomes, with an ordinary phone, a point of existence verification and of economic circulation, with no bank, no terminal, no connectivity, no surveillance, no requirement that the customer own any device or carry any credential at all, and same-day certainty for the merchant, secured by a single photo and a spoken phrase. Seven holders out of ten validate through their own secure element and the blockchain; the three others are recognized by what they are, not by what they carry.

## E.11 Extended offline operation: the day-chain trace and opportunistic relay sync

A merchant of a remote zone can stay away from any connectivity for months, or even a full year. The protocol must answer a hard question honestly: does that merchant lose the EXZ they earned? The answer is no, and the guarantee rests on two mechanisms: a daily regenerated cryptographic trace that proves what happened offline, and an opportunistic relay sync that lets any passing device carry the proof toward the ledger.

### E.11.1 The day-chain: a trace regenerated and sealed on the merchant's side, every day

At each sync, the merchant device receives from the ledger a signed checkpoint: the current epoch and a fresh chain seed, sealed in the device's secure element (TEE). From that moment, every day, even fully offline, the device regenerates the next link of a one-way hash chain: day_key(n) = H(day_key(n-1), day_epoch(n)), inside the secure element. The day's statement, its transcripts and its intraday template hashes are sealed under that day key. The chain is one-way: the device cannot skip a day, rewind a day, or backfill a missing day after the fact, because any forgery breaks the signature chain at the first verification, and the secure element refuses to sign for an epoch it has already passed. Clock tampering is bounded the same way: the element tolerates small drift but refuses large backward jumps, and the ledger cross-checks the chain length against the elapsed time between the
last sync and the reconnection.

What a year offline produces, therefore, is not a pile of unverifiable declarations but a single continuous proof: 365 sealed daily statements, each cryptographically bound to the previous one, each containing only signed transcripts and salted day-bound hashes. The trace proves elapsed time, statement continuity and authenticity of every recorded day; it does not prove the truth of each presence, which remains the job of the presence templates, registry identification and audit regime of E.4 to E.7. The day-chain makes the record unforgeable; the rest of Annex E makes it honest.

### E.11.2 Tolling: how the merchant keeps their EXZ after a year offline

Without this rule, the design would eat its own tail: the melting-exempt window of settlement credit (proposed 7 days, F.2) and the confirmation window K (proposed 30 days, E.6) would both expire long before a remote merchant could ever sync, punishing precisely the merchants who serve the most isolated populations. The protocol therefore tolls both windows during certified offline periods:

