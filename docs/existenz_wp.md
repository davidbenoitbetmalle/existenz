
# The Existenz White Paper

## A Fair Global Currency: Providing a Basic Human Right Through Virtual Currency

## Abstract / Executive Summary

The Existenz (EXZ) project proposes a fair global virtual currency designed to guarantee, through monetary design itself, a subsistence floor for every human being. Inspired by Article 3 of the Universal Declaration of Human Rights ("Everyone has the right to life, liberty and security of person"), Existenz distributes an equal amount of currency to every verified human existence, caps individual accumulation, and applies a controlled melting (demurrage) mechanism that recycles wealth into a global solidarity fund. This white paper formalizes the concept presented in the original Existenz presentation, develops its monetary mechanics, examines two distribution scenarios, addresses governance, identity and technical requirements, and provides an economic and risk analysis together with an implementation roadmap. Annexes A to G propose robust protocols for the structural weaknesses
identified: human uniqueness (Sybil resistance), geographic value anchoring, melting equity, fully offline existence verification at the point of sale for holders who own no phone and carry no physical credential, immediate intraday transaction security for the merchant, end-of-day settlement of merchant value into local currency under a strict single-hop rule, at a statistically bounded, abuse-resistant conversion rate, extended offline operation secured by a daily regenerated cryptographic trace and opportunistic peer-to-peer relay sync, and a quantitative solvency model of the global fund. The protocol's universal guarantee is goods-denominated, not currency-denominated: what is promised to every holder everywhere is the meal, while conversion into local currency is defined as a bounded, coverage-indexed public service of the global fund rather than an unconditional exchange. Annex H extends the design beyond the floor: the issuance rate as a governance parameter (the bridge from subsistence to universal income), a bounded merchant circuit that reduces conversion demand, and an honest treatment of credit and lending outside the protocol, including the inalienability of the issuance stream.

# 1. Introduction: Recognizing Human Existence

Modern monetary systems reward accumulation and speculation, while roughly a quarter of humanity lives below subsistence thresholds and more than a billion people remain financially excluded.

Existenz takes a different route: instead of taxing and redistributing within national borders, it embeds distribution into the currency protocol itself. Every verified human existence receives the same amount of a purpose-built virtual currency. Because the currency melts when hoarded and is capped per account, it functions as a medium of subsistence and circulation rather than a store of value. Each human existence is recognized, economically, by the network.

The core thesis of this paper is that a well-designed melting currency with equal issuance, an accumulation cap and geographic value adjustment can operationalize the right to subsist at global scale, without relying on any state budget.

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

The project also stands in the lineage of monetary thinkers such as Silvio Gesell (Freigeld, stampscrip), the WIR complementary currency in Switzerland, and contemporary universal basic income experiments. Existenz differs from a classic UBI in that it does not transfer existing national money; it issues a dedicated instrument whose properties prevent hoarding and encourage circulation, closer to a barter-like exchange medium than to savings.

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

- The caps of Annex F already ration conversion per merchant (physical, plausibility, tier), which slows any drain but does not create local-currency liquidity.
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

The entire offline verification layer (Annex E) and the entire conversion layer (Annex F) rest on small merchants finding their role profitable. If the commission, margins and conversion liquidity fail to compensate the operational burden (captures, statements, audits, stake), merchants do not adopt, and Existenz becomes a currency that exists but cannot be spent anywhere. Merchant adoption is the single point of failure of the project.

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

Existenz proposes that the recognition of a human existence can be encoded in a monetary protocol: equal issuance for every person, capped accumulation, melting that rewards circulation, and a global fund that recycles excess into emergency solidarity. It is not a savings vehicle and does not aim to be one; it aims to be a helping hand toward daily basic needs for every human, anywhere. The founding rules are simple; the hard problems, identity without surveillance, value without borders, melting without punishing the poor, subsistence without connectivity, without a phone and without any physical credential, honest merchant liquidity without fund abuse or chain fraud, even a year away from the network, a fund honestly bounded as a public service rather than disguised as a universal exchange, a parameterized bridge from subsistence to universal income, an account of credit outside the protocol that protects the issuance stream without pretending to abolish its gray market, and an honest account of the system's structural limits, are addressed by the annex protocols and by Section 11 of this paper, whose parameters remain under the two-thirds governance of
the very people the currency exists to serve.

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
- Parametric rules: melting rate, cap level, floor exemption, sub-wallet limits, zone basket composition, N and stake in RHUP, committee sizes, offline session limits, template retention window, tolling rules of E.11, merchant cap tiers, conversion commission rate and cap band parameters of Annex F. Change requires two-thirds of the affected scope (global or zone) with standard turnout.
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

- Certified offline period: the interval between the last successful sync and reconnection, covered by a valid, unbroken day-chain. Certification is automatic: the chain is either continuous or it is not.
- During the certified period: the melting exemption of the merchant's settlement credit is suspended (the clock stops, it does not run), the confirmation window K of cardless transactions is suspended, and the merchant's provisional credits accrue interest-free and melt-free. The merchant's own existence wallet keeps its ordinary melting (it is a personal wallet, not business income), bounded by the cap as always.
- Bounded exposure: the tolled amount per merchant is bounded by their offline caps (E.7): certified days x offline daily cap. A year offline cannot create an unbounded claim, because the claim was capped every single day it was earned.
- On reconnection: the day-chain is verified, the backlog settles within the caps of F.4, confirmed cardless transactions finalize, and the merchant's integrity index is untouched by the offline duration itself: remoteness is not a fault.

Honest limit, stated in the open: tolling protects the merchant's claim against the ledger. It cannot conjure local-currency liquidity in a zone with no connectivity: conversion still requires an agent, an agent still requires connectivity, and a merchant who never meets either never converts, no matter how valid their chain is. What E.11 guarantees is that when the merchant, or their data, finally reaches the network, everything they earned is still theirs.

### E.11.3 Opportunistic relay sync: every participant is a walking sync point

A remote merchant does not need to reach a connected terminal. They need to cross one person carrying the EXISTENZ app:

- Encounter sync over BLE: when the merchant's phone comes within range of any EXISTENZ app user (a holder with a device-mode element, another merchant, a kiosk operator), the two apps exchange over Bluetooth Low Energy. The merchant's outbound bundle is small (on the order of tens of kilobytes per day of offline backlog, capped): the day-chain head, the signed checkpoint from the last sync, the sealed statement backlog, and the merchant's status certificate, the compact signed document binding their public identifier, attestation tier and last certified epoch.
- Certificate copy, the status validation: the relay copies the merchant's certificate, and the bundle, to its own device. The relay's next sync, wherever connectivity exists, forwards everything to the ledger: the ledger validates the day-chain, marks the merchant's status as certified-current, settles the backlog within caps, and refreshes the registry epoch the merchant will receive at their own next real sync. From that moment, the merchant's status is validated network-wide, even though the merchant themselves is still a month's walk from the nearest antenna.
- Relay duties, deliberately rewardless: relaying is a background transfer of a signed, encrypted-to-the-ledger bundle; a relay cannot read it, alter it (signatures), or profit from it (no relay commission, deliberately, to remove any farming incentive). The cost to the relay is seconds of BLE time; the duty is social, like holding a door. Deletion or refusal by one relay is harmless: the bundle remains on the merchant's device and is offered to the next passer-by, gossip-style, with duplicates deduplicated by the ledger.
- Multi-hop tolerance: bundles carry a TTL and reconnection timestamps; the ledger accepts the earliest valid delivery and rejects stale replays via nonces and day epochs. A bundle may cross several relays over weeks before reaching the network; each hop adds delay, never risk.

### E.11.4 What this secures, and what it does not
- The merchant: a year offline does not destroy a year of earned income. The trace proves elapsed time and continuity; the relay validates their status; the fund settles on reconnection within the caps. The merchant's risk is bounded, knowable in advance, and unaffected by how far they are from the network.
- The network: extended offline merchants remain fully auditable. Statements cannot be forged after the fact, the day-chain proves exactly which days were recorded, and the ordinary audit regime (E.7) applies to the backlog at settlement like any other day.
- The honest remainder: E.11 protects claims, status and auditability. It does not provide live conversion, real-time caps updates or instant challenge rounds in a zone that sees no one; those still wait for the next encounter, human or radio. The design accepts this latency because the alternative, requiring connectivity for validity, would exclude exactly the populations the currency exists for.

# Annex F - Daily Merchant Settlement and Fair Conversion Protocol (Single-Hop Rule)

The merchant cannot eat EXZ: they must buy flour, oil and electricity in local currency, today or tomorrow. Yet the currency's purpose forbids both turning EXZ into a speculative off-ramp and letting it circulate into a value chain. This annex fixes the boundary of the system at exactly one economic hop.

## F.1 The Single-Hop Rule (N+1)

EXZ circulates exactly one hop: from a verified holder to a merchant, against a basic-need good. At the merchant, EXZ stops. There is no supplier chain, no B2B circuit, no second-hop spending of merchant earnings in EXZ. The merchant's earnings exit the system as local currency through conversion (F.3).

Protocol-level enforcement, not a policy: any EXZ received by a merchant account is automatically marked as settlement credit. Settlement credit is non-transferable: the ledger accepts exactly two operations on it, conversion to local currency at the anchored rate, and reversion to the global fund on failed confirmation. A wallet cannot send settlement credit to another account, because no such transaction type exists in the protocol.

Why the chain is forbidden, deliberately:

- Fraud surface: every additional hop is a new interface where volumes can be fabricated. One hop means one auditable interface, one set of presence proofs, one conversion point. The fraud problem becomes finite and inspectable instead of recursive.
- Anchoring coherence: the BNB rate is defined on basic-need goods. A flour wholesaler or a truck owner sells non-basket goods; letting EXZ price them silently floats a second, unanchored exchange rate, which reintroduces speculation the design just excluded.
- Incentive coherence: merchants are paid to verify existence, not to become a banking layer. A supplier chain would make large merchants de facto financial intermediaries, concentrating exactly the accumulation power the founding rules forbid.
- Simplicity for the poorest: a holder needs to understand exactly one thing: my existence buys basic goods at any merchant. A single hop keeps the mental model as simple as the sandwich.

The single deliberate exception is the bounded merchant circuit of H.2, where settlement credit may be spent on basic-need goods with other merchants within a capped tier. It is specified as an explicit, parametric relaxation, adopted for one arithmetic reason, reducing the fund's conversion obligation, not as a silent opening of the value chain.

## F.2 The merchant's three streams

A merchant is first a human being, and only second an economic actor. The protocol keeps three distinct streams:

- Existence wallet: the merchant's own daily issuance, subject to the same cap and melting as anyone else. It never mingles with business streams.
- Settlement credit: EXZ earned from confirmed sales, non-transferable by F.1, melting-exempt for a short operational window (proposed 7 days, tolled during certified offline periods per E.11.2), because the merchant must not be penalized for cash-flow latency they do not control. It may also be spent, before conversion, within the bounded merchant circuit of H.2.
- Conversion account: the local-currency counterpart of settled and convertible EXZ, held with a licensed conversion agent (F.3). This is where daily liquidity materializes.

## F.3 End-of-day declaration and the conversion channel

The declaration ritual is performed by the merchant app itself:

- At the end of the day, the app compiles the daily statement: number of unique validated presences (distinct intraday templates), transactions, EXZ received, EXZ still provisional, anomalies flagged.
- The statement is signed by the merchant and sealed under the day-chain key (E.11.1), then submitted at the next sync, directly or through a relay (E.11.3). It is tamper-evident: a Merkle summary of the day's transcripts, verifiable against the transcripts themselves. A merchant cannot under-declare (the ledger counts transcripts) or over-declare (transcripts are signed by the app, not editable).
- The declared and confirmed amount becomes convertible on the schedule of F.5.

Conversion into local currency is not a market. It is a public service at a public price:

- The conversion rate is the zone BNB rate of Annex B: 1 EXZ buys the same basic basket anywhere, so its local-currency value is exactly the local price of that basket, published weekly. The rate is the same for every merchant, every day, everywhere in the zone. Nothing to negotiate, nothing to speculate on.
- Conversion agents are licensed local actors (mobile money kiosks, microfinance counters, cooperatives) operating under local payment regulation, applying local KYC and tax rules to the merchant's income. The merchant receives the fruit of their work as ordinary, declarable local-currency revenue.
- Liquidity backstop: the global fund (Section 5.2) guarantees agents' conversion capacity within the caps of F.4. The fund is the buyer of last resort at the anchored rate; agents intermediate, take a regulated spread, and carry no exchange risk because the rate is fixed by index, not by market.

## F.4 The statistical cap band: paying fair value without allowing abuse

This is the core anti-abuse device, answering the physical limit question with a statistical ratio. A merchant's convertible volume is bounded by three nested caps; the binding one is the smallest.

Cap 1, the physical cap (per-person arithmetic): the daily convertible volume can never exceed U x M, where U is the number of unique validated presences of the day (distinct intraday templates) and M is the per-person daily allowance in the zone. A merchant who served 40 verified persons cannot claim more than 40 daily allowances worth of EXZ, whatever the goods' prices. This is the physical limit: it derives from verified presences, not from declarations.

Cap 2, the plausibility cap (zone statistics): the daily convertible volume cannot exceed P x M, where P is the plausible served population of the merchant's location, estimated from zone statistics: resident population within walking distance, beneficiary density, comparable merchants' median volumes, BNB demand structure. P is a published zone statistic, computed by the Annex B committee, so it is auditable and not an arbitrary punishment. A sandwich cart cannot plausibly serve ten thousand people a day.

Cap 3, the tier cap (integrity-indexed): each merchant has a daily conversion ceiling that starts modest (proposed: 100 daily allowances equivalent for a new attested merchant) and grows with the merchant's integrity index: an exponentially weighted moving average of confirmation rate, audit outcomes and challenge outcomes from Annexes A and E. Excellent merchants converge toward the plausibility cap; fresh merchants are bounded far below it. A slashed merchant falls back to the floor tier.

Settlement rule: Convertible(day) = min(confirmed EXZ, Cap 1, Cap 2, Cap 3), converted at the anchored BNB rate minus the agent spread. For a backlog settling after a certified offline period (E.11), the same rule applies day by day across the certified period, using each sealed day's statement separately: tolling never converts into a bulk claim that escapes the per-day caps.

## F.5 Fair compensation for the verification work

The merchant does real work for the network: they are, in the cardless mode, the verification infrastructure itself. Fair value means this work is paid explicitly, not hidden in the exchange rate:

- Verification commission: a protocol-paid commission, proposed at 3% of each confirmed transaction, paid from recycled melted units flowing through the global fund, not from the holder's allowance and not from an inflation of the money base. The sandwich buyer keeps their full 1 EXZ; the network pays its verifier.
- The commission is capped per day (proposed: 50 daily allowances equivalent) so that volume farming remains unattractive: beyond the cap, additional transactions pay the goods margin only, which is exactly the merchant's normal business.

This separates the two incomes cleanly: the goods margin pays the commerce; the commission pays the verification service. Abuse on one does not contaminate the other, and neither creates a reason to hold or relay EXZ beyond the single hop.

## F.6 Abuse economics, the gray market, and retrospective adjustment

The system does not trust the day; it reconciles it, and dishonesty is corrected rather than merely punished:

- The gray allocations market, assumed and bounded: a holder selling or pledging their daily allowance for cash, or a merchant buying allowances at a discount, is a phenomenon as old as rationing systems. No technology can prevent it, because biometrics proves presence, not freedom. The protocol does not attempt to police individual behavior, and says so honestly. Its only effective lever is on the beneficiary, the merchant: a merchant who buys allowances can still monetize them only within the physical cap (unique presences x daily allowance), the plausibility cap of the zone and their integrity tier. Industrial-scale harvesting is therefore economically pointless (bounded revenue, detectable concentration patterns, slashable stake), while small-scale gray trading is accepted as an incompressible social phenomenon, the same way every welfare system tolerates it. The constraint is where
it belongs: on the one who would profit, not on the person in need.
- Retrospective adjustment: conversions are advances against confirmation. If, at the end of the confirmation window (E.6, tolled per E.11.2 during certified offline periods), a fraction of a merchant's transactions is found fabricated, the corresponding converted amount is debited from future conversions, the integrity index drops (lowering Cap 3), and in serious cases the stake is slashed. The fund is never structurally drained: every converted EXZ is either confirmed or recovered.
- Statistical band review: a merchant whose daily conversion requests sit persistently above the zone's 95th percentile of comparable merchants triggers an automatic review, not a punishment: fast-track audit of transcripts and presence hashes. Honest outliers (a merchant at a market crossroads) are quickly cleared and their Cap 3 raised; dishonest outliers are caught in the first review.
- No black box: caps, rates and the merchant's own integrity index are all visible in the app, so the merchant always knows, in the morning, exactly how much they can convert in the evening and at what rate. Predictability is itself an anti-abuse device: a merchant with a fair, predictable income has no rational incentive to gamble it on fabrication.
- Single-hop auditability: because EXZ has exactly one hop, the audit surface per day is one statement per merchant, one set of presence hashes, one conversion request. The fraud problem is a closed, finite bookkeeping problem, not an open graph problem.

## F.7 Why same-day, capped, indexed conversion at exactly one hop is the right trade-off

Pure deferral would kill adoption (no merchant accepts a currency they cannot convert); a free float would kill the currency (EXZ becomes a speculative asset and the fund is arbitraged); a value chain would multiply fraud interfaces and float a second, unanchored exchange rate. The design keeps all three failure modes closed: intraday security comes from Annex E (presence templates and known exposure), liquidity comes same-day at a fixed indexed rate through licensed agents, integrity is reconciled over weeks through confirmation, statistics and stake, extended offline claims are preserved by the day-chain and relay mechanisms of E.11 rather than expiring in silence, and the currency's journey ends, by construction, at the merchant's conversion account. The merchant's risk is known in the morning, their income is fair, explicit and immediately usable in local currency, and the global
fund's exposure is always bounded by the physical arithmetic of verified presences.

# Annex G - Global Fund Solvency Model

This annex quantifies the weak point stated in Section 11. The model is deliberately simple, transparent and re-computable by anyone; its purpose is not prediction but order of magnitude, so that the coverage-ratio governance rule of 11.1 is a number, not a slogan.

## G.1 Parameters
- N: enrolled population.
- u: utilization rate, the fraction of enrolled persons who actually spend their daily allowance on a given day.
- v: conversion rate, the fraction of spent EXZ that reaches merchant settlement credit and is presented for conversion into local currency within the reserve horizon. The remainder is spent with merchants who hold part of their earnings past the melting-exempt window (those units melt back, causing no fiat outflow), or is absorbed by commission caps and agent spreads.
- p: the local-currency value of one EXZ, i.e. the zone BNB price of one basic meal. For global estimates this model uses p = 0.60 USD PPP per meal (the order of magnitude of humanitarian food-basket costing), with 2 meals issued per person per day (Scenario B).
- c: the verification commission (proposed 3%), which is also convertible and adds to the payout.
- R: the fund's external endowment, in local-currency equivalents (fiat or committed credit lines), the only resources that can honor conversion.
- H: the reserve horizon, the number of days of projected payout the governance requires the fund to hold at all times.

The daily fiat payout of the fund is then: D = N x u x v x 2p x (1 + c).

## G.2 Global scale: order-of-magnitude table

With N = 8.1 billion and 2 EXZ per person per day:

| Utilization u | Conversion v | Daily payout D | Yearly payout |
|---|---|---|---|
| 30% | 50% | ~1.5 B USD/day | ~550 B USD/yr |
| 30% | 80% | ~2.4 B USD/day | ~880 B USD/yr |
| 60% | 50% | ~3.0 B USD/day | ~1,100 B USD/yr |
| 60% | 80% | ~4.8 B USD/day | ~1,750 B USD/yr |
| 90% | 80% | ~7.2 B USD/day | ~2,630 B USD/yr |
| 90% | 100% | ~9.0 B USD/day | ~3,290 B USD/yr |

Reference points: total global humanitarian assistance is on the order of 55 B USD per year; total official development assistance is on the order of 220 B USD per year; world GDP is on the order of 105 T USD. At u = 60% and v = 80%, the fund's conversion obligation would be roughly 1.7% of world GDP, about thirty times all humanitarian aid combined. The conclusion of 11.1 is therefore not rhetorical but arithmetical: at planetary scale, full-utilization conversion cannot be an unconditional same-day guarantee. It must be a rationed, coverage-indexed public service, or the fund must be externally endowed at a scale no private actor can provide. Annex H.1 turns this conclusion into the definition of the guarantee, and H.2 reduces the demand it must serve.

## G.3 Reserve requirement and the coverage ratio rule

The endowment R covers only the buffer, not the flow: in steady state, whatever is paid out must be replenished by external inflows I, and solvency requires I >= D. The governance rule proposed for Annex D is then explicit and two-sided:

- Coverage rule: R >= H x D at all times, per zone and globally, with H a parametric rule (proposed initial H = 90 days). At global scale, u = 60%, v = 80%, this means a standing endowment on the order of 0.43 T USD.
- Inflow rule: conversion guarantees may only be extended to a zone as long as committed external inflows for that zone cover its projected steady-state payout; new zones are onboarded only when their H x D is endowment-covered (a rollout gate for Section 10, step 5).
- Publication: the coverage ratio R / (H x D) is published continuously on-chain; when it degrades below 1, conversion rations by transparent queue (elder merchants with confirmed transactions first), and the queue state is public, so rationing is governed in the open rather than discovered as failure.

## G.4 What the levers actually are

The model shows where the protocol's real degrees of freedom sit:

- v is the principal lever, and the design already compresses it: settlement credit melts past the operational window (unconverted units return to the commons instead of draining the fund), the commission cap bounds the commission component of v, and the three caps of F.4 bound per-merchant presentation. Every cap in Annex F is, monetarily, a limit on D.
- The merchant circuit of H.2 acts on v from the demand side: a capped fraction of settlement credit spent in kind on basic-need goods never reaches the conversion queue, so D falls without any cap being tightened. Its parameters (circuit fraction, eligible goods) are parametric rules under the two-thirds vote.
- p is anchored per zone by the BNB index: poorer zones have lower p, so the same subsistence claim costs the fund less where it matters most. Global average payout scales with the mix of enrolled zones, giving the roadmap a natural economic sequencing: early zones are also the cheapest per person served.
- u is the success variable: the model states plainly that every point of utilization success is a point of solvency stress, unless matched by endowment inflow. Existenz's measure of victory and its measure of solvency are the same number read in opposite directions.

## G.5 Zone-scale realism

The same arithmetic at rollout scale shows the model is not only a warning but a planning tool:

| Enrolled population N | Payout at u = 60%, v = 80% | Per person per year |
|---|---|---|
| 100,000 (pilot) | ~22 M USD/yr | ~220 USD |
| 10 million (region) | ~2.2 B USD/yr | ~220 USD |
| 100 million (large country) | ~22 B USD/yr | ~220 USD |
| 1 billion | ~220 B USD/yr | ~220 USD |

A pilot is fundable by ordinary philanthropic standards; a full region is a national social-program budget; the planetary scale is a wealth-transfer program of historic order, by construction. This is the honest meaning of a universal subsistence currency: the protocol can make the claim universal, but only the world's real resources, contributed through the fund, can make it universally honored in local currency. The design's job is to make that contribution visible, priced, capped and governed, which is exactly what Sections 5.2 and 11 and Annexes F and G together specify.

# Annex H - Beyond the Floor: Income, Credit, and the Outside Economy

This annex extends the design in three directions that the founding rules deliberately left implicit: what exactly the fund guarantees, how the mechanism scales from subsistence toward a universal income, and how credit can and cannot develop around a currency that is not a store of value.

## H.1 What the protocol actually guarantees: the meal, not the conversion

The five founding rules guarantee a goods-denominated floor: 2 EXZ buy a basic meal from any participating merchant, anywhere, at any time. This guarantee is universal, unconditional, and costs the fund nothing, because it is honored in goods by the merchants who choose to accept EXZ, under the presence verification of Annex E and the caps of Annex F.

Conversion into local currency is a different object, and the document states the difference as a definition rather than a caveat: conversion is a merchant service, backed by the global fund acting as buyer of last resort within a published, coverage-indexed ration (G.3). The ration is real: bounded by the operational-window demurrage of settlement credit (unconverted units return to the commons), by the per-merchant cap band of F.4, by the zone rollout gates of G.3, and reduced in volume by the merchant circuit of H.2. A merchant within the ration converts same-day at the anchored rate; a merchant beyond the ration queues transparently and is paid as the coverage ratio recovers.

This redefinition does not weaken the design; it prices it. The alternative, an unconditional conversion guarantee, is arithmetically refuted by Annex G. The honest formulation is: Existenz can guarantee the meal universally, and the meal is what a subsistence floor is.

## H.2 The bounded merchant circuit

The single-hop rule of Annex F forbids merchant earnings to circulate. One deliberate, bounded exception is specified here, adopted for one arithmetic reason: every EXZ a merchant spends in kind is an EXZ the fund does not have to buy back.

Design: a merchant may spend a capped fraction of their settlement credit (proposed initial parameter: 20% of weekly settlement credit) on basic-need goods purchased from other participating merchants, before conversion. The circuit is protocol-level: settlement credit spent in the circuit is consumed at the BNB-anchored value, generates a normal verified-presence transaction for the receiving merchant, and never leaves the settlement-credit state.

What it buys: a structural reduction of the conversion demand D of Annex G, borne by the merchant economy itself; a partial re-circulation of subsistence value inside the zone, which strengthens local adoption; and a liquidity path for zones where conversion agents are rare, since goods are available wherever merchants exist.

What it costs, stated honestly: it is a controlled leak in the single-hop rule. It reintroduces a bounded merchant-to-merchant interface, with its fraud surface (colluding merchants recycling fabricated settlement credit through each other); the exposure is bounded by the circuit fraction, by the ordinary presence verification required at each circuit purchase, and by the cross-merchant audit already specified in E.7 and F.6. It also adds a parameter to govern, and parameters are governance load. The circuit fraction is therefore a parametric rule of Annex D, set to zero or a low value at pilot start, and raised only against measured fraud data.

## H.3 Adjustable issuance: from subsistence floor to universal income

The issuance rate is a governance parameter, not a constant of nature. Two EXZ per day under the subsistence model is the level at which one EXZ buys one basic meal anywhere; nothing in the mechanism changes at a higher rate. The same ledger, the same identity layer, the same melting, the same single-hop economy operate identically whether the daily issuance buys one meal or three.

This makes the relationship between Existenz and a universal basic income explicit: the protocol is an income architecture whose funded level is currently subsistence. Raising the level is a two-thirds decision that the governance matrix correctly prices: by the rule of 11.2, guarantee levels may only be raised against demonstrated endowment of the fund, never on promise. The fund is thereby revealed as what it structurally is: the interface between the monetary protocol and the world's real resources. The income level the world can operate is exactly the level the world endows. Existenz does not legislate that level; it makes it legible, per person, per zone, at near-zero administrative cost.

## H.4 Credit outside the protocol, and the inalienable stream

A currency that flows daily to every human will attract lending, and an honest document must distinguish what the protocol can guarantee from what it must simply acknowledge.

The known failure mode: advance-selling. UBI pilots consistently document lenders advancing local currency against future transfers at usurious discounts, converting a subsistence floor into debt bondage. No protocol can prevent a person from pledging their future issuance verbally or on paper; biometrics proves presence, not freedom, exactly as in the gray allocations market of F.6.

What the protocol does guarantee, and why it matters:

- The issuance stream is inalienable and unseizable. No transfer, assignment or garnishment of future issuance exists as a protocol operation, mirroring the inalienability of the merchant vote in Annex D. A lender may be repaid voluntarily; the ledger offers no hook to seize, no account to attach, no future to lien.
- Melting punishes hoarded lending: a lender accumulating EXZ repayments watches them melt, so the currency itself is a poor collateral pool.
- The gray market is priced, not policed: identity-lending detection (A.4) and the merchant cap band bound industrial harvesting, while small-scale pledging remains, as in every welfare system, an incompressible social phenomenon outside the protocol's reach.

What the protocol can offer positively: a verifiable income registry. The ledger proves, with zero knowledge of identity, that a pseudonymous account receives a regular issuance stream in a given zone. An outside lender, a bank, a cooperative, a microfinance institution, may lend local currency against that verified stream under local law, exactly as payroll lending works today. The protocol's boundaries in that arrangement are explicit and non-negotiable: the ledger records, it does not enforce; the loan is local law's business, denominated in local currency; and the issuance stream itself remains unseizable, so a defaulting borrower faces the lender's ordinary remedies, never confiscation of subsistence.

## H.5 A licensed lending tier (explicitly not specified)

A future extension would be verified lenders operating inside the protocol, converting at market rates under caps, handling defaults under licensed local law. This annex deliberately does not specify it: a lending tier creates exactly the financial intermediary class the founding rules were written to prevent, and its design (licensing, caps, default handling, interaction with the integrity index) deserves its own annex, written after the subsistence economy of Annexes E and F has been challenged and measured. It is recorded here as future work, so that the omission is a decision, not an oversight.

# References and Lineage
- Universal Declaration of Human Rights, Articles 3, 22 and 25.
- Silvio Gesell, The Natural Economic Order (Freigeld and stampscrip theory).
- WIR Bank complementary currency system, Switzerland.
- David Chaum, blind signature offline e-cash (basis of the pre-issued credential design).
- Cancelable biometrics literature (basis of the salted, day-bound embedding hashes).
- Local exchange trading systems and complementary currency conversion practice (basis of the licensed agent and anchored rate design).
- Offline hash-chain timestamping and delay-tolerant networking (basis of the day-chain trace and opportunistic relay sync of Annex E.11).
- Universal basic income literature and pilot experiments.
- UBI pilot findings on debt, advance-selling of future transfers and garnishment practice (basis of H.4).
- Public credit registry and payroll-lending practice (basis of the outside-protocol lending model of H.4).
- Humanitarian food-basket costing practice (basis of the p = 0.60 USD PPP per meal estimate of Annex G).
- The original Existenz presentation, "Existenz: a fair global currency".
