import Mathlib.Tactic

namespace Descender

/-!
Transparent public setup interface.

This file does not prove cryptographic security.  It records the public setup
conditions needed to rule out a hidden setup trapdoor at the certificate
boundary: a public seed and parameters determine the public description and the
full transcript, verification is public and deterministic, and every setup
state value is recoverable from the transcript that is published.
-/

structure PublicDescription (Seed Parameters Description : Type*) where
  seed : Seed
  parameters : Parameters
  description : Description

namespace PublicDescription

def SamePublicInputs {Seed Parameters Description : Type*}
    (D E : PublicDescription Seed Parameters Description) : Prop :=
  D.seed = E.seed ∧ D.parameters = E.parameters

end PublicDescription

structure SetupTranscript (Seed Parameters Description Step : Type*) where
  published : PublicDescription Seed Parameters Description
  steps : List Step

namespace SetupTranscript

def seed {Seed Parameters Description Step : Type*}
    (T : SetupTranscript Seed Parameters Description Step) : Seed :=
  T.published.seed

def parameters {Seed Parameters Description Step : Type*}
    (T : SetupTranscript Seed Parameters Description Step) : Parameters :=
  T.published.parameters

def description {Seed Parameters Description Step : Type*}
    (T : SetupTranscript Seed Parameters Description Step) : Description :=
  T.published.description

end SetupTranscript

/--
A transparent setup algorithm.

`generate` and `transcript` are deterministic functions of public data.  The
`verify` function is the public checker for a claimed public description and
setup transcript.  `privateState` names any setup-side state that would be
dangerous if kept out of band, while `privateStateFromTranscript` is the public
projection that recovers the same state from the transcript in a transparent
setup certificate.
-/
structure TransparentSetup
    (Seed Parameters Description Step PrivateState : Type*) where
  generate : Seed -> Parameters -> Description
  transcript : Seed -> Parameters ->
    SetupTranscript Seed Parameters Description Step
  verify :
    PublicDescription Seed Parameters Description ->
      SetupTranscript Seed Parameters Description Step -> Bool
  privateState : Seed -> Parameters -> PrivateState
  privateStateFromTranscript :
    SetupTranscript Seed Parameters Description Step -> PrivateState

namespace TransparentSetup

variable {Seed Parameters Description Step PrivateState : Type*}

def publicDescription
    (T : TransparentSetup Seed Parameters Description Step PrivateState)
    (seed : Seed) (params : Parameters) :
    PublicDescription Seed Parameters Description where
  seed := seed
  parameters := params
  description := T.generate seed params

def Verifies
    (T : TransparentSetup Seed Parameters Description Step PrivateState)
    (D : PublicDescription Seed Parameters Description)
    (tr : SetupTranscript Seed Parameters Description Step) : Prop :=
  T.verify D tr = true

/--
The generated transcript carries exactly the public seed, parameters, and
description that `generate` computes.
-/
def PublicStateEquality
    (T : TransparentSetup Seed Parameters Description Step PrivateState) :
    Prop :=
  ∀ seed params, (T.transcript seed params).published =
    T.publicDescription seed params

/--
The public verifier accepts the transcript generated from each public seed and
parameter choice.
-/
def GeneratedVerifies
    (T : TransparentSetup Seed Parameters Description Step PrivateState) :
    Prop :=
  ∀ seed params,
    T.Verifies (T.publicDescription seed params) (T.transcript seed params)

/--
Every accepting public description/transcript pair is the deterministic pair
obtained by rerunning setup from its public seed and parameters.
-/
def Reproducible
    (T : TransparentSetup Seed Parameters Description Step PrivateState) :
    Prop :=
  ∀ ⦃D : PublicDescription Seed Parameters Description⦄
      ⦃tr : SetupTranscript Seed Parameters Description Step⦄,
    T.Verifies D tr ->
      tr.published = D ∧
        D.description = T.generate D.seed D.parameters ∧
          tr = T.transcript D.seed D.parameters

/--
No setup-side state remains hidden: the state associated with public setup
equals the state reconstructed from the published transcript.
-/
def NoPrivateTrapdoor
    (T : TransparentSetup Seed Parameters Description Step PrivateState) :
    Prop :=
  ∀ seed params,
    T.privateState seed params =
      T.privateStateFromTranscript (T.transcript seed params)

abbrev NoHiddenSetupState
    (T : TransparentSetup Seed Parameters Description Step PrivateState) :
    Prop :=
  T.NoPrivateTrapdoor

def AllDataInTranscript
    (T : TransparentSetup Seed Parameters Description Step PrivateState) :
    Prop :=
  ∀ ⦃D : PublicDescription Seed Parameters Description⦄
      ⦃tr : SetupTranscript Seed Parameters Description Step⦄,
    T.Verifies D tr ->
      T.privateState D.seed D.parameters = T.privateStateFromTranscript tr

def IsTransparent
    (T : TransparentSetup Seed Parameters Description Step PrivateState) :
    Prop :=
  T.PublicStateEquality ∧ T.GeneratedVerifies ∧
    T.Reproducible ∧ T.NoPrivateTrapdoor

theorem verify_eq_of_same_inputs
    (T : TransparentSetup Seed Parameters Description Step PrivateState)
    {D E : PublicDescription Seed Parameters Description}
    {tr u : SetupTranscript Seed Parameters Description Step}
    (hD : D = E) (htr : tr = u) :
    T.verify D tr = T.verify E u := by
  subst E
  subst u
  rfl

end TransparentSetup

/--
Certificate that a concrete setup interface is transparent.  This is intended
to be imported later by the KEM security certificate; by itself it is only a
no-backdoor/public-reproducibility condition.
-/
structure TransparentSetupCertificate
    {Seed Parameters Description Step PrivateState : Type*}
    (T : TransparentSetup Seed Parameters Description Step PrivateState) where
  publicState : T.PublicStateEquality
  generatedVerifies : T.GeneratedVerifies
  reproducible : T.Reproducible
  noPrivateTrapdoor : T.NoPrivateTrapdoor

namespace TransparentSetupCertificate

variable {Seed Parameters Description Step PrivateState : Type*}
variable {T : TransparentSetup Seed Parameters Description Step PrivateState}

theorem isTransparent (C : TransparentSetupCertificate T) :
    T.IsTransparent :=
  ⟨C.publicState, C.generatedVerifies, C.reproducible, C.noPrivateTrapdoor⟩

theorem generated_verifies (C : TransparentSetupCertificate T)
    (seed : Seed) (params : Parameters) :
    T.Verifies (T.publicDescription seed params) (T.transcript seed params) :=
  C.generatedVerifies seed params

theorem generated_public_eq (C : TransparentSetupCertificate T)
    (seed : Seed) (params : Parameters) :
    (T.transcript seed params).published = T.publicDescription seed params :=
  C.publicState seed params

theorem verified_public_eq (C : TransparentSetupCertificate T)
    {D : PublicDescription Seed Parameters Description}
    {tr : SetupTranscript Seed Parameters Description Step}
    (h : T.Verifies D tr) :
    tr.published = D :=
  (C.reproducible h).1

theorem verified_description_eq (C : TransparentSetupCertificate T)
    {D : PublicDescription Seed Parameters Description}
    {tr : SetupTranscript Seed Parameters Description Step}
    (h : T.Verifies D tr) :
    D.description = T.generate D.seed D.parameters :=
  (C.reproducible h).2.1

theorem verified_transcript_eq (C : TransparentSetupCertificate T)
    {D : PublicDescription Seed Parameters Description}
    {tr : SetupTranscript Seed Parameters Description Step}
    (h : T.Verifies D tr) :
    tr = T.transcript D.seed D.parameters :=
  (C.reproducible h).2.2

theorem no_hidden_state (C : TransparentSetupCertificate T)
    (seed : Seed) (params : Parameters) :
    T.privateState seed params =
      T.privateStateFromTranscript (T.transcript seed params) :=
  C.noPrivateTrapdoor seed params

theorem verified_no_hidden_state (C : TransparentSetupCertificate T)
    {D : PublicDescription Seed Parameters Description}
    {tr : SetupTranscript Seed Parameters Description Step}
    (h : T.Verifies D tr) :
    T.privateState D.seed D.parameters = T.privateStateFromTranscript tr := by
  rw [C.verified_transcript_eq h]
  exact C.noPrivateTrapdoor D.seed D.parameters

theorem all_data_in_transcript (C : TransparentSetupCertificate T) :
    T.AllDataInTranscript := by
  intro D tr h
  exact C.verified_no_hidden_state h

end TransparentSetupCertificate

end Descender
