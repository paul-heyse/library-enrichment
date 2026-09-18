"""Control for artifact-local rustdoc ID renumbering and full field/member retention."""

from __future__ import annotations

import unittest

from contracts import collect


class IdentityTests(unittest.TestCase):
    def test_rustdoc_id_renumbering(self) -> None:
        generics = {"params": [], "where_predicates": []}
        entries = {
            "1": ("sample", {"module": {"items": ["2"], "is_crate": True}}),
            "2": (
                "Thing",
                {
                    "struct": {
                        "kind": {"plain": {"fields": ["3"]}},
                        "generics": generics,
                        "impls": ["5"],
                    }
                },
            ),
            "3": ("field", {"struct_field": {"primitive": "u32"}}),
            "4": (
                "run",
                {
                    "function": {
                        "sig": {
                            "inputs": [["value", {"primitive": "u32"}]],
                            "output": {"primitive": "u32"},
                            "is_c_variadic": False,
                        },
                        "generics": generics,
                        "has_body": True,
                        "header": {
                            "is_const": False,
                            "is_async": False,
                            "is_unsafe": False,
                            "abi": "Rust",
                        },
                    }
                },
            ),
            "5": (
                None,
                {
                    "impl": {
                        "for": {"resolved_path": {"path": "Thing", "id": "2", "args": None}},
                        "trait": None,
                        "generics": generics,
                        "items": ["4"],
                        "is_synthetic": False,
                        "blanket_impl": None,
                        "is_negative": False,
                    }
                },
            ),
        }
        document = {
            "format_version": 57,
            "index": {
                i: {
                    "id": i,
                    "name": name,
                    "crate_id": 0,
                    "docs": f"Docs for {name}.\n\nSecond paragraph.",
                    "visibility": "public",
                    "inner": inner,
                    "links": {},
                }
                for i, (name, inner) in entries.items()
            },
            "paths": {"1": {"path": ["sample"]}, "2": {"path": ["sample", "Thing"]}},
        }
        first = collect("sample", "1.0.0", document, {"sample::Thing"})
        replacements = {i: f"r{i}" for i in entries}

        def renumber(node: object) -> object:
            if isinstance(node, dict):
                return {replacements.get(k, k): renumber(v) for k, v in node.items()}
            if isinstance(node, list):
                return [renumber(v) for v in node]
            if isinstance(node, str):
                return replacements.get(node, node)
            return node

        other = renumber(document)
        assert isinstance(other, dict)
        second = collect("sample", "1.0.0", other, {"sample::Thing"})
        self.assertEqual([r["id"] for r in first], [r["id"] for r in second])
        self.assertNotEqual(first[0]["artifact"]["id"], second[0]["artifact"]["id"])
        self.assertEqual(
            {r["path"] for r in first},
            {"sample", "sample::Thing", "sample::Thing::field", "sample::Thing::run"},
        )
        for record in first:
            raw = document["index"][record["artifact"]["item_id"]]
            self.assertEqual(record["type_tree"], raw["inner"])
            self.assertIn("Second paragraph.", record["docs"])


if __name__ == "__main__":
    unittest.main()
