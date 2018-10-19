The goal of tinytime is to offer a variety of solutions for timing. All of the solutions are very simple and ready for modification or adjustment. All solutions focus on timing precisely. Some solutions may be more accurate than others.

# Usage
You can use the Python (2 or 3) solutions for quick use, or compile the Rust solution. All of the solutions have the same options and syntax.

```
USAGE: tinytime <COMMAND>

PYTHON TINYTIME EXAMPLE:
python tinytime3.py ./examples/fib.py

OR COMPILED:
tinytime python ./examples/fib.py
```

*Tip: Insert the compiled executable into some directory in your PATH variable so you can call it from anywhere.*

`tinytime2.py` is for Python 2.3+, while `tinytime3.py` is for Python 3.4+.

## Whitespace
Tiny time generally ignores whitespace in the command. For example, `tinytime     python    ./examples/fib.py` makes the `COMMAND` option `"python ./examples/fib.py"`, instead of `"    python    ./examples/fib.py"`. In order to keep the whitespace, just wrap it in double quotes: `tinytime "    python    ./examples/fib.py"` makes the `COMMAND` option `"    python    ./examples/fib.py"`.
