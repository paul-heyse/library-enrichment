"""Fixed, explicitly requested runtime object observation inside the isolated capsule."""

import annotationlib
import contextlib
import importlib
import inspect
import io
import json
import sys
from pathlib import Path
from typing import TypedDict, cast


class Selection(TypedDict):
    module: str
    attributes: list[str]


class Input(TypedDict):
    selection: Selection
    max_output_bytes: int


class RuntimeResult(TypedDict):
    module: str
    selection: list[str]
    outcome: str
    type_name: str | None
    signature: str | None
    docstring: str | None
    attributes: list[str]
    limitations: list[str]


class BoundedText(io.StringIO):
    """Bound captured Python writes while acknowledging the original write length."""

    def __init__(self) -> None:
        super().__init__()
        self.remaining = 4096
        self.truncated = False

    def write(self, value: str) -> int:
        accepted = value[: self.remaining]
        self.remaining -= len(accepted)
        self.truncated |= len(accepted) != len(value)
        super().write(accepted)
        return len(value)


def main() -> None:
    request = cast(Input, json.loads(Path("/capsule/runtime-selection.json").read_text()))
    selection = request["selection"]
    limit = max(1024, min(request["max_output_bytes"], 1_048_576))
    sys.path.insert(0, "/capsule/python")
    output, errors = BoundedText(), BoundedText()
    result: RuntimeResult = {
        "module": selection["module"],
        "selection": selection["attributes"],
        "outcome": "results",
        "type_name": None,
        "signature": None,
        "docstring": None,
        "attributes": [],
        "limitations": [
            "Attribute names are an observed dir() result; dynamic attributes may be omitted."
        ],
    }
    with contextlib.redirect_stdout(output), contextlib.redirect_stderr(errors):
        try:
            value = importlib.import_module(selection["module"])
            for part in selection["attributes"]:
                value = getattr(value, part)
            kind = type(value)
            result["type_name"] = f"{kind.__module__}.{kind.__qualname__}"
            try:
                if not callable(value):
                    raise TypeError("the selected object is not callable")
                result["signature"] = str(
                    inspect.signature(
                        value,
                        follow_wrapped=False,
                        eval_str=False,
                        annotation_format=annotationlib.Format.STRING,
                    )
                )
            except (ValueError, TypeError):
                result["limitations"].append("This object exposes no inspect.signature result.")
            doc = getattr(value, "__doc__", None)
            if isinstance(doc, str):
                result["docstring"] = doc[:32768]
                if len(doc) > 32768:
                    result["outcome"] = "incomplete"
                    result["limitations"].append("The runtime docstring exceeds 32768 characters.")
            attributes = sorted(set(dir(value)))
            result["attributes"] = attributes[:1024]
            if len(attributes) > 1024:
                result["outcome"] = "incomplete"
                result["limitations"].append("Attribute enumeration exceeds 1024 names.")
            signature = result["signature"]
            if signature is not None and len(signature) > 8192:
                result["signature"] = None
                result["outcome"] = "incomplete"
                result["limitations"].append("The runtime signature exceeds 8192 characters.")
        except Exception as error:
            result["outcome"] = "failed"
            result["limitations"].append(f"{type(error).__name__}: {error}"[:2048])
    report = {
        "result": result,
        "stdout": output.getvalue(),
        "stderr": errors.getvalue(),
        "output_truncated": output.truncated or errors.truncated,
    }

    def encoded() -> str:
        return json.dumps(report, ensure_ascii=False, separators=(",", ":"))

    # Budget the complete encoded report, including escaping and multibyte characters.
    # Truncation never fabricates a valid signature or type name from a prefix.
    value = encoded()
    if len(value.encode()) > limit:
        result["outcome"] = "incomplete"
        result["limitations"].append("The complete runtime report exceeded its output byte budget.")
        report["stdout"], report["stderr"], report["output_truncated"] = "", "", True
        value = encoded()
    while len(value.encode()) > limit and result["attributes"]:
        result["attributes"] = result["attributes"][: len(result["attributes"]) // 2]
        value = encoded()
    while len(value.encode()) > limit and result["docstring"]:
        result["docstring"] = result["docstring"][: len(result["docstring"]) // 2]
        value = encoded()
    for field in ("signature", "type_name"):
        if len(value.encode()) > limit:
            if field == "signature":
                result["signature"] = None
            else:
                result["type_name"] = None
            value = encoded()
    if len(value.encode()) > limit:
        result["limitations"] = ["Runtime result exceeded its complete output byte budget."]
        value = encoded()
    # An unusually long requested identity may itself exceed a tiny operator limit. Rust
    # retains an explicit OutputLimit observation in that case; never change the identity.
    sys.stdout.write(value)


if __name__ == "__main__":
    main()
