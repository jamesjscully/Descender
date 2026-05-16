import Lake
open Lake DSL

package «descender» where
  -- add package configuration options here

require mathlib from git
  "https://github.com/leanprover-community/mathlib4.git" @ "v4.29.1"

@[default_target]
lean_lib Descender where
  -- add library configuration options here

