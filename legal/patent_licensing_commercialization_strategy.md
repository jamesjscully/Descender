# Patent, Licensing, and Commercialization Strategy

Status: working strategy
Date: 2026-05-22

This document defines a concrete strategy for patenting, licensing, releasing,
and commercializing cryptographic primitives and supporting software developed
in the Descender project.

This is not legal advice. The plan is intended to make the business, patent,
standards, and release posture explicit before external publication or partner
discussions. Any actual patent application, public pledge, license agreement,
trademark program, export-control position, customer warranty, or indemnity
commitment should be reviewed by counsel.

## Executive Summary

The strategic goal is to make Descender primitives widely adopted while
capturing value from institutional certainty.

The core policy is:

> Make the standard profile free to adopt. Sell certainty, certification,
> indemnity, proprietary extensions, and high-liability deployment support.

This deliberately avoids the failure mode where a patent blocks adoption of a
technically superior primitive. The patent portfolio should be used to support
standards adoption, defensive leverage, enterprise confidence, and premium
commercial channels, not to tax ordinary interoperable use.

## Target Assets

The plan covers any Descender-originated asset that may have technical,
commercial, or defensive value:

- Signature primitives
- KEM primitives
- Commitment schemes
- Identification protocols
- Fiat-Shamir transcript designs
- Public-view/witness compression techniques
- Leakage/soundness proof techniques
- Setup and parameter-generation methods
- Verification acceleration techniques
- Batch verification or aggregation techniques
- Threshold, multisignature, adaptor, or composable variants
- Constant-time reference implementations
- Optimized production implementations
- Test vectors, conformance suites, and fuzzing harnesses
- Formal models and Lean proof artifacts
- Parameter-search tools and validation pipelines
- Deployment profiles for wallets, ledgers, HSMs, identity, CA, blockchain, or
  government systems

## Commercial Thesis

Cryptographic primitives rarely monetize best through a simple tax on
implementation. They monetize best when they become infrastructure.

The project should therefore optimize for:

- standards-body comfort
- open-source adoption
- independent implementation
- ecosystem trust
- conformance clarity
- security-review credibility
- high-assurance deployment support
- premium certification and indemnity

The patent estate should create optionality and leverage. It should not make
browser vendors, Linux distributions, blockchain clients, academic teams, or
standards bodies choose a weaker alternative to avoid licensing uncertainty.

## Invention Capture Policy

Every candidate primitive or materially novel mechanism should receive an
internal invention disclosure before public release.

Each invention disclosure should record:

- working title
- short non-confidential summary
- detailed technical description
- problem solved
- known prior art
- novelty claim
- security claim
- threat model
- mathematical construction
- protocol flow
- parameter families
- implementation constraints
- variants and fallback designs
- performance advantages
- deployment use cases
- known limitations
- names of human contributors
- date of conception
- date reduced to practice
- links to supporting notes, code, proofs, benchmarks, and experiments

AI-assisted ideation should be documented carefully. The record should identify
the human contribution to conception, selection, refinement, proof strategy, and
implementation. Do not list an AI system as an inventor.

## Disclosure Control

Default rule:

> If patent rights may matter, file before public disclosure.

Public disclosure includes:

- public GitHub commits
- public preprints
- conference talks
- standards-list posts
- public issue comments
- mailing-list explanations
- public benchmark reports
- published reference code that enables practice of the invention
- investor decks distributed without confidentiality controls

Private disclosure should use:

- NDA where practical
- invention disclosure records
- restricted access review packets
- dated technical memos
- clear labels such as "confidential pre-filing draft"

U.S. law may provide a limited grace period for inventor-originated disclosure,
but many foreign rights can be impaired by disclosure before filing. The working
policy is to preserve optionality by filing first.

## Patent Filing Strategy

### Filing Tiers

Use three filing tiers.

Tier 0: no filing

- routine implementation detail
- obvious engineering optimization
- poor novelty confidence
- low commercial importance
- better kept as trade secret or simply released

Tier 1: provisional filing

- plausible novelty
- strategic uncertainty
- public release likely soon
- standards relevance
- useful defensive value
- supports fundraising, partner discussions, or publication

Tier 2: full prosecution

- high novelty confidence
- central primitive or construction
- broad deployment relevance
- clear implementation path
- commercial licensing value
- defensive importance against platform incumbents

### Provisional Applications

Use provisional applications to establish an early priority date while the
technical and market case matures.

Each provisional should include:

- full technical disclosure
- diagrams or protocol flows
- pseudocode
- concrete parameter examples
- implementation details
- variants
- failure cases
- security rationale
- performance comparison
- use cases
- claim-drafting notes for counsel

Do not file thin provisionals that merely name the idea. The provisional should
enable a skilled implementer to practice the invention and should include enough
variant coverage to avoid being boxed into one embodiment.

### Nonprovisional Conversion

Within 9 months of a provisional filing, hold a conversion review.

Decision inputs:

- novelty search update
- external cryptanalysis status
- implementation maturity
- benchmark data
- market pull
- standards opportunity
- partner interest
- expected prosecution cost
- expected licensing leverage
- defensive value

At 10 months, decide whether to:

- convert to U.S. nonprovisional only
- file PCT for international option value
- abandon
- refile improved disclosure where legally appropriate
- split into multiple applications

Do not wait until the final week of the 12-month deadline.

### International Strategy

Use PCT only for filings with serious commercial or defensive value.

Potential national-phase jurisdictions:

- United States
- European Patent Office
- United Kingdom
- Japan
- South Korea
- Canada
- Australia
- Israel
- Singapore

China should be considered if manufacturing, hardware, or standards leverage
requires it, but cost, enforcement strategy, and disclosure tradeoffs should be
reviewed carefully.

### Claiming Strategy

For cryptographic primitives, claims should be drafted around concrete
computer-implemented systems and methods, not abstract math alone.

Potential claim surfaces:

- key-generation method
- signature-generation method
- verification method
- commitment method
- challenge derivation method
- transcript encoding method
- public-view compression method
- witness recovery or local recovery method
- parameter-generation method
- batch verification method
- rejection-sampling or leakage-control method
- constant-time implementation method
- hardened deployment system
- hardware security module profile
- wallet or blockchain validation profile
- certificate or identity infrastructure profile

The strongest filings should include:

- method claims
- system claims
- computer-readable medium claims
- protocol-profile claims
- dependent claims covering parameter families
- dependent claims covering implementation hardening
- dependent claims covering aggregation or composability

## Prior-Art and Freedom-to-Operate Program

Before major filing or release decisions, run two separate reviews.

Prior-art review asks:

- Is this actually novel?
- What publications, patents, standards, or code are closest?
- Which parts are defensible?
- Which parts should not be claimed?

Freedom-to-operate review asks:

- Can we ship this without infringing others?
- Are there blocking patents?
- Are there standards-essential patent commitments?
- Are there implementation dependencies with restrictive licenses?
- Are there third-party code, proof, or test-vector provenance issues?

Maintain a living prior-art matrix for each primitive.

Matrix columns:

- reference
- date
- authors/assignee
- technical overlap
- differences
- risk level
- claim impact
- license or patent status
- notes

## Release Architecture

Use a layered release model.

### Public Repository

Recommended public repo structure:

```text
LICENSE
NOTICE
PATENTS.md
README.md
SECURITY.md
SPEC.md
CONTRIBUTING.md
CODEOWNERS
reference/
optimized/
bindings/
test-vectors/
conformance/
fuzz/
formal/
benchmarks/
audits/
certification/
```

### Copyright Licenses

Default:

- code: Apache-2.0
- tests: Apache-2.0
- benchmarks: Apache-2.0
- formal models: Apache-2.0
- test vectors: Apache-2.0 unless there is a specific reason to use CC0
- standalone spec document: Apache-2.0 or CC-BY-4.0

Apache-2.0 is the default because it is permissive and includes an explicit
contributor patent grant.

Avoid using GPL as the main license for core primitives. Copyleft can be useful
for some projects, but cryptographic primitives should be easy to adopt in
operating systems, browsers, firmware, HSMs, wallets, embedded devices,
enterprise security products, and closed compliance environments.

Avoid MIT as the main license for patent-sensitive cryptographic code unless a
separate patent grant is extremely clear. MIT is simple, but too thin for this
case.

### Patent Pledge

Every public release of a patented or patent-pending standard profile should
include a separate `PATENTS.md`.

The intended policy:

> The patent holder grants a worldwide, royalty-free, irrevocable license for
> standards-conforming implementations, subject to defensive termination.

The public patent pledge should cover:

- making implementations
- using implementations
- selling products containing conforming implementations
- offering for sale
- importing
- distributing
- reproducing
- sublicensing as necessary for downstream distribution

The public patent pledge should not cover:

- non-standard proprietary variants
- incompatible forks marketed as the same standard
- use of trademarks or certification marks
- claims of audit approval
- claims of formal validation
- indemnity
- warranties
- commercial support
- private implementation reviews
- proprietary deployment profiles

Draft counsel-reviewed language before publication.

### Defensive Termination

The public patent license should terminate for parties that initiate patent
claims against conforming implementations of the same standard.

The goal is to create a patent peace zone:

- everyone can implement the standard
- participants cannot use their own patents to attack conforming implementers
- the project keeps leverage against hostile patent assertion

### Trademark Reservation

Keep names, logos, and certification marks separate from the code license.

Possible marks:

- Descender
- Descender Signature
- Descender KEM
- Descender Certified
- Descender Conformant
- Descender Hardened

The public can implement the standard, but cannot claim certification or use
reserved marks except under trademark rules.

## Commercial License Architecture

Use separate commercial agreements for broader rights and paid assurance.

### Free Public Adoption License

Audience:

- open-source libraries
- academic implementations
- standards bodies
- browsers
- operating systems
- blockchain clients
- ordinary commercial products using standard profiles
- interoperability tests

Rights:

- royalty-free patent license for conforming standard profiles
- no trademark rights
- no warranty
- no indemnity
- no support obligation

### Commercial Patent License

Audience:

- proprietary variant developers
- security vendors
- HSM vendors
- smart-card vendors
- hardware wallet vendors
- payment networks
- banks
- certificate authorities
- identity providers
- defense contractors
- government integrators
- blockchain infrastructure vendors with high-liability needs

Paid rights may include:

- non-standard extensions
- proprietary profiles
- closed validation suites
- sublicensing rights
- indemnity
- patent non-assert beyond public profile
- contractual warranties
- regulatory support
- procurement documentation
- implementation review

Pricing model:

| Tier | Target | Indicative Price |
| --- | --- | --- |
| Community | Open source, academic, standards | Free |
| Startup | Small commercial product | $5k-$25k/year |
| Standard Commercial | Enterprise software/security vendors | $50k-$250k/year |
| Hardware | HSM, secure element, smart card, hardware wallet | $100k-$1M/year |
| High Liability | Banking, CA, government, payment, national identity | Custom |

Pricing principles:

- do not charge per signature
- do not charge ordinary open-source distribution
- avoid per-user friction for standard software
- charge for risk transfer, proprietary scope, certification, and support
- reserve custom pricing for high-liability environments

### Certification Agreement

Certification is separate from patent rights.

Certification may cover:

- conformance to the spec
- serialization and transcript correctness
- test-vector compliance
- deterministic or randomized nonce requirements
- constant-time behavior under a stated model
- side-channel review
- fault-injection review
- memory-safety review
- parameter validation
- key lifecycle behavior
- misuse-resistance properties
- integration into target protocol
- version pinning
- renewal requirements

Certification deliverables:

- certificate ID
- product/version scope
- tested platform scope
- conformance report
- security report
- renewal date
- allowed mark usage
- public registry entry, if desired

Certification should be paid and renewable.

### Audit and Assurance Services

Paid services:

- implementation audit
- constant-time review
- formal verification assistance
- protocol integration review
- deployment threat modeling
- side-channel hardening
- HSM integration
- wallet integration
- blockchain consensus integration
- CA or identity profile review
- compliance support
- procurement support
- incident response

This should become a primary revenue line because it scales with trust and
deployment risk rather than with ordinary adoption.

## Standards Strategy

The standard profile should be free to implement.

Standards targets may include:

- IETF
- CFRG
- NIST
- ISO/IEC
- FIDO
- W3C
- Linux Foundation projects
- blockchain standards groups
- wallet and hardware-wallet ecosystems
- CA/Browser Forum, if certificate use becomes relevant

Standards package should include:

- clean specification
- security proof or proof sketch
- reference implementation
- optimized implementation
- test vectors
- conformance suite
- parameter rationale
- implementation guidance
- side-channel guidance
- patent pledge
- disclosure of patent applications and licensing terms

Standards-facing posture:

- royalty-free for conforming implementations
- defensive termination only
- no hidden licensing fees
- clear trademark separation
- clear security status
- clear review history

## Open-Source Strategy

The public implementation should be good enough to earn trust but carefully
scoped in its claims.

Use precise language:

- "reference implementation"
- "designed to avoid secret-dependent branches and memory access"
- "tested under the platforms listed in audits/"
- "not a universal guarantee of constant-time execution"
- "production suitability depends on compiler, CPU, runtime, integration, and
  threat model"

Avoid broad claims:

- "unbreakable"
- "constant-time on all platforms"
- "post-quantum secure" without exact assumptions and security levels
- "audited" without identifying audit scope
- "certified" unless the product has passed the certification program

## Contributor and Ownership Policy

Before public contributions are accepted, decide on:

- DCO or CLA
- inbound equals outbound licensing
- patent grant from contributors
- code provenance rules
- generated-code policy
- AI-assisted contribution policy
- contractor assignment agreements
- employee invention assignment

Recommended default:

- Apache-2.0 inbound/outbound for code
- DCO for casual contributions
- CLA for substantial external contributors if patent portfolio control is
  important
- written contractor IP assignment for paid work
- clear prohibition on knowingly submitting third-party encumbered code

## Export-Control and Compliance Posture

Cryptographic source releases may trigger export-control analysis.

Before public release of production-quality code:

- classify the release type
- determine whether source is publicly available
- determine whether notification is needed
- keep records of publication and notifications
- avoid shipping restricted binaries to sanctioned parties
- avoid customer-specific export promises without counsel

This should not block open research by default, but it should be handled
deliberately for commercial releases.

## Trade Secret Policy

Most core primitive details should not be kept as trade secrets if the goal is
standardization and review. Cryptographic trust usually requires public scrutiny.

Potential trade-secret candidates:

- customer-specific hardening recipes
- unreleased performance optimizations
- certification test infrastructure
- attack harnesses not ready for publication
- partner integration details
- pricing and sales process
- private patent-claim strategy

Avoid relying on trade secrecy for the security of the primitive.

## Public Communication Policy

Every public statement should be accurate about maturity.

Use stage labels:

- research candidate
- experimental implementation
- review candidate
- reference implementation
- production candidate
- certified implementation
- deployment profile

Do not let marketing outrun review status.

Public release announcements should include:

- what is being released
- what is not being claimed
- patent pledge summary
- license summary
- security status
- audit status
- how to report vulnerabilities
- how to request certification or commercial terms

## Revenue Channels

Primary revenue channels:

- certification fees
- implementation audits
- commercial patent licenses for non-standard/proprietary use
- indemnified enterprise licenses
- HSM/hardware-wallet integration
- banking/government/CA deployment support
- custom hardened implementations
- formal verification packages
- compliance and procurement support
- training

Secondary revenue channels:

- sponsored development
- standards participation retainers
- support subscriptions
- managed test-vector/conformance services
- bug bounty coordination
- incident response

Avoid making ordinary standards-conforming software use a primary revenue
channel. That would reduce adoption.

## Product Packaging

Potential commercial offerings:

### Descender Community

- public spec
- public code
- public test vectors
- public issue tracker
- no warranty
- no support SLA
- no certification mark

### Descender Conformance

- private conformance run
- report
- compatibility guidance
- no security certification
- limited mark usage if desired

### Descender Certified

- conformance plus security review
- product/version-specific certification
- public registry option
- mark usage
- annual renewal

### Descender Enterprise

- certification
- support SLA
- integration review
- security advisory access
- commercial license
- optional indemnity

### Descender High Assurance

- hardware or regulated deployment
- side-channel review
- fault review
- formal methods package
- procurement documentation
- custom contract
- optional indemnity

## Risk Register

| Risk | Impact | Mitigation |
| --- | --- | --- |
| Public disclosure before filing | Loss of foreign rights | File first; control disclosure |
| Patent blocks adoption | Ecosystem routes around primitive | Royalty-free standard pledge |
| Weak patent claims | Low licensing leverage | Prior-art review; counsel-drafted claims |
| Overbroad security claims | Liability and reputational loss | Precise maturity labels |
| Contributor patent contamination | Licensing uncertainty | Apache-2.0, DCO/CLA, provenance checks |
| Export-control mistake | Release or sales friction | Classification and notification review |
| Certification mark misuse | Brand dilution | Trademark rules and enforcement |
| Standards-body mistrust | Adoption failure | Publish clear RF terms and review artifacts |
| Patent aggression backlash | Community rejection | Defensive posture; avoid taxing commons |
| Unreviewed implementation shipped | Security incident | staged releases and certification gates |

## Default Decision Rules

File a provisional when:

- the idea appears novel
- public disclosure is likely
- the primitive or construction could become central
- a standard profile may emerge
- the implementation has commercial deployment paths
- blocking others from patenting the same space matters

Do not file when:

- the idea is clearly prior art
- the implementation detail is routine
- the filing would distract from stronger assets
- publication speed matters more than patent value
- defensive publication is sufficient

Release royalty-free when:

- the implementation conforms to the standard profile
- adoption increases network value
- the user does not need indemnity, certification, or proprietary scope

Charge when:

- the customer wants certification
- the customer wants indemnity
- the customer wants non-standard/private variants
- the deployment is high liability
- the customer needs procurement documentation
- the customer needs custom engineering or audit support

## Near-Term Recommendation

For Descender, the immediate next step is to create an invention capture packet
for each major primitive or mechanism currently represented in the Lean,
research, and reference-code trees.

Initial candidates:

- Descender signature construction
- public-view/witness compression
- committed sigma protocol flow
- quotient/link commitment mechanics
- local delta byte commitment mechanics
- leakage certificate methodology
- parameter-search and validation pipeline
- constant-time reference implementation architecture

Each candidate should be classified as:

- publish freely without filing
- file provisional before publication
- keep internal pending more analysis
- defensive publication only

