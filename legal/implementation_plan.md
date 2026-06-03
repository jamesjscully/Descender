# Implementation Plan: Patent, Licensing, and Commercialization Track

Status: active checklist
Date: 2026-05-22

This plan turns the legal and commercialization strategy into concrete work.
Check boxes should be updated as tasks are completed. Use links to artifacts
where possible.

This plan is not legal advice. Anything that becomes externally binding should
be reviewed by qualified counsel before use.

## Milestone Overview

- [ ] M0: Set up operating structure and document control
- [ ] M1: Inventory inventions and public-disclosure risk
- [ ] M2: Prior-art and freedom-to-operate review
- [ ] M3: Provisional filing packet for first patentable assets
- [ ] M4: Public release architecture and repo policy
- [ ] M5: Patent pledge and commercial license drafts
- [ ] M6: Certification and trademark program design
- [ ] M7: Standards and ecosystem adoption package
- [ ] M8: Commercial packaging and sales collateral
- [ ] M9: Governance, compliance, and recurring review loop

## M0: Operating Structure and Document Control

Goal: create a controlled non-technical workflow so invention capture,
licensing, disclosure, and commercialization decisions are deliberate.

### Directory and Document Setup

- [x] Create `legal/` directory
- [x] Create strategy document
- [x] Create implementation checklist
- [x] Create `legal/README.md` index
- [ ] Add document owners
- [ ] Add review cadence
- [ ] Add version history section to each externally relevant document
- [ ] Decide whether `legal/` remains public or should be moved to a private
      planning repository before sensitive details are added

### Confidentiality Discipline

- [ ] Define what belongs in public repo versus private counsel folder
- [ ] Create private folder or secure workspace for invention disclosures
- [ ] Label pre-filing materials as confidential
- [ ] Define public-disclosure approval process
- [ ] Define who can approve public technical publication
- [ ] Define who can approve partner/investor disclosure
- [ ] Create NDA template request process through counsel

### Entity and Ownership

- [ ] Decide whether IP is held personally, by a company, or by a dedicated IP
      holding entity
- [ ] Confirm assignment path for existing work
- [ ] Confirm assignment path for future contributors
- [ ] Confirm contractor IP assignment template
- [ ] Confirm employment or founder invention assignment if applicable
- [ ] Create list of contributors and contribution dates
- [ ] Identify any third-party code, proof, or dataset dependencies

### Counsel and Advisors

- [ ] Identify patent counsel with cryptography/software experience
- [ ] Identify open-source licensing counsel
- [ ] Identify export-control counsel or advisor
- [ ] Identify trademark counsel or general IP counsel
- [ ] Identify one external cryptographer for pre-filing technical sanity review
- [ ] Identify one commercial advisor familiar with cryptography infrastructure

## M1: Invention Inventory and Disclosure Risk

Goal: identify what may be patentable, what has already been disclosed, and
what should be protected before further publication.

### Repo and Research Inventory

- [ ] Inventory `Descender/*.lean` files for primitive-level mechanisms
- [ ] Inventory `research/cryptanalysis/` for invention-relevant memos
- [ ] Inventory `research/parameter_search/` for parameter-generation or
      validation methods
- [ ] Inventory `tools/production-reference/`
- [ ] Inventory `tools/parameter-search/`
- [ ] Inventory `scripts/research/`
- [ ] Inventory public-facing documentation such as `research_program.md` and
      `lean_proof_plan.md`
- [ ] Record which artifacts are already public in git history
- [ ] Record which artifacts are local-only or unpublished

### Candidate Invention List

Create one disclosure stub per candidate.

- [ ] Descender signature construction
- [ ] Descender KEM profile, if commercially separate
- [ ] committed sigma protocol construction
- [ ] public-view/witness compression
- [ ] quotient/link commitment mechanism
- [ ] local delta byte commitment mechanism
- [ ] hidden committed quotient link mechanism
- [ ] locally recoverable syndrome method
- [ ] information-set commitment method
- [ ] leakage certificate method
- [ ] finite-support or conditioned distribution proof technique
- [ ] diagonal amplification technique
- [ ] block challenge soundness technique
- [ ] self-randomization method
- [ ] parameter-search and validation pipeline
- [ ] constant-time reference architecture
- [ ] blockchain deployment profile
- [ ] HSM or hardware-wallet deployment profile

For each stub, record:

- [ ] title
- [ ] contributor list
- [ ] conception date estimate
- [ ] first reduction-to-practice date estimate
- [ ] links to code/proofs/memos
- [ ] one-paragraph summary
- [ ] novelty hypothesis
- [ ] closest known prior art
- [ ] current disclosure status
- [ ] commercial relevance
- [ ] recommended action

### Disclosure Risk Classification

Classify every candidate:

- [ ] A: file before any more public disclosure
- [ ] B: already disclosed; evaluate U.S. grace-period and foreign impact
- [ ] C: publish as defensive publication
- [ ] D: release freely without patent action
- [ ] E: keep internal pending review

### Immediate Hold List

Create a hold list for items that should not be publicly explained further until
counsel review.

- [ ] Identify sensitive unpublished details
- [ ] Identify public docs that need wording changes before wider sharing
- [ ] Identify conference/post/preprint plans that should wait for filing
- [ ] Identify partner decks or diagrams that need confidentiality labels

## M2: Prior-Art and Freedom-to-Operate Review

Goal: avoid filing weak patents and avoid shipping into known blocking rights.

### Prior-Art Search

For each Tier 1 or Tier 2 candidate:

- [ ] Search academic papers
- [ ] Search ePrint
- [ ] Search IACR publications
- [ ] Search NIST PQC materials
- [ ] Search IETF/CFRG drafts
- [ ] Search ISO/IEC references where accessible
- [ ] Search GitHub/code implementations
- [ ] Search Google Patents
- [ ] Search USPTO patents and applications
- [ ] Search EPO/Espacenet
- [ ] Search WIPO Patentscope
- [ ] Search known schemes: Schnorr, Fiat-Shamir, Stern, Lyubashevsky, lattice
      signatures, code-based signatures, MPC-in-the-head, Picnic, Dilithium,
      Falcon, SPHINCS+, FRI/STARK-adjacent transcript techniques where relevant

### Prior-Art Matrix

Create a matrix per candidate with:

- [ ] reference name
- [ ] publication or priority date
- [ ] authors/assignee
- [ ] relevant claim or construction
- [ ] overlap with Descender
- [ ] difference from Descender
- [ ] novelty impact
- [ ] filing impact
- [ ] notes for counsel

### Freedom-to-Operate Review

For each planned release:

- [ ] Identify third-party patents likely to be relevant
- [ ] Identify standards-essential patent commitments
- [ ] Identify third-party software licenses
- [ ] Identify third-party proof artifact licenses
- [ ] Identify code generated from external examples
- [ ] Identify dependencies with GPL, AGPL, SSPL, BUSL, or noncommercial terms
- [ ] Identify export-control classification questions
- [ ] Identify trademark naming conflicts

### Decision Gate

Before M3:

- [ ] Mark candidates as `file`, `do not file`, `defer`, or `defensive publish`
- [ ] Confirm which candidates need counsel-drafted provisional applications
- [ ] Confirm which candidates can be released under ordinary repo policy
- [ ] Confirm which public claims should be avoided

## M3: Provisional Filing Packet

Goal: prepare filing-ready technical materials before public release.

### Packet Template

For each patent candidate, prepare:

- [ ] invention title
- [ ] inventor/contributor statement
- [ ] technical field
- [ ] background and problem
- [ ] summary of invention
- [ ] detailed construction
- [ ] protocol diagrams
- [ ] pseudocode
- [ ] parameter examples
- [ ] security rationale
- [ ] performance rationale
- [ ] implementation notes
- [ ] constant-time or side-channel notes, if relevant
- [ ] variants and alternatives
- [ ] deployment examples
- [ ] experimental results
- [ ] known limitations
- [ ] prior-art distinction notes
- [ ] claim-drafting suggestions
- [ ] appendices with code/proof references

### First Filing Candidates

Prioritize candidates by centrality and disclosure risk.

- [ ] Candidate 1: Descender signature construction
- [ ] Candidate 2: public-view/witness compression
- [ ] Candidate 3: committed sigma protocol flow
- [ ] Candidate 4: quotient/link commitment mechanics
- [ ] Candidate 5: leakage certificate methodology
- [ ] Candidate 6: parameter-search/validation pipeline, if novel enough
- [ ] Candidate 7: implementation-hardening profile, if novel enough

### Counsel Review

For each filing candidate:

- [ ] Send invention packet to counsel
- [ ] Hold technical walkthrough
- [ ] Identify missing embodiments
- [ ] Identify overbroad or weak claims
- [ ] Identify needed diagrams
- [ ] Identify enablement gaps
- [ ] Identify foreign filing considerations
- [ ] Approve final filing draft
- [ ] File provisional
- [ ] Record application number
- [ ] Record priority date
- [ ] Record 12-month conversion deadline
- [ ] Record 9-month internal conversion review date

### Post-Filing Actions

After filing:

- [ ] Mark what can now be publicly disclosed
- [ ] Update release notes to say "patent pending" only where accurate
- [ ] Update `PATENTS.md` draft with application status if counsel approves
- [ ] Schedule nonprovisional/PCT decision review
- [ ] Add filing to IP docket

## M4: Public Release Architecture and Repo Policy

Goal: make public release easy to adopt and legally coherent.

### Licensing Files

- [ ] Add root `LICENSE` if absent
- [ ] Choose Apache-2.0 for public implementation code
- [ ] Add `NOTICE`
- [ ] Draft `PATENTS.md`
- [ ] Draft `SECURITY.md`
- [ ] Draft `CONTRIBUTING.md`
- [ ] Decide DCO versus CLA
- [ ] Add contributor sign-off language if using DCO
- [ ] Add CLA process if using CLA
- [ ] Add code provenance policy

### Public Repo Structure

- [ ] Define canonical public package layout
- [ ] Separate reference and optimized implementations
- [ ] Separate spec from implementation
- [ ] Add test-vector directory
- [ ] Add conformance directory
- [ ] Add fuzzing directory
- [ ] Add benchmark directory
- [ ] Add audit directory
- [ ] Add certification directory with public process overview only
- [ ] Add clear security status labels

### Reference Implementation Policy

- [ ] Define supported languages
- [ ] Define constant-time coding rules
- [ ] Define compiler/platform assumptions
- [ ] Define unsafe-code policy
- [ ] Define memory-allocation policy
- [ ] Define random/nonce-generation requirements
- [ ] Define serialization requirements
- [ ] Define parameter-validation requirements
- [ ] Define test-vector coverage
- [ ] Define fuzzing requirements
- [ ] Define CI gates

### Public Claim Language

Add standard wording:

- [ ] "research candidate" wording
- [ ] "reference implementation" wording
- [ ] "designed to avoid secret-dependent branches and memory access" wording
- [ ] "constant-time under stated model only" wording
- [ ] "not certified unless explicitly listed" wording
- [ ] "no warranty/no indemnity in public release" wording
- [ ] vulnerability disclosure wording

## M5: Patent Pledge and Commercial License Drafts

Goal: create the legal architecture that encourages adoption while reserving
paid value.

### Public Patent Pledge

Draft with counsel:

- [ ] grant scope
- [ ] covered patents and applications
- [ ] covered standard profiles
- [ ] rights granted
- [ ] defensive termination
- [ ] downstream distribution language
- [ ] no trademark grant
- [ ] no certification grant
- [ ] no warranty
- [ ] no indemnity
- [ ] no support obligation
- [ ] treatment of future continuation patents
- [ ] treatment of forks and non-conforming variants

### Commercial Patent License

Draft template covering:

- [ ] non-standard variants
- [ ] proprietary profiles
- [ ] sublicensing rights
- [ ] high-liability deployment
- [ ] hardware and embedded use
- [ ] support bundle options
- [ ] warranties, if any
- [ ] indemnity, if any
- [ ] audit rights
- [ ] usage reporting, if any
- [ ] fees
- [ ] renewal
- [ ] termination
- [ ] survival
- [ ] confidentiality
- [ ] export compliance

### Pricing Model

- [ ] Define free community tier
- [ ] Define startup tier
- [ ] Define standard commercial tier
- [ ] Define hardware/HSM tier
- [ ] Define high-liability custom tier
- [ ] Define discount policy for open-source maintainers
- [ ] Define academic/research policy
- [ ] Define government pricing strategy
- [ ] Define support SLA pricing
- [ ] Define indemnity pricing

### License Decision Matrix

Create a one-page matrix:

- [ ] standards-conforming open source: free
- [ ] standards-conforming ordinary commercial software: free patent pledge
- [ ] standards-conforming with certification mark: paid
- [ ] non-standard proprietary variant: paid
- [ ] HSM/smart-card/hardware wallet: paid
- [ ] banking/government/CA deployment: paid/custom
- [ ] indemnity required: paid/custom
- [ ] support SLA required: paid

## M6: Certification and Trademark Program

Goal: create a paid trust layer that does not block implementation.

### Trademark Program

- [ ] Choose marks
- [ ] Run trademark clearance search
- [ ] Decide filing classes
- [ ] File trademark applications where appropriate
- [ ] Draft trademark usage guidelines
- [ ] Define allowed and prohibited uses
- [ ] Define enforcement process

### Certification Program

- [ ] Define certification levels
- [ ] Define product/version scope
- [ ] Define platform scope
- [ ] Define renewal cadence
- [ ] Define revocation rules
- [ ] Define public registry policy
- [ ] Define confidentiality policy for reports
- [ ] Define pricing

### Technical Certification Gates

- [ ] Spec conformance
- [ ] Test-vector pass
- [ ] Negative-test pass
- [ ] Fuzzing threshold
- [ ] Serialization review
- [ ] transcript-domain-separation review
- [ ] nonce/randomness review
- [ ] parameter-validation review
- [ ] constant-time review
- [ ] side-channel review
- [ ] fault-injection review for hardware
- [ ] memory-safety review
- [ ] dependency review
- [ ] build reproducibility review, if relevant
- [ ] integration threat model

### Certification Artifacts

- [ ] certificate template
- [ ] report template
- [ ] public registry format
- [ ] badge/mark files
- [ ] customer agreement
- [ ] renewal notice
- [ ] revocation notice
- [ ] vulnerability handling process for certified products

## M7: Standards and Ecosystem Adoption Package

Goal: make the primitive easy for serious ecosystems to evaluate and adopt.

### Standards Packet

- [ ] concise spec
- [ ] rationale document
- [ ] security proof or proof sketch
- [ ] assumptions list
- [ ] parameter table
- [ ] implementation guidance
- [ ] side-channel guidance
- [ ] test vectors
- [ ] reference implementation
- [ ] optimized implementation
- [ ] conformance suite
- [ ] benchmark report
- [ ] patent pledge summary
- [ ] IPR disclosure statement draft
- [ ] security review status
- [ ] known limitations

### Target Ecosystems

Evaluate and prioritize:

- [ ] IETF/CFRG
- [ ] NIST
- [ ] ISO/IEC
- [ ] FIDO
- [ ] W3C identity groups
- [ ] Linux and open-source crypto libraries
- [ ] blockchain clients
- [ ] hardware wallets
- [ ] HSM vendors
- [ ] CA/browser ecosystem
- [ ] government identity programs

### Adoption Outreach

- [ ] Identify technical champions
- [ ] Identify skeptical reviewers
- [ ] Identify likely implementation partners
- [ ] Identify early commercial pilots
- [ ] Prepare non-confidential technical deck
- [ ] Prepare public FAQ
- [ ] Prepare patent/licensing FAQ
- [ ] Prepare security-status FAQ
- [ ] Prepare comparison to incumbent primitives

## M8: Commercial Packaging and Sales Collateral

Goal: package paid value in a way customers can understand and buy.

### Offerings

- [ ] Descender Community
- [ ] Descender Conformance
- [ ] Descender Certified
- [ ] Descender Enterprise
- [ ] Descender High Assurance

### Collateral

- [ ] one-page commercial overview
- [ ] technical whitepaper
- [ ] security review summary
- [ ] certification program overview
- [ ] patent pledge explainer
- [ ] commercial license explainer
- [ ] FAQ for legal/procurement
- [ ] FAQ for CTO/security teams
- [ ] deployment guide
- [ ] support SLA sheet
- [ ] pricing sheet
- [ ] order form
- [ ] mutual NDA template path

### Customer Segments

Prioritize:

- [ ] blockchain infrastructure
- [ ] hardware wallet vendors
- [ ] HSM vendors
- [ ] identity providers
- [ ] certificate authorities
- [ ] banks and payment systems
- [ ] defense/government integrators
- [ ] cloud KMS vendors
- [ ] secure messaging vendors
- [ ] operating-system crypto libraries

### Sales Process

- [ ] inbound qualification form
- [ ] technical discovery checklist
- [ ] legal/procurement checklist
- [ ] export/compliance checklist
- [ ] security review intake
- [ ] certification quote template
- [ ] commercial license quote template
- [ ] support renewal process

## M9: Governance, Compliance, and Recurring Review

Goal: keep the program reliable as the technical work evolves.

### Docket and Deadline Management

- [ ] Create IP docket
- [ ] Track provisional filing dates
- [ ] Track 9-month conversion review dates
- [ ] Track 12-month conversion deadlines
- [ ] Track PCT deadlines
- [ ] Track national-phase deadlines
- [ ] Track office-action deadlines
- [ ] Track trademark deadlines
- [ ] Track certification renewals
- [ ] Track commercial license renewals

### Release Review Board

Before major public releases, review:

- [ ] patent filing status
- [ ] disclosure risk
- [ ] license files
- [ ] patent pledge consistency
- [ ] security claim wording
- [ ] export-control posture
- [ ] third-party dependency licenses
- [ ] trademark usage
- [ ] certification claims

### Quarterly Review

Every quarter:

- [ ] update invention inventory
- [ ] update prior-art matrix
- [ ] update FTO risk register
- [ ] review patent conversion decisions
- [ ] review public documentation for overclaims
- [ ] review certification pipeline
- [ ] review commercial pricing
- [ ] review standards opportunities
- [ ] review open-source adoption
- [ ] review security incidents or disclosures

### Metrics

Track:

- [ ] number of invention disclosures
- [ ] number of provisional filings
- [ ] number of converted applications
- [ ] number of public implementations
- [ ] number of independent implementations
- [ ] number of conformance-suite users
- [ ] number of standards discussions
- [ ] number of external reviews
- [ ] number of certified products
- [ ] certification revenue
- [ ] support revenue
- [ ] commercial license revenue
- [ ] vulnerability reports
- [ ] time to disclosure response

## First 30 Days

- [ ] Decide whether `legal/` can stay public or should be mirrored privately
- [ ] Pick document owner
- [ ] Select patent counsel
- [ ] Create invention disclosure template
- [ ] Inventory current repo for candidate inventions
- [ ] Build initial candidate invention list
- [ ] Classify disclosure status for each candidate
- [ ] Identify any details that should not be further disclosed publicly
- [ ] Run first rough prior-art search on top three candidates
- [ ] Prepare first provisional packet outline
- [ ] Draft preliminary public release policy
- [ ] Draft preliminary `PATENTS.md` outline
- [ ] Draft preliminary certification levels

## First 60 Days

- [ ] Complete invention disclosure packets for top candidates
- [ ] Complete prior-art matrix for top candidates
- [ ] Hold counsel review for first filing candidates
- [ ] File first provisional application, if justified
- [ ] Decide Apache-2.0/DCO/CLA policy
- [ ] Draft public patent pledge with counsel
- [ ] Draft commercial license term sheet
- [ ] Draft certification mark strategy
- [ ] Prepare standards evaluation packet outline
- [ ] Identify first three ecosystem targets
- [ ] Identify first three commercial pilot targets

## First 90 Days

- [ ] File additional provisionals if justified
- [ ] Publish or prepare public release after filing
- [ ] Finalize `LICENSE`, `NOTICE`, `PATENTS.md`, `SECURITY.md`, and
      `CONTRIBUTING.md`
- [ ] Publish conformance/test-vector package or internal release candidate
- [ ] Commission external review or define review scope
- [ ] Finalize certification program MVP
- [ ] Prepare commercial collateral MVP
- [ ] Begin standards outreach
- [ ] Begin pilot customer outreach
- [ ] Set quarterly review cadence

## Open Questions

- [ ] Which entity will own the IP?
- [ ] Which artifacts are already publicly disclosed through git or external
      sharing?
- [ ] Which primitive is the first patent priority?
- [ ] Are international rights important enough to justify PCT costs?
- [ ] Do we want DCO only, CLA only, or DCO now with CLA for substantial
      contributions?
- [ ] Should test vectors be Apache-2.0 for simplicity or CC0 for maximum
      reuse?
- [ ] What name should be used for certification marks?
- [ ] Which market is the first commercial target: blockchain, HSM, identity,
      banking, CA, or government?
- [ ] What claims are we willing to make publicly about constant-time behavior?
- [ ] What level of indemnity are we willing to sell, if any?

## Working Definitions

Standard profile:

An implementation profile described in a public specification and listed as
covered by the public patent pledge.

Conforming implementation:

An implementation that follows the standard profile, passes required test
vectors, and does not introduce incompatible behavior while claiming to
implement the standard.

Certified implementation:

A specific product/version/platform combination that has passed the project's
certification process and is allowed to use the certification mark.

Commercial license:

A negotiated agreement for rights or services not included in the public patent
pledge, such as proprietary variants, indemnity, certification, support, or
high-liability deployment terms.

High-liability deployment:

Any deployment where failure creates substantial financial, legal, safety,
national-security, or infrastructure risk. Examples include banking, payment
networks, CA infrastructure, government identity, HSMs, secure elements,
hardware wallets, and consensus-critical blockchain validation.

