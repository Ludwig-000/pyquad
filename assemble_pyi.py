# pyi_generator.py
import re
import json
from pathlib import Path
from typing import Dict, List, Optional, Any, Set

# --- CONFIGURATION ---

# 1. Set the root directory of your Rust project's source code.
SRC_DIR = Path("src") 

# 2. Set the desired output path for the .pyi file.
PYI_FILE = Path("pyquad.pyi")

# 3. Set the file path for storing user-defined type translations.
TYPE_MAP_FILE = Path(".pyi_type_map.json")

# 4. Markers for the automated section in the .pyi file.
#    The script will only ever write between these two lines.
START_MARKER = "# --- START AUTOMATED SECTION ---"
END_MARKER = "# --- END AUTOMATED SECTION ---"

# --- CORE LOGIC ---

# Base translations from Rust to Python types.
# This list can be expanded with more common types.
DEFAULT_TYPE_MAP = {
    "i8": "int", "i16": "int", "i32": "int", "i64": "int", "isize": "int",
    "u8": "int", "u16": "int", "u32": "int", "u64": "int", "usize": "int",
    "f32": "float", "f64": "float",
    "bool": "bool",
    "String": "str",
    "str": "str",
    "PyObject": "Any",
    "PyAny": "Any",
    "None": "None",
    # Add more common pyo3 types if you use them
    "PyErr": "Exception",
    "PyDict": "dict",
    "PyList": "list",
    "PyTuple": "tuple",
    "PyString": "str",
}

# Regex to capture pyo3 macros and their associated items.
# Using the `(?s)` flag to make `.` match newlines.
RE_PYFUNCTION = re.compile(
    r"(?s)"
    r"(?P<docs>(?:///.*?\n\s*)*)"  # Doc comments
    # CORRECTED: Escaped the opening square bracket
    r"#\[pyfunction\s*(?:\((?P<py_args>.*?)\))?\]\s*"  # Macro with optional args
    r"(?:pub\s+)?(?:async\s+)?fn\s+(?P<name>\w+)\s*"  # fn keyword and name
    r"\((?P<args>.*?)\)\s*"  # Function arguments
    r"(?:->\s*(?P<ret>.*?))?\s*\{"  # Optional return type
)

RE_PYCLASS = re.compile(
    r"(?s)"
    r"(?P<docs>(?:///.*?\n\s*)*)"  # Doc comments
    # CORRECTED: Escaped the opening square bracket
    r"#\[pyclass\s*(?:\((?P<py_args>.*?)\))?\]\s*"  # Macro
    r"(?:#[derive\(.*?\)\s*\n]*)*" # Optional derive macros
    r"(?:pub\s+)?struct\s+(?P<name>\w+)"  # struct keyword and name
)

RE_PYMETHODS = re.compile(
    r"(?s)"
    r"#[pymethods]\s*impl\s+(?P<class_name>\w+)\s*\{"
    r"(?P<methods>.*?)\n\s*\}"
)

# Regex to parse methods inside an `impl` block.
RE_PYMETHOD_FN = re.compile(
    r"(?s)"
    # Alt 1: Tagged function. Captures the first tag. Allows other attributes before `fn`.
    r"(?P<docs>(?:^\s*///.*?\n)*)"
    r"\s*#\[(?P<tag>\w+)\s*(?:\((?P<py_args>.*?)\))?\]\s*"
    r"(?:#\[[^\]]*\]\s*)*"  # Consume any other attributes (safer pattern)
    r"(?:pub\s+)?(?:async\s+)?fn\s+(?P<name_with_tag>\w+)"

    # Alt 2: Function with docs, but the first attribute isn't a simple tag.
    r"|(?:(?P<docs_no_tag>(?:^\s*///.*?\n)*)\s*(?:pub\s+)?(?:async\s+)?fn\s+(?P<name_no_tag>\w+))"

    # Alt 3: Function with no docs or tags.
    r"|(?:pub\s+)?(?:async\s+)?fn\s+(?P<name>\w+)\s*"

    # Common trailer for all alternatives
    r"\((?P<args>.*?)\)\s*"
    r"(?:->\s*(?P<ret>.*?))?\s*\{"
)


RE_CLASS_ATTR = re.compile(
    r"^\s*#\[classattr\]\s*\n\s*(?:pub\s+)?(?:const\s+)?(?P<name>\w+)\s*:\s*(?P<type>.*?)\s*="
, re.MULTILINE)

class PyiGenerator:
    """Orchestrates the generation of the .pyi file."""

    def __init__(self):
        self.type_map = DEFAULT_TYPE_MAP.copy()
        self.user_type_map = self._load_user_type_map()
        self.type_map.update(self.user_type_map)
        self.imports = set(["Any"]) # `Any` is a common fallback

    def _load_user_type_map(self) -> Dict[str, str]:
        if TYPE_MAP_FILE.exists():
            print(f"Loading user type map from '{TYPE_MAP_FILE}'...")
            with open(TYPE_MAP_FILE, "r") as f:
                return json.load(f)
        return {}

    def _save_user_type_map(self):
        print(f"💾 Saving updated user type map to '{TYPE_MAP_FILE}'...")
        with open(TYPE_MAP_FILE, "w") as f:
            json.dump(self.user_type_map, f, indent=2, sort_keys=True)

    def translate_type(self, rust_type: str) -> str:
        """Translates a Rust type to a Python type, asking the user if unknown."""
        # Clean up the type string
        rust_type = rust_type.strip().replace('&', '').replace("mut ", "").strip()
        
        # Handle simple cases first
        if rust_type in self.type_map:
            return self.type_map[rust_type]

        # Handle PyResult<T> -> T
        match = re.match(r"PyResult\s*<\s*(.*)\s*>", rust_type, re.IGNORECASE)
        if match:
            inner_type = match.group(1)
            return self.translate_type(inner_type) if inner_type else "None"

        # Handle Option<T> -> Optional[T]
        match = re.match(r"Option\s*<\s*(.*)\s*>", rust_type)
        if match:
            self.imports.add("Optional")
            inner_type = self.translate_type(match.group(1))
            return f"Optional[{inner_type}]"

        # Handle Vec<T> -> list[T]
        match = re.match(r"Vec\s*<\s*(.*)\s*>", rust_type)
        if match:
            self.imports.add("list")
            inner_type = self.translate_type(match.group(1))
            return f"list[{inner_type}]"
            
        # Handle HashMap<K, V> -> dict[K, V]
        match = re.match(r"HashMap\s*<\s*(.*)\s*,\s*(.*)\s*>", rust_type)
        if match:
            self.imports.add("dict")
            key_type = self.translate_type(match.group(1))
            val_type = self.translate_type(match.group(2))
            return f"dict[{key_type}, {val_type}]"

        # If still unknown, ask the user
        print(f"\n❓ Encountered unknown Rust type: '{rust_type}'")
        py_type = input("   Enter the corresponding Python type (e.g., 'str', 'MyCustomClass', 'Any'): ").strip()
        
        if not py_type:
            print("   No input given, defaulting to 'Any'.")
            py_type = "Any"
            
        self.type_map[rust_type] = py_type
        self.user_type_map[rust_type] = py_type
        self._save_user_type_map() # Save immediately
        return py_type

    def _format_docstring(self, doc_text: str, indent: str) -> str:
        if not doc_text:
            return ""
        # Clean up /// and strip whitespace
        lines = [line.strip().removeprefix("///").strip() for line in doc_text.strip().split('\n')]
        # Join with proper indentation
        formatted_lines = '\n'.join([f"{indent}{line}" for line in lines])
        return f'{indent}"""\n{formatted_lines}\n{indent}"""\n'

    def parse_args(self, args_str: str, is_method: bool = False) -> str:
        """Parses Rust function arguments into Python format."""
        if not args_str.strip():
            return ""

        py_args = []
        # Split args carefully, respecting generics like HashMap<K, V>
        # This is a heuristic and might need refinement for very complex cases.
        args = re.split(r',\s*(?![^<]*>)', args_str)

        for i, arg in enumerate(args):
            arg = arg.strip()
            if not arg: continue
            
            # Skip `self` arguments in methods
            if is_method and i == 0 and ('self' in arg):
                continue

            # Check for `pyo3(from_py_with = "...")` or other attributes and remove them
            arg = re.sub(r'#\[.*?\]', '', arg).strip()

            parts = arg.split(':')
            if len(parts) != 2:
                print(f"⚠️  Could not parse argument: '{arg}'. Skipping.")
                continue
            
            name = parts[0].strip().removeprefix('_') # Remove leading underscore for python
            rust_type = parts[1].strip()
            py_type = self.translate_type(rust_type)
            py_args.append(f"{name}: {py_type}")

        return ", ".join(py_args)
    
    def get_py_name(self, name: str, py_args: Optional[str]) -> str:
        """Gets the exposed Python name, checking for `#[pyo3(name = "...")]`."""
        if py_args:
            match = re.search(r'name\s*=\s*"([^"]+)"', py_args)
            if match:
                return match.group(1)
        return name

    def run(self):
        """Main execution method."""
        print("🚀 Starting .pyi generation...")
        
        standalone_funcs = []
        classes = {}

        for rs_file in sorted(SRC_DIR.rglob("*.rs")):
            namm=  f"{rs_file}"
            if (namm == "src\py_abstractions\structs\GLAM\Vec3.rs"):
                continue
            print(f"   - Scanning {rs_file}...")
            content = rs_file.read_text()

            # Find standalone functions
            for match in RE_PYFUNCTION.finditer(content):
                data = match.groupdict()
                name = self.get_py_name(data['name'], data['py_args'])
                args = self.parse_args(data['args'])
                ret = self.translate_type(data.get('ret') or 'None')
                doc = self._format_docstring(data['docs'], "    ")
                standalone_funcs.append(f"def {name}({args}) -> {ret}:\n{doc}    ...\n")

            # Find classes
            for match in RE_PYCLASS.finditer(content):
                data = match.groupdict()
                name = self.get_py_name(data['name'], data['py_args'])
                if name not in classes:
                    classes[name] = {
                        "doc": self._format_docstring(data['docs'], "    "),
                        "methods": [],
                        "attrs": []
                    }
            
            # Find class attributes
            for match in RE_CLASS_ATTR.finditer(content):
                data = match.groupdict()
                # Heuristic: Find which class this belongs to by searching backwards
                # This is imperfect but works for `impl` blocks in the same file.
                preceding_text = content[:match.start()]
                impl_match = list(re.finditer(r"impl\s+(\w+)", preceding_text))
                if impl_match:
                    class_name = impl_match[-1].group(1)
                    if class_name in classes:
                        attr_name = data['name']
                        attr_type = self.translate_type(data['type'])
                        classes[class_name]['attrs'].append(f"    {attr_name}: {attr_type} = ...\n")

            # Find pymethods blocks
            for match in RE_PYMETHODS.finditer(content):
                class_name = match.group('class_name')
                methods_block = match.group('methods')
                
                if class_name not in classes:
                    print(f"⚠️ Found methods for unknown class '{class_name}'. Skipping.")
                    continue
                # --- NEW LINES START ---
                # Pre-process the block to remove non-function items that confuse the regex.
                # This strips out everything from `#[classattr]` to the next semicolon.
                methods_block_cleaned = re.sub(r"(?s)#\[classattr\].*?;", "", methods_block)
                # --- NEW LINES END ---
                for meth_match in RE_PYMETHOD_FN.finditer(methods_block_cleaned):
                    data = meth_match.groupdict()
                    
                    is_new = data['tag'] == 'new'
                    is_staticmethod = data['tag'] == 'staticmethod'
                    py_args_str = data.get('py_args')
                    
                    # Regex has multiple capture groups for name; find the one that matched
                    # --- START MODIFICATION ---
                    name = data.get('name_with_tag') or data.get('name_no_tag') or data.get('name')

                    if not name:
                        # This should no longer happen with the fixed regex, but it's a safe fallback.
                        if is_new:
                            name = "__init__" # #[new] implies a constructor
                        else:
                            print(f"⚠️  Could not determine function name in class '{class_name}'. Skipping.")
                            continue # Could not determine function name
    
                    # Special handling for #[new] which in Rust is often on a function named `new`
                    if is_new:
                        name = "__init__"
                    # --- END MODIFICATION ---

                    doc = self._format_docstring(data.get('docs') or data.get('docs_no_tag') or '', "        ")
                    
                    final_name = self.get_py_name(name, py_args_str)
                    args = self.parse_args(data['args'], is_method=True)
                    ret = "None" if is_new else self.translate_type(data.get('ret') or 'None')
                    
                    method_def = ""
                    if is_staticmethod:
                        method_def += "    @staticmethod\n"
                        method_def += f"    def {final_name}({args}) -> {ret}:\n{doc}        ...\n"
                    else:
                        # `__init__` or regular instance method
                        method_def = f"    def {final_name}(self, {args}) -> {ret}:\n{doc}        ...\n"

                    classes[class_name]['methods'].append(method_def)

        # Assemble the final .pyi content
        output = [f"from typing import {', '.join(sorted(list(self.imports)))}", "\n"]
        
        if standalone_funcs:
            output.append("# --- Standalone Functions ---\n")
            output.extend(sorted(standalone_funcs))
            output.append("\n")

        if classes:
            output.append("# --- Classes ---\n")
            for name, data in sorted(classes.items()):
                output.append(f"class {name}:")
                if not data['doc'] and not data['attrs'] and not data['methods']:
                    output.append("    ...\n\n")
                    continue
                output.append("\n")
                if data['doc']:
                    output.append(data['doc'])
                if data['attrs']:
                    output.extend(sorted(data['attrs']))
                if data['attrs'] and data['methods']:
                     output.append("\n") # Add newline between attrs and methods
                if data['methods']:
                    output.extend(sorted(data['methods'], key=lambda x: "def __init__" not in x))
                output.append("\n")

        self.write_pyi_file("".join(output))

    def write_pyi_file(self, content: str):
        """Writes the generated content to the .pyi file inside the markers."""
        print(f"✍️ Writing to '{PYI_FILE}'...")
        
        # Ensure the parent directory exists
        PYI_FILE.parent.mkdir(parents=True, exist_ok=True)
        
        file_content = ""
        if PYI_FILE.exists():
            file_content = PYI_FILE.read_text()

        start_idx = file_content.find(START_MARKER)
        end_idx = file_content.find(END_MARKER)

        header = ""
        footer = ""

        if start_idx != -1 and end_idx != -1 and start_idx < end_idx:
            header = file_content[:start_idx]
            footer = file_content[end_idx + len(END_MARKER):]
        else:
            header = "# This file is generated by pyi_generator.py\n# You can add custom definitions outside the automated section.\n\n"
        
        final_content = (
            header.strip() +
            f"\n\n{START_MARKER}\n" +
            content.strip() +
            f"\n{END_MARKER}\n" +
            footer.strip() +
            "\n"
        )

        PYI_FILE.write_text(final_content)
        print("Done.")

if __name__ == "__main__":
    generator = PyiGenerator()
    generator.run()