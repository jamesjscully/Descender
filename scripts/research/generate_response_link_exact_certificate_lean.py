#!/usr/bin/env python3
"""Generate the Lean response-link exact-certificate summary."""

from __future__ import annotations

import hashlib
import json
import sys
from fractions import Fraction
from pathlib import Path
from textwrap import dedent


ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts/research"))
import verify_response_link_bounded_kernel_certificate as certver  # noqa: E402

READINESS_DIR = ROOT / "research/cryptanalysis/signature_readiness_20260516"
CERT_JSON = READINESS_DIR / "response_link_bounded_kernel_exact_enum_n32_q12289_coords_0_3_b96.json"
MANIFEST_JSON = READINESS_DIR / "response_link_bounded_kernel_audit_manifest_n32_q12289_coords_0_3_b96.json"
OUTPUT_LEAN = ROOT / "Descender/Generated/ResponseLinkExactCertificateN32Q12289Coords03B96.lean"


def lean_string(value: str) -> str:
    return json.dumps(value)


def lean_int_list(values: list[int]) -> str:
    return "[" + ", ".join(str(value) for value in values) + "]"


def lean_int_matrix(rows: list[list[int]]) -> str:
    rendered_rows = [lean_int_list(row) for row in rows]
    return "[\n    " + ",\n    ".join(rendered_rows) + "\n  ]"


def lean_nat_list(values: list[int]) -> str:
    return "[" + ", ".join(str(value) for value in values) + "]"


def lean_nat_matrix(rows: list[list[int]]) -> str:
    rendered_rows = [lean_nat_list(row) for row in rows]
    return "[\n    " + ",\n    ".join(rendered_rows) + "\n  ]"


def lean_nat_3tensor(rows: list[list[list[int]]]) -> str:
    rendered_rows = [lean_nat_matrix(row) for row in rows]
    return "[\n    " + ",\n    ".join(rendered_rows) + "\n  ]"


def lean_rat_literal(value: Fraction) -> str:
    return f"(BoundedKernelExactSummary.rat ({value.numerator} : Int) {value.denominator})"


def lean_rat_list(values: list[Fraction]) -> str:
    return "[" + ", ".join(lean_rat_literal(value) for value in values) + "]"


def lean_rat_matrix(rows: list[list[Fraction]]) -> str:
    rendered_rows = [lean_rat_list(row) for row in rows]
    return "[\n    " + ",\n    ".join(rendered_rows) + "\n  ]"


def gram_schmidt_data(
    reduced_basis: list[list[int]],
) -> tuple[list[list[Fraction]], list[Fraction]]:
    dimension = len(reduced_basis)

    def dot(left: list[Fraction], right: list[Fraction]) -> Fraction:
        return sum((a * b for a, b in zip(left, right)), start=Fraction(0))

    basis_star: list[list[Fraction]] = []
    diagonal: list[Fraction] = []
    mu = [[Fraction(0) for _ in range(dimension)] for __ in range(dimension)]
    for row_idx, basis_row in enumerate(reduced_basis):
        rational_row = [Fraction(value) for value in basis_row]
        star_row = rational_row[:]
        for prev in range(row_idx):
            mu[row_idx][prev] = dot(rational_row, basis_star[prev]) / diagonal[prev]
            for col in range(len(basis_row)):
                star_row[col] -= mu[row_idx][prev] * basis_star[prev][col]
        basis_star.append(star_row)
        diagonal.append(dot(star_row, star_row))
    return mu, diagonal


def integer_coordinates(
    basis_rows: list[list[int]], vector: list[int]
) -> list[int]:
    transposed = [
        [basis_rows[row][col] for row in range(len(basis_rows))]
        for col in range(len(vector))
    ]
    solution = certver.solve_rational_square(transposed, vector)
    if solution is None:
        raise SystemExit("basis coordinate solve failed")
    if any(value.denominator != 1 for value in solution):
        raise SystemExit("basis coordinate solve produced non-integral coordinates")
    return [int(value) for value in solution]


def matrix_entry_rejection_witness(row: int, col: int) -> list[int]:
    rejection_limit = 65535 - (65535 % certver.Q)
    counter = 0
    samples: list[int] = []
    while True:
        h = hashlib.sha256()
        h.update(certver.DOMAIN)
        h.update(row.to_bytes(2, "little"))
        h.update(col.to_bytes(2, "little"))
        h.update(counter.to_bytes(4, "little"))
        digest = h.digest()
        for idx in range(0, len(digest), 2):
            value = int.from_bytes(digest[idx : idx + 2], "little")
            samples.append(value)
            if value < rejection_limit:
                return samples
        counter += 1


def matrix_rejection_witness() -> list[list[list[int]]]:
    return [
        [
            matrix_entry_rejection_witness(row, col)
            for col in range(certver.COLS)
        ]
        for row in range(certver.ROWS)
    ]


def render(cert: dict, manifest: dict) -> str:
    cert_rel = CERT_JSON.relative_to(ROOT).as_posix()
    manifest_rel = MANIFEST_JSON.relative_to(ROOT).as_posix()
    cert_sha = hashlib.sha256(CERT_JSON.read_bytes()).hexdigest()
    manifest_sha = hashlib.sha256(MANIFEST_JSON.read_bytes()).hexdigest()
    shortest_record = cert["shortest_vector_in_cover_ball"]
    shortest = shortest_record["coeff_bound"]
    shortest_vector = shortest_record["vector"]
    response_matrix = certver.full_response_matrix()
    response_matrix_witness = matrix_rejection_witness()
    reduced_basis = [[int(value) for value in row] for row in cert["reduced_basis"]]
    canonical_basis, rank, kernel_dimension = certver.canonical_intersection_basis(
        response_matrix, cert["coords"]
    )
    reduced_from_canonical = [
        integer_coordinates(canonical_basis, row) for row in reduced_basis
    ]
    canonical_from_reduced = [
        integer_coordinates(reduced_basis, row) for row in canonical_basis
    ]
    mu, diagonal = gram_schmidt_data(reduced_basis)
    coords = ", ".join(str(coord) for coord in cert["coords"])
    return dedent(
        f"""\
        import Descender.ResponseLinkExactCertificate

        namespace Descender
        namespace ResponseLinkExactCertificate
        namespace Generated
        namespace ResponseLinkExactCertificateN32Q12289Coords03B96

        /-!
        Generated by `scripts/research/generate_response_link_exact_certificate_lean.py`.

        Source certificate: `{cert_rel}`
        Source certificate SHA-256: `{cert_sha}`

        Source audit manifest: `{manifest_rel}`
        Source audit manifest SHA-256: `{manifest_sha}`
        Manifest payload SHA-256: `{manifest["manifest_payload_sha256"]}`
        -/

        def sourceCertificatePath : String := {lean_string(cert_rel)}
        def sourceCertificateSha256 : String := {lean_string(cert_sha)}
        def sourceManifestPath : String := {lean_string(manifest_rel)}
        def sourceManifestSha256 : String := {lean_string(manifest_sha)}
        def manifestPayloadSha256 : String :=
          {lean_string(manifest["manifest_payload_sha256"])}
        def sourceMatrixDomain : String := {lean_string(cert["matrix_domain"])}
        def sourceMatrixDomainBytes : List Nat :=
          {lean_nat_list(list(cert["matrix_domain"].encode()))}
        def sourceMatrixTranscriptSha256 : String :=
          {lean_string(cert["matrix_transcript_sha256"])}
        def sourceCanonicalBasisSha256 : String :=
          {lean_string(cert["canonical_basis_sha256"])}
        def sourceReducedBasisSha256 : String :=
          {lean_string(cert["reduced_basis_sha256"])}
        def sourceCoverBallVectorSha256 : String :=
          {lean_string(cert["cover_ball_vector_sha256"])}

        def shortestVector : List Int :=
          {lean_int_list(shortest_vector)}

        def responseMatrix : List (List Int) :=
          {lean_int_matrix(response_matrix)}

        def responseMatrixRejectionWitness : List (List (List Nat)) :=
          {lean_nat_3tensor(response_matrix_witness)}

        def canonicalBasis : List (List Int) :=
          {lean_int_matrix(canonical_basis)}

        def reducedBasis : List (List Int) :=
          {lean_int_matrix(reduced_basis)}

        def reducedFromCanonical : List (List Int) :=
          {lean_int_matrix(reduced_from_canonical)}

        def canonicalFromReduced : List (List Int) :=
          {lean_int_matrix(canonical_from_reduced)}

        def gramSchmidtMu : List (List Rat) :=
          {lean_rat_matrix(mu)}

        def gramSchmidtDiagonal : List Rat :=
          {lean_rat_list(diagonal)}

        def summary : BoundedKernelExactSummary where
          q := {cert["q"]}
          matrixRows := {manifest["statement"]["matrix_rows"]}
          matrixCols := {manifest["statement"]["matrix_cols"]}
          coords := [{coords}]
          responseDifferenceBound := {cert["response_difference_bound"]}
          shortestCoeffBound := BoundedKernelExactSummary.maxAbsCoeff shortestVector
          minimumCoverCoeffBound := 114
          combinedConstraintRank := {cert["combined_constraint_rank"]}
          combinedKernelDimension := {cert["combined_kernel_dimension"]}
          status := {lean_string(cert["status"])}
          certificateSha256 := sourceCertificateSha256
          manifestPayloadSha256 := manifestPayloadSha256

        theorem shortest_vector_length :
            shortestVector.length = summary.matrixCols := by
          native_decide

        theorem shortest_vector_max_abs_eq_139 :
            BoundedKernelExactSummary.maxAbsCoeff shortestVector = {shortest} := by
          native_decide

        theorem shortest_vector_l1_eq_1112 :
            BoundedKernelExactSummary.l1Coeff shortestVector = {shortest_record["l1_weight"]} := by
          native_decide

        theorem shortest_vector_support_eq_21 :
            BoundedKernelExactSummary.supportCount shortestVector = {shortest_record["support"]} := by
          native_decide

        theorem response_matrix_shape :
            BoundedKernelExactSummary.matrixShape summary.matrixRows summary.matrixCols responseMatrix = true := by
          native_decide

        theorem response_matrix_rejection_witness_uint16 :
            BoundedKernelExactSummary.rejectionWitnessWithinUInt16
              responseMatrixRejectionWitness = true := by
          native_decide

        theorem source_matrix_domain_bytes_uint8 :
            BoundedKernelExactSummary.bytesWithinUInt8
              sourceMatrixDomainBytes = true := by
          native_decide

        theorem response_matrix_rejection_witness_from_sha256 :
            BoundedKernelExactSummary.sha256MatrixSamplesForWitness
              sourceMatrixDomainBytes
              responseMatrixRejectionWitness = responseMatrixRejectionWitness := by
          native_decide

        theorem response_matrix_from_rejection_witness :
            BoundedKernelExactSummary.matrixFromRejectionWitness?
              summary.q
              responseMatrixRejectionWitness = some responseMatrix := by
          native_decide

        theorem certificate_statement_fields_match_sources :
            summary.q = {cert["q"]} ∧
            summary.matrixRows = {manifest["statement"]["matrix_rows"]} ∧
            summary.matrixCols = {manifest["statement"]["matrix_cols"]} ∧
            summary.coords = [{coords}] ∧
            summary.responseDifferenceBound = {cert["response_difference_bound"]} ∧
            summary.combinedConstraintRank = {cert["combined_constraint_rank"]} ∧
            summary.combinedKernelDimension = {cert["combined_kernel_dimension"]} ∧
            summary.certificateSha256 = sourceCertificateSha256 ∧
            summary.manifestPayloadSha256 = manifestPayloadSha256 ∧
            sourceMatrixDomain = {lean_string(cert["matrix_domain"])} ∧
            sourceMatrixTranscriptSha256 = {lean_string(cert["matrix_transcript_sha256"])} ∧
            sourceCanonicalBasisSha256 = {lean_string(cert["canonical_basis_sha256"])} ∧
            sourceReducedBasisSha256 = {lean_string(cert["reduced_basis_sha256"])} ∧
            sourceCoverBallVectorSha256 = {lean_string(cert["cover_ball_vector_sha256"])} := by
          native_decide

        theorem reduced_basis_shape :
            BoundedKernelExactSummary.matrixShape summary.matrixCols summary.matrixCols reducedBasis = true := by
          native_decide

        theorem canonical_basis_shape :
            BoundedKernelExactSummary.matrixShape summary.matrixCols summary.matrixCols canonicalBasis = true := by
          native_decide

        theorem canonical_basis_rows_satisfy_response_link :
            BoundedKernelExactSummary.basisSatisfiesResponseLink
              (Int.ofNat summary.q)
              responseMatrix
              summary.coords
              canonicalBasis = true := by
          native_decide

        theorem reduced_basis_from_canonical :
            BoundedKernelExactSummary.transformBasis reducedFromCanonical canonicalBasis =
              reducedBasis := by
          native_decide

        theorem canonical_basis_from_reduced :
            BoundedKernelExactSummary.transformBasis canonicalFromReduced reducedBasis =
              canonicalBasis := by
          native_decide

        theorem reduced_basis_rows_satisfy_response_link :
            BoundedKernelExactSummary.basisSatisfiesResponseLink
              (Int.ofNat summary.q)
              responseMatrix
              summary.coords
              reducedBasis = true := by
          native_decide

        theorem shortest_vector_satisfies_response_link :
            BoundedKernelExactSummary.rowSatisfiesResponseLink
              (Int.ofNat summary.q)
              responseMatrix
              summary.coords
              shortestVector = true := by
          native_decide

        def enumerationStats : BoundedKernelExactSummary.EnumStats :=
          BoundedKernelExactSummary.exactEnumerationStats
            reducedBasis
            gramSchmidtMu
            gramSchmidtDiagonal
            summary.responseDifferenceBound
            summary.matrixCols

        theorem exact_enumeration_top_coefficient_forced_zero :
            BoundedKernelExactSummary.exactInterval
              0
              (BoundedKernelExactSummary.getRatAt gramSchmidtDiagonal 22)
              (Rat.ofInt (Int.ofNat (23 * summary.responseDifferenceBound * summary.responseDifferenceBound))) =
              (0, 0) := by
          native_decide

        theorem exact_enumeration_second_coefficient_forced_zero :
            BoundedKernelExactSummary.exactInterval
              (BoundedKernelExactSummary.offsetFrom
                gramSchmidtMu
                (List.replicate summary.matrixCols 0)
                21
                22
                summary.matrixCols)
              (BoundedKernelExactSummary.getRatAt gramSchmidtDiagonal 21)
              (Rat.ofInt (Int.ofNat (23 * summary.responseDifferenceBound * summary.responseDifferenceBound))) =
              (0, 0) := by
          native_decide

        theorem exact_enumeration_first_branch_interval :
            BoundedKernelExactSummary.exactInterval
              (BoundedKernelExactSummary.offsetFrom
                gramSchmidtMu
                (List.replicate summary.matrixCols 0)
                20
                21
                summary.matrixCols)
              (BoundedKernelExactSummary.getRatAt gramSchmidtDiagonal 20)
              (Rat.ofInt (Int.ofNat (23 * summary.responseDifferenceBound * summary.responseDifferenceBound))) =
              (-2, 2) := by
          native_decide

        theorem exact_enumeration_stats_verified :
            enumerationStats = {{
              visitedNodes := {cert["visited_nodes"]},
              visitedLeaves := {cert["visited_leaves"]},
              nonzeroInCover := {cert["nonzero_vectors_in_cover_ball"]},
              minCoeffBound := some 114,
              boundedWitnessFound := false
            }} := by
          native_decide

        theorem status_infeasible : summary.statusInfeasible := by
          unfold BoundedKernelExactSummary.statusInfeasible summary
          rfl

        theorem response_bound_eq_96 :
            summary.responseDifferenceBound = 96 := by
          native_decide

        theorem shortest_coeff_bound_eq_139 :
            summary.shortestCoeffBound = 139 := by
          native_decide

        theorem minimum_cover_coeff_bound_eq_114 :
            summary.minimumCoverCoeffBound = 114 := by
          native_decide

        theorem exact_certificate_excludes_bounded_nonzero_kernel :
            summary.excludesBoundedNonzeroKernel := by
          unfold BoundedKernelExactSummary.excludesBoundedNonzeroKernel summary
          native_decide

        theorem no_nonzero_kernel_vector_inside_response_box
            {{coeffBound : Nat}}
            (hle : coeffBound <= summary.responseDifferenceBound)
            (hshort : summary.minimumCoverCoeffBound <= coeffBound) :
            False :=
          summary.no_vector_with_coeff_bound_at_most_response_bound
            exact_certificate_excludes_bounded_nonzero_kernel hle hshort

        end ResponseLinkExactCertificateN32Q12289Coords03B96
        end Generated
        end ResponseLinkExactCertificate
        end Descender
        """
    )


def main() -> None:
    cert = json.loads(CERT_JSON.read_text())
    manifest = json.loads(MANIFEST_JSON.read_text())
    if cert.get("status") != "infeasible":
        raise SystemExit("response-link exact certificate must be infeasible")
    if manifest.get("schema") != "Descender.response_link.bounded_kernel.audit_manifest.v1":
        raise SystemExit("unexpected response-link audit manifest schema")
    if manifest.get("replay", {}).get("status") != "verified":
        raise SystemExit("response-link audit manifest replay must be verified")
    if cert.get("q") != certver.Q:
        raise SystemExit("response-link certificate q mismatch")
    if cert.get("matrix_domain") != certver.DOMAIN.decode():
        raise SystemExit("response-link matrix domain mismatch")
    if cert.get("coords") != manifest.get("statement", {}).get("coords_fixed_mod_q_to_zero"):
        raise SystemExit("response-link coords mismatch between certificate and manifest")
    if cert.get("response_difference_bound") >= cert["shortest_vector_in_cover_ball"]["coeff_bound"]:
        raise SystemExit("shortest coefficient bound must exceed response bound")
    matrix = certver.full_response_matrix()
    if cert.get("matrix_transcript_sha256") != certver.matrix_transcript_sha256(matrix):
        raise SystemExit("response matrix transcript mismatch")
    witness_matrix = [
        [
            [
                next(
                    value % certver.Q
                    for value in matrix_entry_rejection_witness(row, col)
                    if value < 65535 - (65535 % certver.Q)
                )
            ][0]
            for col in range(certver.COLS)
        ]
        for row in range(certver.ROWS)
    ]
    if witness_matrix != matrix:
        raise SystemExit("response matrix rejection witness mismatch")
    reduced_basis = [[int(value) for value in row] for row in cert["reduced_basis"]]
    canonical_basis, rank, kernel_dimension = certver.canonical_intersection_basis(
        matrix, cert["coords"]
    )
    if rank != cert.get("combined_constraint_rank"):
        raise SystemExit("combined constraint rank mismatch")
    if kernel_dimension != cert.get("combined_kernel_dimension"):
        raise SystemExit("combined kernel dimension mismatch")
    if not all(
        certver.row_satisfies_relation(matrix, cert["coords"], row)
        for row in reduced_basis
    ):
        raise SystemExit("reduced basis row fails response-link relation")
    if not all(
        certver.row_satisfies_relation(matrix, cert["coords"], row)
        for row in canonical_basis
    ):
        raise SystemExit("canonical basis row fails response-link relation")
    reduced_from_canonical = [
        integer_coordinates(canonical_basis, row) for row in reduced_basis
    ]
    canonical_from_reduced = [
        integer_coordinates(reduced_basis, row) for row in canonical_basis
    ]
    if [
        certver.row_lattice_membership(canonical_basis, row)
        for row in reduced_basis
    ].count(True) != len(reduced_basis):
        raise SystemExit("reduced basis row is outside canonical lattice")
    if [
        certver.row_lattice_membership(reduced_basis, row)
        for row in canonical_basis
    ].count(True) != len(canonical_basis):
        raise SystemExit("canonical basis row is outside reduced lattice")
    if not certver.row_satisfies_relation(
        matrix, cert["coords"], cert["shortest_vector_in_cover_ball"]["vector"]
    ):
        raise SystemExit("shortest vector fails response-link relation")

    OUTPUT_LEAN.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT_LEAN.write_text(render(cert, manifest), encoding="utf-8")
    print(json.dumps({"lean": str(OUTPUT_LEAN.relative_to(ROOT))}, indent=2))


if __name__ == "__main__":
    main()
