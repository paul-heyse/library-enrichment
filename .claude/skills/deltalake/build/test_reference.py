"""Semantic regression controls for contract access, routing, replay and invalidation."""

from __future__ import annotations

import hashlib
import json
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import acquire
import inputs

ROOT = Path(__file__).resolve().parents[1]


class ReferenceTests(unittest.TestCase):
    def test_capture_lock_matches_acquisition(self) -> None:
        inputs.verify(ROOT)
        for p in (ROOT / "build/acquired").glob("*/ACQUISITION.json"):
            d = json.loads(p.read_text())
            self.assertEqual(
                hashlib.sha256((p.parent / "Cargo.lock").read_bytes()).hexdigest(), d["lock_sha256"]
            )

    def test_access_and_associated_output(self) -> None:
        records = json.loads(
            (
                ROOT / "content/operations/deltalake_core.operations.load.LoadBuilder.json"
            ).read_text()
        )
        by_name = {r["name"]: r for r in records}
        self.assertEqual(by_name["with_columns"]["access"], "returned_inferred")
        self.assertEqual(by_name["get_custom_execute_handler"]["access"], "internal_trait")
        self.assertIn("RecordBatchStream", by_name["Output"]["signature"])
        self.assertIn("DeltaTableError", by_name["Output"]["signature"])

    def test_constructor_outputs_not_closure_bounds(self) -> None:
        rows = json.loads((ROOT / "content/catalogs/operation-map.json").read_text())
        by_owner = {r["owner"]: r for r in rows}
        self.assertIn("deltalake_core::operations::load_cdf::CdfLoadBuilder", by_owner)
        for name in ["update::UpdateBuilder", "delete::DeleteBuilder"]:
            row = by_owner["deltalake_core::operations::" + name]
            self.assertFalse(any("when_" in v for v in row["constructed_by"]))

    def test_kernel_source_links_are_portable_and_fork_specific(self) -> None:
        records = []
        for directory in ["operations", "modules"]:
            for path in (ROOT / "content" / directory).glob("buoyant_*.json"):
                records.extend(json.loads(path.read_text()))
        self.assertTrue(records)
        for record in records:
            self.assertIn(
                "buoyant-data/delta-kernel-rs/tree/8ba063f8", record["artifact"]["origin"]
            )
            if record["source_url"]:
                self.assertNotIn("/home/", record["source_url"])
                self.assertNotIn("/git/checkouts/", record["source_url"])
                self.assertIn("delta-kernel-rs/blob/8ba063f8", record["source_url"])

    def test_reviewed_claims_and_profile_scope(self) -> None:
        rows = json.loads((ROOT / "content/capabilities/catalog.json").read_text())
        self.assertEqual(len(rows), 18)
        receipt = json.loads(
            (ROOT / "skill_improvement/evidence/implementation/probe-results.json").read_text()
        )
        for row in rows:
            for claim in row["claims"]:
                if "assertion_test" in claim:
                    self.assertIn(claim["assertion_test"], receipt["passed_tests"])
            self.assertTrue(row["effects"] and row["errors"] and row["unknowns"])

    def test_replay_never_generates_lock_and_checks_package_identity(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "source"
            source.mkdir()
            retained = root / "retained"
            retained.mkdir()
            revision = "a" * 40
            lock = (
                'version = 4\n[[package]]\nname = "kernel"\nversion = "1.0.0"\n'
                f'source = "git+https://example.invalid/kernel#{revision}"\n'
            ).encode()
            (retained / "Cargo.lock").write_bytes(lock)
            (retained / "ACQUISITION.json").write_text(
                json.dumps({"lock_sha256": hashlib.sha256(lock).hexdigest()})
            )
            git = {
                "toolchain": "nightly-2026-09-13",
                "lock_deps": {"kernel": revision},
                "rev": "b" * 40,
                "target": "x86_64-unknown-linux-gnu",
            }
            commands = []

            def run(command: list[str], **kwargs: object) -> str:
                commands.append(command)
                return ""

            with patch.object(acquire, "run", side_effect=run):
                acquire.resolve_lock(git, source, root, root / "target", retained / "Cargo.lock")
            self.assertEqual((source / "Cargo.lock").read_bytes(), lock)
            self.assertFalse(any("generate-lockfile" in command for command in commands))
            wrong = dict(git, lock_deps={"missing_kernel": revision})
            with (
                patch.object(acquire, "run", side_effect=run),
                self.assertRaises(acquire.AcquireError),
            ):
                acquire.resolve_lock(wrong, source, root, root / "target", retained / "Cargo.lock")


if __name__ == "__main__":
    unittest.main()
