module Spike_core
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let v_Q: u16 = mk_u16 12289

let v_FIELD_BITS: usize = mk_usize 14

let v_DIGEST_BYTES: usize = mk_usize 32

let v_CHALLENGE_BYTES: usize = mk_usize 32

let v_LINEAR_SIGMA_PROOF_COMMITMENT_BYTES: usize = mk_usize 32

let v_REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS: usize = mk_usize 12

let v_REVIEWED_FQ12_ACCOUNT_CARRIED_SYNDROME_PACKED_BITS: usize =
  v_REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS *! v_FIELD_BITS

let v_REVIEWED_FQ12_ACCOUNT_CARRIED_SYNDROME_SLACK_BITS: usize =
  (v_DIGEST_BYTES *! mk_usize 8 <: usize) -! v_REVIEWED_FQ12_ACCOUNT_CARRIED_SYNDROME_PACKED_BITS

let v_REVIEWED_FQ12_ACCOUNT_CARRIED_ACTION_POLYNOMIAL_COEFFICIENTS_LOW_TO_HIGH: t_Array u16
  (mk_usize 13) =
  let list =
    [
      mk_u16 25; mk_u16 1; mk_u16 0; mk_u16 0; mk_u16 0; mk_u16 0; mk_u16 0; mk_u16 0; mk_u16 0;
      mk_u16 0; mk_u16 0; mk_u16 0; mk_u16 1
    ]
  in
  FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 13);
  Rust_primitives.Hax.array_of_list 13 list

let add_mod (lhs rhs: u16) : u16 =
  cast (((cast (lhs <: u16) <: u32) +! (cast (rhs <: u16) <: u32) <: u32) %!
      (cast (v_Q <: u16) <: u32)
      <:
      u32)
  <:
  u16

let sub_mod (lhs rhs: u16) : u16 =
  cast (Core_models.Num.impl_u32__rem_euclid (((cast (v_Q <: u16) <: u32) +!
            (cast (lhs <: u16) <: u32)
            <:
            u32) -!
          (cast (rhs <: u16) <: u32)
          <:
          u32)
        (cast (v_Q <: u16) <: u32)
      <:
      u32)
  <:
  u16

let read_bits_le (bytes: t_Slice u8) (bit_offset bit_len: usize) : u32 =
  let out:u32 = mk_u32 0 in
  let out:u32 =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      bit_len
      (fun out temp_1_ ->
          let out:u32 = out in
          let _:usize = temp_1_ in
          true)
      out
      (fun out i ->
          let out:u32 = out in
          let i:usize = i in
          let bit:usize = bit_offset +! i in
          let byte:u8 = bytes.[ bit /! mk_usize 8 <: usize ] in
          let value:u8 = (byte >>! (bit %! mk_usize 8 <: usize) <: u8) &. mk_u8 1 in
          let out:u32 = out |. ((cast (value <: u8) <: u32) <<! i <: u32) in
          out)
  in
  out

let any_bits_set_le (bytes: t_Slice u8) (bit_offset bit_len: usize) : bool =
  let found:bool = false in
  let found:bool =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      bit_len
      (fun found temp_1_ ->
          let found:bool = found in
          let _:usize = temp_1_ in
          true)
      found
      (fun found i ->
          let found:bool = found in
          let i:usize = i in
          let bit:usize = bit_offset +! i in
          if
            (((bytes.[ bit /! mk_usize 8 <: usize ] <: u8) >>! (bit %! mk_usize 8 <: usize) <: u8) &.
              mk_u8 1
              <:
              u8) <>.
            mk_u8 0
          then
            let found:bool = true in
            found
          else found)
  in
  found

let poly_trim (poly: Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global)
    : Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global =
  let poly:Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global =
    Rust_primitives.Hax.while_loop (fun poly ->
          let poly:Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global = poly in
          true)
      (fun poly ->
          let poly:Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global = poly in
          ((Alloc.Vec.impl_1__len #u16 #Alloc.Alloc.t_Global poly <: usize) >. mk_usize 0 <: bool) &&
          ((poly.[ (Alloc.Vec.impl_1__len #u16 #Alloc.Alloc.t_Global poly <: usize) -! mk_usize 1
                <:
                usize ]
              <:
              u16) =.
            mk_u16 0
            <:
            bool))
      (fun poly ->
          let poly:Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global = poly in
          Rust_primitives.Hax.Int.from_machine (mk_u32 0) <: Hax_lib.Int.t_Int)
      poly
      (fun poly ->
          let poly:Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global = poly in
          let
          (tmp0: Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global), (out: Core_models.Option.t_Option u16) =
            Alloc.Vec.impl_1__pop #u16 #Alloc.Alloc.t_Global poly
          in
          let poly:Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global = tmp0 in
          let _:Core_models.Option.t_Option u16 = out in
          poly)
  in
  poly

let poly_mul_mod (lhs rhs modulus: t_Slice u16) : Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global =
  if Core_models.Slice.impl__is_empty #u16 lhs || Core_models.Slice.impl__is_empty #u16 rhs
  then Alloc.Vec.impl__new #u16 ()
  else
    let _:Prims.unit =
      if
        ~.(((Core_models.Slice.impl__len #u16 modulus <: usize) >. mk_usize 0 <: bool) &&
          ((modulus.[ (Core_models.Slice.impl__len #u16 modulus <: usize) -! mk_usize 1 <: usize ]
              <:
              u16) =.
            mk_u16 1
            <:
            bool))
      then
        Rust_primitives.Hax.never_to_any (Core_models.Panicking.panic_fmt (Core_models.Fmt.Rt.impl_1__new_const
                  (mk_usize 1)
                  (let list = ["challenge action modulus must be monic"] in
                    FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
                    Rust_primitives.Hax.array_of_list 1 list)
                <:
                Core_models.Fmt.t_Arguments)
            <:
            Rust_primitives.Hax.t_Never)
    in
    let product:Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global =
      Alloc.Vec.from_elem #u16
        (mk_u16 0)
        (((Core_models.Slice.impl__len #u16 lhs <: usize) +!
            (Core_models.Slice.impl__len #u16 rhs <: usize)
            <:
            usize) -!
          mk_usize 1
          <:
          usize)
    in
    let product:Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global =
      Rust_primitives.Hax.Folds.fold_enumerated_slice lhs
        (fun product temp_1_ ->
            let product:Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global = product in
            let _:usize = temp_1_ in
            true)
        product
        (fun product temp_1_ ->
            let product:Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global = product in
            let (lhs_degree: usize), (x: u16) = temp_1_ in
            Rust_primitives.Hax.Folds.fold_enumerated_slice rhs
              (fun product temp_1_ ->
                  let product:Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global = product in
                  let _:usize = temp_1_ in
                  true)
              product
              (fun product temp_1_ ->
                  let product:Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global = product in
                  let (rhs_degree: usize), (y: u16) = temp_1_ in
                  let degree:usize = lhs_degree +! rhs_degree in
                  let product:Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global =
                    Alloc.Slice.impl__to_vec (Rust_primitives.Hax.Monomorphized_update_at.update_at_usize
                          (Alloc.Vec.impl_1__as_slice product <: t_Slice u16)
                          degree
                          (add_mod (product.[ degree ] <: u16)
                              (cast (((cast (x <: u16) <: u32) *! (cast (y <: u16) <: u32) <: u32) %!
                                    (cast (v_Q <: u16) <: u32)
                                    <:
                                    u32)
                                <:
                                u16)
                            <:
                            u16)
                        <:
                        t_Slice u16)
                  in
                  product)
            <:
            Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global)
    in
    Rust_primitives.Hax.while_loop_cf (fun product ->
          let product:Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global = product in
          true)
      (fun product ->
          let product:Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global = product in
          (Alloc.Vec.impl_1__len #u16 #Alloc.Alloc.t_Global product <: usize) >=.
          (Core_models.Slice.impl__len #u16 modulus <: usize)
          <:
          bool)
      (fun product ->
          let product:Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global = product in
          Rust_primitives.Hax.Int.from_machine (mk_u32 0) <: Hax_lib.Int.t_Int)
      product
      (fun product ->
          let product:Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global = product in
          if
            (Alloc.Vec.impl_1__len #u16 #Alloc.Alloc.t_Global product <: usize) =. mk_usize 0
            <:
            bool
          then
            Core_models.Ops.Control_flow.ControlFlow_Break
            ((), product <: (Prims.unit & Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global))
            <:
            Core_models.Ops.Control_flow.t_ControlFlow
              (Prims.unit & Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global)
              (Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global)
          else
            let coefficient:u16 =
              product.[ (Alloc.Vec.impl_1__len #u16 #Alloc.Alloc.t_Global product <: usize) -!
                mk_usize 1
                <:
                usize ]
            in
            let degree_delta:usize =
              (Alloc.Vec.impl_1__len #u16 #Alloc.Alloc.t_Global product <: usize) -!
              (Core_models.Slice.impl__len #u16 modulus <: usize)
            in
            let
            (tmp0: Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global), (out: Core_models.Option.t_Option u16)
            =
              Alloc.Vec.impl_1__pop #u16 #Alloc.Alloc.t_Global product
            in
            let product:Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global = tmp0 in
            let _:Core_models.Option.t_Option u16 = out in
            if coefficient =. mk_u16 0
            then
              let product:Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global = poly_trim product in
              Core_models.Ops.Control_flow.ControlFlow_Continue product
              <:
              Core_models.Ops.Control_flow.t_ControlFlow
                (Prims.unit & Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global)
                (Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global)
            else
              let product:Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global =
                Rust_primitives.Hax.Folds.fold_enumerated_slice (modulus.[ {
                        Core_models.Ops.Range.f_end
                        =
                        (Core_models.Slice.impl__len #u16 modulus <: usize) -! mk_usize 1 <: usize
                      }
                      <:
                      Core_models.Ops.Range.t_RangeTo usize ]
                    <:
                    t_Slice u16)
                  (fun product temp_1_ ->
                      let product:Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global = product in
                      let _:usize = temp_1_ in
                      true)
                  product
                  (fun product temp_1_ ->
                      let product:Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global = product in
                      let (idx: usize), (modulus_coefficient: u16) = temp_1_ in
                      let degree:usize = degree_delta +! idx in
                      let product:Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global =
                        Alloc.Slice.impl__to_vec (Rust_primitives.Hax.Monomorphized_update_at.update_at_usize
                              (Alloc.Vec.impl_1__as_slice product <: t_Slice u16)
                              degree
                              (sub_mod (product.[ degree ] <: u16)
                                  (cast (((cast (coefficient <: u16) <: u32) *!
                                          (cast (modulus_coefficient <: u16) <: u32)
                                          <:
                                          u32) %!
                                        (cast (v_Q <: u16) <: u32)
                                        <:
                                        u32)
                                    <:
                                    u16)
                                <:
                                u16)
                            <:
                            t_Slice u16)
                      in
                      product)
              in
              Core_models.Ops.Control_flow.ControlFlow_Continue (poly_trim product)
              <:
              Core_models.Ops.Control_flow.t_ControlFlow
                (Prims.unit & Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global)
                (Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global))

let fq12_syndrome_normalize (poly: Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global)
    : t_Array u16 (mk_usize 12) =
  let _:Prims.unit =
    Hax_lib.v_assert ((Alloc.Vec.impl_1__len #u16 #Alloc.Alloc.t_Global poly <: usize) <=.
        v_REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS
        <:
        bool)
  in
  let out:t_Array u16 (mk_usize 12) = Rust_primitives.Hax.repeat (mk_u16 0) (mk_usize 12) in
  let out:t_Array u16 (mk_usize 12) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      (Alloc.Vec.impl_1__len #u16 #Alloc.Alloc.t_Global poly <: usize)
      (fun out temp_1_ ->
          let out:t_Array u16 (mk_usize 12) = out in
          let _:usize = temp_1_ in
          true)
      out
      (fun out idx ->
          let out:t_Array u16 (mk_usize 12) = out in
          let idx:usize = idx in
          Rust_primitives.Hax.Monomorphized_update_at.update_at_usize out idx (poly.[ idx ] <: u16)
          <:
          t_Array u16 (mk_usize 12))
  in
  out

let fq12_syndrome_add (lhs rhs: t_Array u16 (mk_usize 12)) : t_Array u16 (mk_usize 12) =
  let out:t_Array u16 (mk_usize 12) = Rust_primitives.Hax.repeat (mk_u16 0) (mk_usize 12) in
  let out:t_Array u16 (mk_usize 12) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      v_REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS
      (fun out temp_1_ ->
          let out:t_Array u16 (mk_usize 12) = out in
          let _:usize = temp_1_ in
          true)
      out
      (fun out idx ->
          let out:t_Array u16 (mk_usize 12) = out in
          let idx:usize = idx in
          Rust_primitives.Hax.Monomorphized_update_at.update_at_usize out
            idx
            (add_mod (lhs.[ idx ] <: u16) (rhs.[ idx ] <: u16) <: u16)
          <:
          t_Array u16 (mk_usize 12))
  in
  out

let fq12_syndrome_mul (lhs rhs: t_Array u16 (mk_usize 12)) : t_Array u16 (mk_usize 12) =
  fq12_syndrome_normalize (poly_mul_mod (lhs <: t_Slice u16)
        (rhs <: t_Slice u16)
        (v_REVIEWED_FQ12_ACCOUNT_CARRIED_ACTION_POLYNOMIAL_COEFFICIENTS_LOW_TO_HIGH <: t_Slice u16)
      <:
      Alloc.Vec.t_Vec u16 Alloc.Alloc.t_Global)

let unpack_reviewed_fq12_account_carried_syndrome_digest (digest: t_Array u8 (mk_usize 32))
    : Core_models.Result.t_Result (t_Array u16 (mk_usize 12)) Alloc.String.t_String =
  if
    any_bits_set_le (digest <: t_Slice u8)
      v_REVIEWED_FQ12_ACCOUNT_CARRIED_SYNDROME_PACKED_BITS
      v_REVIEWED_FQ12_ACCOUNT_CARRIED_SYNDROME_SLACK_BITS
  then
    Core_models.Result.Result_Err
    (Alloc.String.f_to_string #string
        #FStar.Tactics.Typeclasses.solve
        "syndrome digest is not canonical: high bits are set")
    <:
    Core_models.Result.t_Result (t_Array u16 (mk_usize 12)) Alloc.String.t_String
  else
    let out:t_Array u16 (mk_usize 12) = Rust_primitives.Hax.repeat (mk_u16 0) (mk_usize 12) in
    match
      Rust_primitives.Hax.Folds.fold_range_return (mk_usize 0)
        v_REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS
        (fun out temp_1_ ->
            let out:t_Array u16 (mk_usize 12) = out in
            let _:usize = temp_1_ in
            true)
        out
        (fun out idx ->
            let out:t_Array u16 (mk_usize 12) = out in
            let idx:usize = idx in
            let decoded:u32 =
              read_bits_le (digest <: t_Slice u8) (idx *! v_FIELD_BITS <: usize) v_FIELD_BITS
            in
            if decoded >=. (cast (v_Q <: u16) <: u32)
            then
              let args:(usize & u32) = idx, decoded <: (usize & u32) in
              let args:t_Array Core_models.Fmt.Rt.t_Argument (mk_usize 2) =
                let list =
                  [
                    Core_models.Fmt.Rt.impl__new_display #usize args._1;
                    Core_models.Fmt.Rt.impl__new_display #u32 args._2
                  ]
                in
                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 2);
                Rust_primitives.Hax.array_of_list 2 list
              in
              Core_models.Ops.Control_flow.ControlFlow_Break
              (Core_models.Ops.Control_flow.ControlFlow_Break
                (Core_models.Result.Result_Err
                  (Core_models.Hint.must_use #Alloc.String.t_String
                      (Alloc.Fmt.format (Core_models.Fmt.Rt.impl_1__new_v1 (mk_usize 2)
                              (mk_usize 2)
                              (let list = ["syndrome residue "; " is non-canonical: "] in
                                FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 2);
                                Rust_primitives.Hax.array_of_list 2 list)
                              args
                            <:
                            Core_models.Fmt.t_Arguments)
                        <:
                        Alloc.String.t_String))
                  <:
                  Core_models.Result.t_Result (t_Array u16 (mk_usize 12)) Alloc.String.t_String)
                <:
                Core_models.Ops.Control_flow.t_ControlFlow
                  (Core_models.Result.t_Result (t_Array u16 (mk_usize 12)) Alloc.String.t_String)
                  (Prims.unit & t_Array u16 (mk_usize 12)))
              <:
              Core_models.Ops.Control_flow.t_ControlFlow
                (Core_models.Ops.Control_flow.t_ControlFlow
                    (Core_models.Result.t_Result (t_Array u16 (mk_usize 12)) Alloc.String.t_String)
                    (Prims.unit & t_Array u16 (mk_usize 12))) (t_Array u16 (mk_usize 12))
            else
              Core_models.Ops.Control_flow.ControlFlow_Continue
              (Rust_primitives.Hax.Monomorphized_update_at.update_at_usize out
                  idx
                  (cast (decoded <: u32) <: u16))
              <:
              Core_models.Ops.Control_flow.t_ControlFlow
                (Core_models.Ops.Control_flow.t_ControlFlow
                    (Core_models.Result.t_Result (t_Array u16 (mk_usize 12)) Alloc.String.t_String)
                    (Prims.unit & t_Array u16 (mk_usize 12))) (t_Array u16 (mk_usize 12)))
      <:
      Core_models.Ops.Control_flow.t_ControlFlow
        (Core_models.Result.t_Result (t_Array u16 (mk_usize 12)) Alloc.String.t_String)
        (t_Array u16 (mk_usize 12))
    with
    | Core_models.Ops.Control_flow.ControlFlow_Break ret -> ret
    | Core_models.Ops.Control_flow.ControlFlow_Continue out ->
      Core_models.Result.Result_Ok out
      <:
      Core_models.Result.t_Result (t_Array u16 (mk_usize 12)) Alloc.String.t_String

/// Opaque random-oracle stand-in for `reviewed_fq12_challenge_action_vector`.
/// In production this is a SHA-256 rejection sampler; per the spike OBLIGATION
/// it is abstracted (random oracle, not proven algebraic).
let action_oracle (challenge: t_Array u8 (mk_usize 32)) : t_Array u16 (mk_usize 12) =
  let out:t_Array u16 (mk_usize 12) = Rust_primitives.Hax.repeat (mk_u16 0) (mk_usize 12) in
  let out:t_Array u16 (mk_usize 12) =
    Rust_primitives.Hax.Folds.fold_range (mk_usize 0)
      v_REVIEWED_FQ12_ACCOUNT_CARRIED_PUBLIC_ROWS
      (fun out temp_1_ ->
          let out:t_Array u16 (mk_usize 12) = out in
          let _:usize = temp_1_ in
          true)
      out
      (fun out idx ->
          let out:t_Array u16 (mk_usize 12) = out in
          let idx:usize = idx in
          Rust_primitives.Hax.Monomorphized_update_at.update_at_usize out
            idx
            ((cast (challenge.[ idx ] <: u8) <: u16) %! v_Q <: u16)
          <:
          t_Array u16 (mk_usize 12))
  in
  out

/// THE TARGET (verbatim modulo `reviewed_fq12_challenge_action_vector` ->
/// `action_oracle`). RHS of `z = a + action(c)*w` over Fq12.
let reviewed_fq12_account_carried_expected_response_syndrome
      (challenge witness_commitment proof_commitment: t_Array u8 (mk_usize 32))
    : Core_models.Result.t_Result (t_Array u16 (mk_usize 12)) Alloc.String.t_String =
  match
    unpack_reviewed_fq12_account_carried_syndrome_digest witness_commitment
    <:
    Core_models.Result.t_Result (t_Array u16 (mk_usize 12)) Alloc.String.t_String
  with
  | Core_models.Result.Result_Ok witness ->
    (match
        unpack_reviewed_fq12_account_carried_syndrome_digest proof_commitment
        <:
        Core_models.Result.t_Result (t_Array u16 (mk_usize 12)) Alloc.String.t_String
      with
      | Core_models.Result.Result_Ok randomizer ->
        Core_models.Result.Result_Ok
        (fq12_syndrome_add randomizer
            (fq12_syndrome_mul (action_oracle challenge <: t_Array u16 (mk_usize 12)) witness
              <:
              t_Array u16 (mk_usize 12)))
        <:
        Core_models.Result.t_Result (t_Array u16 (mk_usize 12)) Alloc.String.t_String
      | Core_models.Result.Result_Err err ->
        Core_models.Result.Result_Err err
        <:
        Core_models.Result.t_Result (t_Array u16 (mk_usize 12)) Alloc.String.t_String)
  | Core_models.Result.Result_Err err ->
    Core_models.Result.Result_Err err
    <:
    Core_models.Result.t_Result (t_Array u16 (mk_usize 12)) Alloc.String.t_String
