import re
import os
from pathlib import Path
from collections import defaultdict

# --- Configuration ---
RUST_FILE_PATH = Path("./src/py_abstractions/py_structs.rs") # Adjust if your file is elsewhere
OUTPUT_PYI_PATH = Path("./pyquad.pyi")
MACRO_NAME = "make_basic_pyclass" # Assumes this is the primary macro defining classes
MARKER_STRING = "#end of functions" # The marker to find in the existing .pyi file

# --- Rust to Python Type Mapping ---
# --- Rust to Python Type Mapping ---
# Maps Rust primitive types to their Python equivalents for type hinting.
RUST_TO_PYTHON_TYPE_MAP = {
    "f32": "float",
    "f64": "float",

    "i8": "int",
    "i16": "int",
    "i32": "int",
    "i64": "int",
    "i128": "int",
    "isize": "int",

    "u8": "int",
    "u16": "int",
    "u32": "int",
    "u64": "int",
    "u128": "int",
    "usize": "int",

    "String": "str",
    "str": "str",
    "bool": "bool",
    "char": "str",

    # Extended entries for common Rust types and macros:
    "Vec<u8>": "bytes", 
    "Vec": "List",             # Rust Vec<T> -> Python List[T]
    "vec!": "list",            # Macro to create a Vec, equivalent to a Python list
    "Option": "Optional",      # Option<T> -> Optional[T]
    "Result": "Union",         # Result<T, E> -> Union[T, E] (or a custom type for error handling)
    "HashMap": "Dict",         # HashMap<K, V> -> Dict[K, V]
    "BTreeMap": "Dict",        # BTreeMap<K, V> -> Dict[K, V]
    "HashSet": "Set",          # HashSet<T> -> Set[T]
    "BTreeSet": "Set",         # BTreeSet<T> -> Set[T]
    "VecDeque": "Deque",       # VecDeque<T> -> Deque[T] (from collections.deque)
    "LinkedList": "List",      # LinkedList<T> -> Python list (as an approximation)
    "Box": "",                 # Box<T> acts as a smart pointer; its inner type remains T
    "Rc": "",                  # Rc<T> is a reference-counted pointer; use T directly
    "Arc": "",                 # Arc<T> is an atomic reference-counted pointer; use T directly
    "Cell": "",                # Cell<T> provides interior mutability; type remains T
    "RefCell": "",             # RefCell<T> provides interior mutability; type remains T
    "Duration": "timedelta",   # Duration -> datetime.timedelta (from the datetime module)
    "Instant": "datetime",     # Instant -> datetime (approximation)
    "SystemTime": "datetime",  # SystemTime -> datetime (approximation)
    "PathBuf": "Path",         # PathBuf -> pathlib.Path (from the pathlib module)
    "OsString": "str",         # OsString -> str (platform-specific strings typically map to Python str)
    "OsStr": "str",            # OsStr -> str
    "Cow": "str",              # Cow<'_, str> (Clone on Write) -> str (or bytes, depending on usage)
    "Regex": "Pattern",        # Regex -> re.Pattern (from the re module)
}


# Default to Any for unknown types remains handled by the .get() method in map_rust_type_to_python

def map_rust_type_to_python(rust_type: str) -> str:
    """Maps a Rust type string to its Python equivalent."""
    return RUST_TO_PYTHON_TYPE_MAP.get(rust_type, "Any") # Default to Any if unknown

def parse_rust_file(file_path: Path):
    """
    Parses the Rust file to extract class definitions from macros
    and constant definitions from impl blocks.
    (This function remains the same as before)
    """
    if not file_path.exists():
        raise FileNotFoundError(f"Rust file not found: {file_path}")

    content = file_path.read_text()

    macro_regex = re.compile(
        rf"{MACRO_NAME}!\s*\(\s*(\w+)\s*,\s*\{{([^}}]+)\}}\s*\);",
        re.MULTILINE | re.DOTALL
    )
    field_regex = re.compile(r"(\w+)\s*:\s*([\w:]+)")
    impl_start_regex = re.compile(r"^\s*impl\s+(\w+)\s*\{", re.MULTILINE)
    const_regex = re.compile(r"^\s*pub\s+const\s+(\w+)\s*:\s*Self\s*=", re.MULTILINE)

    classes = {}
    constants = defaultdict(list)

    # Pass 1: Parse Macro Invocations
    for match in macro_regex.finditer(content):
        class_name = match.group(1)
        fields_block = match.group(2)
        fields = {}
        for field_match in field_regex.finditer(fields_block):
            fields[field_match.group(1)] = field_match.group(2)
        if class_name not in classes:
             classes[class_name] = fields
        else:
             print(f"Warning: Duplicate class definition found for {class_name}. Using first definition.")

    # Pass 2: Parse Impl Blocks for Constants
    current_impl_class = None
    lines = content.splitlines()
    for line in lines:
        impl_match = impl_start_regex.match(line)
        if impl_match:
            potential_class = impl_match.group(1)
            current_impl_class = potential_class if potential_class in classes else None
        elif line.strip() == "}":
            current_impl_class = None
        elif current_impl_class:
            const_match = const_regex.match(line)
            if const_match:
                constants[current_impl_class].append(const_match.group(1))

    return classes, constants


def generate_pyi_class_definitions(classes, constants):
    """
    Generates only the class definition strings for the .pyi file,
    including necessary imports within this block.
    """
    pyi_lines = [
        "# --- Auto-generated class definitions start ---",
        "from typing import Any, ClassVar", # Imports needed for the generated block
        "",
    ]

    sorted_class_names = sorted(classes.keys())

    for class_name in sorted_class_names:
        fields = classes[class_name]
        pyi_lines.append(f"class {class_name}:")

        # Add ClassVar constants first
        class_constants = sorted(constants.get(class_name, []))
        if class_constants:
            for const_name in class_constants:
                pyi_lines.append(f"    {const_name}: ClassVar['{class_name}']")
            pyi_lines.append("")

        # Add __init__ method
        init_args = [f"{fname}: {map_rust_type_to_python(rtype)}" for fname, rtype in fields.items()]
        init_signature = f"    def __init__(self, {', '.join(init_args)}) -> None: ..."
        pyi_lines.append(init_signature)
        pyi_lines.append("")

        # Add attributes (get/set properties)
        for field_name, rust_type in fields.items():
            py_type = map_rust_type_to_python(rust_type)
            pyi_lines.append(f"    {field_name}: {py_type}")

        # Add __repr__
        pyi_lines.append("")
        pyi_lines.append("    def __repr__(self) -> str: ...")
        pyi_lines.append("\n")

    pyi_lines.append("# --- Auto-generated class definitions end ---")
    return "\n".join(pyi_lines)

def main():
    """Main function to parse, generate, and update the .pyi file."""
    print(f"Parsing Rust file: {RUST_FILE_PATH}")
    try:
        classes, constants = parse_rust_file(RUST_FILE_PATH)
    except FileNotFoundError as e:
        print(f"Error: {e}")
        return
    except Exception as e:
        print(f"An unexpected error occurred during parsing: {e}")
        return

    if not classes:
        print("No classes found using the specified macro. No changes made to .pyi file.")
        return

    print(f"Found {len(classes)} classes: {', '.join(classes.keys())}")
    # ... (optional: print found constants) ...

    print("\nGenerating new .pyi class definitions...")
    new_pyi_section = generate_pyi_class_definitions(classes, constants)

    final_content = ""
    prefix_content = ""

    if OUTPUT_PYI_PATH.exists():
        print(f"Processing existing file: {OUTPUT_PYI_PATH}")
        existing_content = OUTPUT_PYI_PATH.read_text()
        marker_index = existing_content.find(MARKER_STRING)

        if marker_index != -1:
            print(f"Found marker '{MARKER_STRING}'. Preserving content above it.")
            # Find the end of the line containing the marker
            marker_line_end_index = existing_content.find('\n', marker_index)
            if marker_line_end_index == -1: # Marker is on the last line
                 marker_line_end_index = len(existing_content)

            # Keep everything up to and including the marker line
            prefix_content = existing_content[:marker_line_end_index]

            # Ensure there's a newline after the prefix before adding new content
            if not prefix_content.endswith('\n'):
                prefix_content += '\n'

            final_content = prefix_content + '\n' + new_pyi_section

        else:
            print(f"Warning: Marker '{MARKER_STRING}' not found in {OUTPUT_PYI_PATH}.")
            print("Appending new definitions to the end of the file.")
            # Keep all existing content and append
            prefix_content = existing_content
            # Ensure separation before appending
            if prefix_content and not prefix_content.endswith('\n'):
                prefix_content += '\n'

            final_content = prefix_content + '\n# --- Auto-generated content appended below (marker not found) ---\n' + new_pyi_section

    else:
        print(f"File {OUTPUT_PYI_PATH} does not exist. Creating new file.")
        print(f"Please add manual definitions and the '{MARKER_STRING}' marker later if needed.")
        # For a new file, just write the generated content.
        # User should add the marker manually above this content if they add custom functions later.
        final_content = new_pyi_section

    # Write the final result
    print(f"\nWriting final content to: {OUTPUT_PYI_PATH}")
    try:
        OUTPUT_PYI_PATH.parent.mkdir(parents=True, exist_ok=True)
        OUTPUT_PYI_PATH.write_text(final_content)
        print("Successfully updated/created pyquad.pyi")
    except IOError as e:
        print(f"Error writing file {OUTPUT_PYI_PATH}: {e}")
    except Exception as e:
        print(f"An unexpected error occurred during file writing: {e}")


if __name__ == "__main__":
    main()