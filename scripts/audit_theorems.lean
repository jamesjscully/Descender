/-
Semantic theorem audit for the Descender library.

This complements the Rust source-inspection gates with checks that only the
elaborator can provide.  Run after a successful `lake build`:

    lake env lean --run scripts/audit_theorems.lean

It fails (exit 1) when:
  * any constant defined in a `Descender.*` module depends on an axiom outside
    the allowed set (in particular `sorryAx`, i.e. a `sorry` anywhere in the
    import closure, or a hand-added `axiom` in any imported helper file);
  * a watched gate-endpoint name is missing or is not an actual theorem
    (e.g. it was stubbed as an `axiom`/`opaque`).

It also writes the fully elaborated statements of the watched gate endpoints
to `audit/lean_theorem_statements.txt` so reviewers can diff what the
theorems actually say — including every hypothesis a section `variable`
contributed — rather than trusting marker substrings in the source text.
-/
import Lean
import Descender

open Lean

/-- Axioms a closed Descender proof is allowed to use.  The `ofReduceBool` /
`ofReduceNat` / `trustCompiler` entries are required by `native_decide`; their
use is reported (they extend the trusted base to the compiler) but allowed. -/
def allowedAxioms : NameSet :=
  ([``propext, ``Classical.choice, ``Quot.sound,
    ``Lean.ofReduceBool, ``Lean.ofReduceNat, `Lean.trustCompiler] : List Name)
    |>.foldl (init := {}) (·.insert ·)

/-- `native_decide` generates one compiler-trust axiom per use, named
`<decl>._native.native_decide.ax_*`.  These are allowed but reported: they
extend the trusted base from the kernel to the compiler. -/
def isNativeDecideAxiom (n : Name) : Bool :=
  (n.toString.splitOn "._native.native_decide.ax").length > 1

/-- Compiler-trust axioms, reported separately. -/
def isCompilerTrustAxiom (n : Name) : Bool :=
  n == ``Lean.ofReduceBool || n == ``Lean.ofReduceNat ||
    n == `Lean.trustCompiler || isNativeDecideAxiom n

/-- A constant is a watched gate endpoint when its name is shaped like the
ones the Rust preflight keys on: instantiation endpoints and versioned gate
theorems. -/
def isWatched (n : Name) : Bool :=
  let s := n.toString
  (s.splitOn "instantiation").length > 1 || (s.splitOn "_v1").length > 1

def isDescenderModule (n : Name) : Bool :=
  n.getRoot == `Descender

/-- Axioms used by a single constant (fresh visited set). -/
def axiomsOf (env : Environment) (declName : Name) : Array Name :=
  let (_, st) :=
    ((CollectAxioms.collect declName : CollectAxioms.M Unit).run env).run {}
  st.axioms

def main : IO UInt32 := do
  initSearchPath (← findSysroot)
  let env ← importModules #[{ module := `Descender }] {} (trustLevel := 1024)

  -- Constants declared in Descender.* modules.
  let moduleNames := env.header.moduleNames
  let isLocal (declName : Name) : Bool :=
    match env.getModuleIdxFor? declName with
    | some idx => isDescenderModule (moduleNames[idx.toNat]!)
    | none => false
  let locals := env.constants.fold (init := #[]) fun acc declName _ =>
    if isLocal declName && !declName.isInternal then acc.push declName else acc
  let locals := locals.qsort (·.toString < ·.toString)

  -- Single shared-state axiom sweep over the union dependency closure.
  let (_, axState) :=
    ((locals.forM CollectAxioms.collect : CollectAxioms.M Unit).run env).run {}
  let usedAxioms := axState.axioms

  let mut exitCode : UInt32 := 0
  let isAllowed (a : Name) : Bool := allowedAxioms.contains a || isCompilerTrustAxiom a
  let disallowed := usedAxioms.filter (fun a => !isAllowed a)
  let nativeDecideCount := (usedAxioms.filter isNativeDecideAxiom).size
  if disallowed.isEmpty then
    IO.println s!"OK: {locals.size} Descender constants depend only on allowed axioms."
    IO.println s!"  compiler-trust (native_decide) axiom instances: {nativeDecideCount}"
    for a in usedAxioms do
      if !isNativeDecideAxiom a then
        IO.println s!"  {a}"
  else
    IO.eprintln s!"FAIL: disallowed axioms in the Descender dependency closure: {disallowed}"
    -- Attribute disallowed axioms to (up to 20) offending constants.
    let mut reported := 0
    for declName in locals do
      if reported >= 20 then break
      let bad := (axiomsOf env declName).filter (fun a => !isAllowed a)
      if !bad.isEmpty then
        IO.eprintln s!"  {declName} uses {bad}"
        reported := reported + 1
    exitCode := 1

  -- Watched gate endpoints: must exist as genuine theorems; dump statements.
  let watched := locals.filter isWatched
  let coreCtx : Core.Context := { fileName := "<audit>", fileMap := default }
  let mut lines : Array String := #[]
  let mut missingTheorems : Array Name := #[]
  for declName in watched do
    match env.find? declName with
    | some (.thmInfo info) =>
      let (fmt, _) ← Meta.MetaM.toIO (Meta.ppExpr info.type) coreCtx { env } {} {}
      let compilerTrusted :=
        (axiomsOf env declName).any isCompilerTrustAxiom
      lines := lines.push <|
        s!"theorem {declName} :\n{fmt}\n" ++
        (if compilerTrusted then "-- depends on native_decide (compiler-trust axioms)\n" else "")
    | some _ =>
      -- Definitions/structures with gate-shaped names are listed, not failed;
      -- only stubbed proofs (axiom/opaque where a theorem is expected) fail.
      match env.find? declName with
      | some (.axiomInfo _) | some (.opaqueInfo _) =>
        missingTheorems := missingTheorems.push declName
      | _ =>
        lines := lines.push s!"(non-theorem declaration) {declName}\n"
    | none => missingTheorems := missingTheorems.push declName
  for declName in missingTheorems do
    IO.eprintln s!"FAIL: watched gate endpoint {declName} is an axiom/opaque stub or missing"
  if !missingTheorems.isEmpty then
    exitCode := 1
  IO.FS.writeFile "audit/lean_theorem_statements.txt"
    (String.intercalate "\n" lines.toList)
  IO.println s!"Wrote {watched.size} watched gate-endpoint statements to audit/lean_theorem_statements.txt"
  return exitCode
